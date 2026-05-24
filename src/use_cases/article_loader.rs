use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use std::fs;
use std::path::Path;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use crate::domain::article::{Article, FrontMatter};
use crate::presentation::templates::markdown_image::MarkdownImageTemplate;
use crate::presentation::templates::code_block::CodeBlockTemplate;
use askama::Template;

pub fn load_articles(dir: &str) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["InspiredGitHub"];

    let mut articles = Vec::new();
    let paths = fs::read_dir(dir)?;

    for path in paths {
        let path = path?.path();
        
        if !path.is_file() || path.extension().map_or(true, |ext| ext != "md") {
            continue;
        }

        match parse_article(&path, &ps, theme) {
            Ok(article) => articles.push(article),
            Err(e) => tracing::error!("Error parsing article {:?}: {}", path, e),
        }
    }

    articles.sort_by_key(|a| a.date);

    Ok(articles)
}

fn parse_article(
    file_path: &Path,
    ps: &SyntaxSet,
    theme: &Theme,
) -> Result<Article, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;

    if !content.starts_with("---") {
        return Err(format!("Missing frontmatter in {:?}", file_path).into());
    }

    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(format!("Invalid frontmatter format in {:?}", file_path).into());
    }

    let yaml_str = parts[1];
    let markdown_content = parts[2];

    let fm: FrontMatter = serde_yaml::from_str(yaml_str)?;
    let date = chrono::NaiveDate::parse_from_str(fm.date.trim(), "%Y-%m-%d")?;
    let slug = slug::slugify(&fm.title);

    let content_html = render_markdown(markdown_content, ps, theme);

    Ok(Article {
        slug,
        title: fm.title,
        date,
        content_html,
    })
}

fn render_markdown(markdown_content: &str, ps: &SyntaxSet, theme: &Theme) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(markdown_content, options);

    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();

    let mut in_image = false;
    let mut image_dest = String::new();
    let mut image_title = String::new();
    let mut image_alt = String::new();

    let mut new_events = Vec::new();

    for event in parser {
        if in_image {
            match event {
                Event::End(TagEnd::Image) => {
                    in_image = false;
                    let display_caption = if !image_title.is_empty() {
                        &image_title
                    } else if !image_alt.is_empty() {
                        &image_alt
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
            Event::Start(Tag::Image { dest_url, title, .. }) => {
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
                let syntax = ps
                    .find_syntax_by_token(&code_block_lang)
                    .unwrap_or_else(|| ps.find_syntax_plain_text());

                let highlighted =
                    match highlighted_html_for_string(&code_block_content, ps, syntax, theme) {
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
    html_output
}

fn escape_html(s: &str) -> String {
    let mut escaped = String::new();
    for c in s.chars() {
        match c {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(c),
        }
    }
    escaped
}
