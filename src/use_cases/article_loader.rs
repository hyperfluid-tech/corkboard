use super::helpers::close_open_fences;
use super::markdown_renderer::render_markdown;
use crate::domain::article::{Article, FrontMatter};
use std::fs;
use std::path::Path;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;

pub fn load_articles(
    dir: &str,
    truncate_lines: usize,
) -> Result<Vec<Article>, Box<dyn std::error::Error>> {
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

        match parse_article(&path, &ps, theme, truncate_lines) {
            Ok(article) => articles.push(article),
            Err(e) => tracing::error!("Error parsing article {:?}: {}", path, e),
        }
    }

    articles.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(articles)
}

fn parse_article(
    file_path: &Path,
    ps: &SyntaxSet,
    theme: &Theme,
    truncate_lines: usize,
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

    let (content, toc) = render_markdown(markdown_content, ps, theme);

    let lines: Vec<&str> = markdown_content.lines().collect();
    let (preview, has_more_content) = if lines.len() > truncate_lines {
        let truncated_md = lines[..truncate_lines].join("\n");
        let safe_md = close_open_fences(&truncated_md);
        let (p, _) = render_markdown(&safe_md, ps, theme);
        (p, true)
    } else {
        (content.clone(), false)
    };

    Ok(Article {
        slug,
        title: fm.title,
        date,
        content,
        preview,
        has_more_content,
        subheading: fm.subheading,
        thumbnail: fm.thumbnail,
        toc,
    })
}
