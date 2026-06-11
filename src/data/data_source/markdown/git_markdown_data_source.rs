use super::markdown_data_source::MarkdownDataSource;
use crate::data::model::markdown_document::MarkdownDocument;
use crate::data::usecase::get_markdown_files_from_directory_usecase::GetMarkdownFilesFromDirectoryUsecase;
use crate::domain::model::error::AppError;
use std::path::PathBuf;
use tempfile::TempDir;

pub struct GitMarkdownDataSource {
    _temp_dir: TempDir,
    articles_path: String,
}

impl GitMarkdownDataSource {
    pub fn new(
        link: &str,
        folder: &str,
        username: Option<&str>,
        password: Option<&str>,
        _branch: &str,
    ) -> Result<Self, AppError> {
        let temp_dir = tempfile::Builder::new()
            .prefix("corkboard-git-")
            .tempdir()
            .map_err(|e| AppError::GitError(format!("Failed to create temp directory: {}", e)))?;

        let final_link = inject_credentials(link, username, password)?;

        tracing::info!("Cloning repository {} into temporary directory", link);

        let url = gix::url::parse(final_link.as_bytes().into())
            .map_err(|e| AppError::GitError(format!("Failed to parse URL: {}", e)))?;

        let mut prepare_clone = gix::prepare_clone(url, temp_dir.path())
            .map_err(|e| AppError::GitError(format!("Failed to prepare clone: {}", e)))?;

        let (mut prepare_checkout, _) = prepare_clone
            .fetch_then_checkout(
                gix::progress::Discard,
                &std::sync::atomic::AtomicBool::new(false),
            )
            .map_err(|e| AppError::GitError(format!("Failed to fetch: {}", e)))?;

        let (_repo, _): (gix::Repository, _) = prepare_checkout
            .main_worktree(
                gix::progress::Discard,
                &std::sync::atomic::AtomicBool::new(false),
            )
            .map_err(|e| AppError::GitError(format!("Failed to checkout: {}", e)))?;

        let mut articles_path = PathBuf::from(temp_dir.path());
        if !folder.is_empty() {
            articles_path.push(folder);
        }

        Ok(Self {
            _temp_dir: temp_dir,
            articles_path: articles_path.to_string_lossy().to_string(),
        })
    }
}

fn inject_credentials(
    link: &str,
    username: Option<&str>,
    password: Option<&str>,
) -> Result<String, AppError> {
    let mut parsed_url = match url::Url::parse(link) {
        Ok(u) => u,
        Err(_) => return Ok(link.to_string()),
    };

    if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
        if password.is_some() {
            tracing::warn!(
                "Git password was configured but the protocol is '{}'. Credentials will be ignored.",
                parsed_url.scheme()
            );
        }
        return Ok(link.to_string());
    }

    if let Some(p) = password {
        parsed_url
            .set_password(Some(p))
            .map_err(|_| AppError::GitError("Failed to set password in URL".to_string()))?;
        if let Some(u) = username {
            parsed_url
                .set_username(u)
                .map_err(|_| AppError::GitError("Failed to set username in URL".to_string()))?;
        }
    }
    Ok(parsed_url.to_string())
}

impl MarkdownDataSource for GitMarkdownDataSource {
    fn fetch_all(&self) -> Result<Vec<MarkdownDocument>, Box<dyn std::error::Error>> {
        GetMarkdownFilesFromDirectoryUsecase::execute(&self.articles_path)
    }
}
