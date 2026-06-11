use crate::data::data_source::markdown::markdown_data_source::MarkdownDataSource;
use crate::domain::model::article::Article;
use crate::domain::repository::article_repository::ArticleRepository;
use crate::infrastructure::markdown::helpers::close_open_fences;
use crate::infrastructure::markdown::markdown_renderer::render_markdown;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct MarkdownArticleRepository<DS: MarkdownDataSource> {
    data_source: DS,
    truncate_lines: usize,
}

impl<DS: MarkdownDataSource> MarkdownArticleRepository<DS> {
    pub fn new(data_source: DS, truncate_lines: usize) -> Self {
        Self {
            data_source,
            truncate_lines,
        }
    }
}

impl<DS: MarkdownDataSource> ArticleRepository for MarkdownArticleRepository<DS> {
    fn load_all(&self) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let theme = &ts.themes["InspiredGitHub"];

        let documents = self.data_source.fetch_all()?;
        let mut articles = Vec::new();

        for doc in documents {
            let date = chrono::NaiveDate::parse_from_str(doc.frontmatter.date.trim(), "%Y-%m-%d")?;

            let title = match doc.frontmatter.title.or(doc.frontmatter.heading) {
                Some(t) => t,
                None => {
                    tracing::error!(
                        "Article in file {} is missing both 'title' and 'heading'",
                        doc.file_name
                    );
                    continue;
                }
            };

            let slug = slug::slugify(&title);

            let (content, toc) = render_markdown(&doc.body, &ps, theme);

            let lines: Vec<&str> = doc.body.lines().collect();
            let (preview, has_more_content) = if lines.len() > self.truncate_lines {
                let truncated_md = lines[..self.truncate_lines].join("\n");
                let safe_md = close_open_fences(&truncated_md);
                let (p, _) = render_markdown(&safe_md, &ps, theme);
                (p, true)
            } else {
                (content.clone(), false)
            };

            let description = doc.frontmatter.description.or(doc.frontmatter.subheading);

            articles.push(Article {
                slug,
                title,
                date,
                content,
                preview,
                has_more_content,
                description,
                thumbnail: doc.frontmatter.thumbnail,
                toc,
            });
        }

        articles.sort_by_key(|b| std::cmp::Reverse(b.date));

        Ok(articles)
    }
}
