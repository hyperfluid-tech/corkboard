use super::heading_parser::parse_and_rewrite_headings;
use super::helpers::escape_html;
use crate::domain::toc_entry::TocEntry;
use crate::presentation::templates::code_block::CodeBlockTemplate;
use crate::presentation::templates::markdown_image::MarkdownImageTemplate;
use askama::Template;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::highlighting::Theme;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn render_markdown(
    markdown_content: &str,
    syntax_set: &SyntaxSet,
    theme: &Theme,
) -> (String, Vec<TocEntry>) {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let raw_events: Vec<Event> = Parser::new_ext(markdown_content, options).collect();

    let (raw_events, toc) = parse_and_rewrite_headings(raw_events);

    let mut new_events = Vec::new();
    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();

    let mut in_image = false;
    let mut image_dest = String::new();
    let mut image_title = String::new();
    let mut image_alt = String::new();

    for event in raw_events {
        if in_image {
            match event {
                Event::End(TagEnd::Image) => {
                    in_image = false;
                    let display_caption = if !image_title.is_empty() {
                        &image_title
                    } else {
                        ""
                    };

                    let template = MarkdownImageTemplate {
                        src: &image_dest,
                        alt: &image_alt,
                        caption: display_caption,
                    };

                    let html = template.render().unwrap();
                    new_events.push(Event::Html(html.into()));
                }
                Event::Text(text) => {
                    image_alt.push_str(&text);
                }
                _ => {}
            }
            continue;
        }

        match event {
            Event::Start(Tag::Image {
                dest_url, title, ..
            }) => {
                in_image = true;
                image_dest = dest_url.to_string();
                image_title = title.to_string();
                image_alt.clear();
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_block_content.clear();
                code_block_lang = match kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    _ => String::new(),
                };
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let syntax = syntax_set
                    .find_syntax_by_token(&code_block_lang)
                    .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

                let highlighted = match highlighted_html_for_string(
                    &code_block_content,
                    syntax_set,
                    syntax,
                    theme,
                ) {
                    Ok(html) => html,
                    Err(_) => {
                        format!(
                            "<pre><code>{}</code></pre>",
                            escape_html(&code_block_content)
                        )
                    }
                };
                let template = CodeBlockTemplate {
                    content: &highlighted,
                };
                let html = template.render().unwrap();
                new_events.push(Event::Html(html.into()));
            }
            Event::Start(Tag::Table(aligns)) => {
                new_events.push(Event::Html(
                    "<div class=\"table-scroll-container\" tabindex=\"0\" role=\"region\" aria-label=\"Scrollable table\">".into()
                ));
                new_events.push(Event::Start(Tag::Table(aligns)));
            }
            Event::End(TagEnd::Table) => {
                new_events.push(Event::End(TagEnd::Table));
                new_events.push(Event::Html("</div>".into()));
            }
            Event::Text(text) if in_code_block => code_block_content.push_str(&text),
            Event::Text(text) => new_events.push(Event::Text(text)),
            Event::SoftBreak | Event::HardBreak if in_code_block => code_block_content.push('\n'),
            Event::SoftBreak => new_events.push(Event::SoftBreak),
            Event::HardBreak => new_events.push(Event::HardBreak),
            other if !in_code_block => new_events.push(other),
            _ => {}
        }
    }

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, new_events.into_iter());

    (html_output, toc)
}
