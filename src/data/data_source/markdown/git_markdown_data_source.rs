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
        assets_folder: &str,
        username: Option<&str>,
        password: Option<&str>,
        _branch: &str,
    ) -> Result<Self, AppError> {
        unsafe {
            openssl_probe::init_openssl_env_vars();
            if let Ok(cert_file) = std::env::var("SSL_CERT_FILE") {
                if std::env::var("CURL_CA_BUNDLE").is_err() {
                    std::env::set_var("CURL_CA_BUNDLE", &cert_file);
                }
            }
        }

        let temp_dir = tempfile::Builder::new()
            .prefix("corkboard-git-")
            .tempdir()
            .map_err(|e| AppError::GitError(format!("Failed to create temp directory: {}", e)))?;

        let final_link = inject_credentials(link, username, password)?;

        let url = gix::url::parse(final_link.as_bytes().into())
            .map_err(|e| AppError::GitError(format!("Failed to parse URL: {}", e)))?;

        let mut prepare_clone = gix::prepare_clone(url, temp_dir.path())
            .map_err(|e| AppError::GitError(format!("Failed to prepare clone: {}", e)))?;

        let (mut prepare_checkout, _) = prepare_clone
            .fetch_then_checkout(
                gix::progress::Discard,
                &std::sync::atomic::AtomicBool::new(false),
            )
            .map_err(|e| AppError::GitError(format!("Failed to fetch: {:?}", e)))?;

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

        if !assets_folder.is_empty() {
            copy_git_assets(temp_dir.path(), assets_folder)?;
        }

        Ok(Self {
            _temp_dir: temp_dir,
            articles_path: articles_path.to_string_lossy().to_string(),
        })
    }
}

fn copy_git_assets(temp_dir_path: &std::path::Path, assets_folder: &str) -> Result<(), AppError> {
    let src_assets_path = temp_dir_path.join(assets_folder);
    if !src_assets_path.exists() || !src_assets_path.is_dir() {
        return Err(AppError::GitError(format!(
            "Configured git assets_folder '{}' does not exist in the repository.",
            assets_folder
        )));
    }

    let dest_assets_path = std::path::PathBuf::from("assets");
    if !dest_assets_path.exists() {
        std::fs::create_dir_all(&dest_assets_path).map_err(|e| {
            AppError::GitError(format!("Failed to create local assets directory: {}", e))
        })?;
    }

    copy_dir_recursive(&src_assets_path, &dest_assets_path)
        .map_err(|e| AppError::GitError(format!("Failed to copy assets from Git: {}", e)))?;

    Ok(())
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
            return Err(AppError::GitError(format!(
                "Git password was configured but the protocol is '{}'. Credentials cannot be injected.",
                parsed_url.scheme()
            )));
        }
        return Ok(link.to_string());
    }

    let Some(p) = password else {
        return Ok(parsed_url.to_string());
    };

    parsed_url
        .set_password(Some(p))
        .map_err(|_| AppError::GitError("Failed to set password in URL".to_string()))?;

    if let Some(u) = username {
        parsed_url
            .set_username(u)
            .map_err(|_| AppError::GitError("Failed to set username in URL".to_string()))?;
    }

    Ok(parsed_url.to_string())
}

impl MarkdownDataSource for GitMarkdownDataSource {
    fn fetch_all(&self) -> Result<Vec<MarkdownDocument>, Box<dyn std::error::Error>> {
        GetMarkdownFilesFromDirectoryUsecase::execute(&self.articles_path)
    }
}

#[cfg(feature = "git")]
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    if !src.is_dir() {
        return Ok(());
    }

    if !dst.exists() {
        std::fs::create_dir_all(dst)?;
    }

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&path, &dest_path)?;
            continue;
        }
        std::fs::copy(&path, &dest_path)?;
    }
    Ok(())
}
