use crate::data::data_source::markdown::markdown_data_source::MarkdownDataSource;
use crate::data::usecase::extract_asset_references_usecase::ExtractAssetReferencesUsecase;
use crate::data::usecase::sanitize_markdown_html_usecase::SanitizeMarkdownHtmlUsecase;
use crate::domain::model::article::Article;
use crate::domain::model::error::AppError;
use crate::domain::repository::article_repository::ArticleRepository;
use crate::infrastructure::markdown::helpers::close_open_fences;
use crate::infrastructure::markdown::markdown_renderer::render_markdown;
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

        let documents = self.data_source.fetch_all()?;
        let mut articles = Vec::new();

        for doc in documents {
            let date = chrono::NaiveDate::parse_from_str(doc.frontmatter.date.trim(), "%Y-%m-%d")?;

            let title = doc
                .frontmatter
                .title
                .or(doc.frontmatter.heading)
                .ok_or_else(|| {
                    AppError::InvalidArticle(format!(
                        "Article in file {} is missing both 'title' and 'heading'",
                        doc.file_name
                    ))
                })?;

            let slug = slug::slugify(&title);

            let (raw_content, toc) = render_markdown(&doc.body, &ps);
            let content = SanitizeMarkdownHtmlUsecase::execute(raw_content);

            let lines: Vec<&str> = doc.body.lines().collect();
            let (preview, has_more_content) = if lines.len() > self.truncate_lines {
                let truncated_md = lines[..self.truncate_lines].join("\n");
                let safe_md = close_open_fences(&truncated_md);
                let (raw_preview, _) = render_markdown(&safe_md, &ps);
                let p = SanitizeMarkdownHtmlUsecase::execute(raw_preview);
                (p, true)
            } else {
                (content.clone(), false)
            };

            let asset_refs = ExtractAssetReferencesUsecase::execute(
                &doc.body,
                doc.frontmatter.thumbnail.as_deref(),
            );

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
                referenced_assets: asset_refs.local_assets,
                referenced_external_origins: asset_refs.external_origins,
            });
        }

        articles.sort_by_key(|b| std::cmp::Reverse(b.date));

        Ok(articles)
    }
}
