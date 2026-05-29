use crate::presentation::state::AppState;
use crate::presentation::templates::thumbnail::ThumbnailSourceTemplate;
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, header};
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SourceParams {
    pub title: Option<String>,
}

pub async fn thumbnail_source_handler(
    State(state): State<AppState>,
    Query(params): Query<SourceParams>,
) -> impl IntoResponse {
    let current_title = params
        .title
        .unwrap_or_else(|| state.settings.blog_title.clone());

    ThumbnailSourceTemplate { current_title }
}

pub async fn thumbnails_handler(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> impl IntoResponse {
    // Validate filename to prevent path traversal
    if !is_safe_filename(&filename) {
        tracing::warn!(
            "Blocked attempt to access invalid thumbnail path: {}",
            filename
        );
        return (StatusCode::BAD_REQUEST, "Invalid filename").into_response();
    }

    let cache_path = std::path::Path::new("templates/thumbnails").join(&filename);

    // 1. Serve from cache if it exists
    if cache_path.exists() {
        match std::fs::read(&cache_path) {
            Ok(bytes) => {
                return (StatusCode::OK, [(header::CONTENT_TYPE, "image/png")], bytes)
                    .into_response();
            }
            Err(e) => {
                tracing::error!("Failed to read cached thumbnail {:?}: {}", cache_path, e);
            }
        }
    }

    // 2. Cache miss: Determine slug and look up title
    let stem = &filename[..filename.len() - 4];
    let blog_title_slug = slug::slugify(&state.settings.blog_title);
    let title = if stem == "blog" || stem == blog_title_slug {
        state.settings.blog_title.clone()
    } else {
        match state.articles.iter().find(|a| a.slug == stem) {
            Some(article) => article.title.clone(),
            None => {
                return (StatusCode::NOT_FOUND, "Article not found").into_response();
            }
        }
    };

    // 3. Render programmatically using Headless Chrome in blocking thread
    let port = state.settings.port;
    let title_clone = title.clone();
    let cache_path_clone = cache_path.clone();

    let result = tokio::task::spawn_blocking(move || capture_screenshot(port, title_clone)).await;

    match result {
        Ok(Ok(png_bytes)) => {
            // Write to cache
            if let Err(e) = std::fs::write(&cache_path_clone, &png_bytes) {
                tracing::error!(
                    "Failed to cache generated thumbnail {:?}: {}",
                    cache_path_clone,
                    e
                );
            } else {
                tracing::info!(
                    "Successfully generated and cached thumbnail: {:?}",
                    cache_path_clone
                );
            }

            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "image/png")],
                png_bytes,
            )
                .into_response()
        }
        Ok(Err(err_msg)) => {
            tracing::error!("Error capturing screenshot: {}", err_msg);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate thumbnail: {}", err_msg),
            )
                .into_response()
        }
        Err(join_err) => {
            tracing::error!("Blocking task join error: {:?}", join_err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to run screenshot background task".to_string(),
            )
                .into_response()
        }
    }
}

fn capture_screenshot(port: u16, title: String) -> Result<Vec<u8>, String> {
    use headless_chrome::{Browser, LaunchOptionsBuilder};
    use std::time::Duration;
    use std::ffi::OsStr;

    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .enable_gpu(true)
        .window_size(Some((1400, 830)))
        .args(vec![
            OsStr::new("--no-sandbox"),
            OsStr::new("--ignore-gpu-blocklist"),
            OsStr::new("--enable-webgl"),
        ])
        .build()
        .map_err(|e| format!("Failed to build launch options: {:?}", e))?;

    let browser =
        Browser::new(options).map_err(|e| format!("Failed to launch browser: {:?}", e))?;

    let tab = browser
        .new_tab()
        .map_err(|e| format!("Failed to open tab: {:?}", e))?;

    // Force the viewport to exactly 1200x630 via CDP
    tab.call_method(headless_chrome::protocol::cdp::Emulation::SetDeviceMetricsOverride {
        width: 1200,
        height: 630,
        device_scale_factor: 1.0,
        mobile: false,
        scale: None,
        screen_width: None,
        screen_height: None,
        position_x: None,
        position_y: None,
        dont_set_visible_size: None,
        screen_orientation: None,
        viewport: None,
        display_feature: None,
        device_posture: None,
    })
    .map_err(|e| format!("Failed to set device metrics: {:?}", e))?;

    let encoded_title = percent_encode(&title);
    let url = format!(
        "http://127.0.0.1:{}/thumbnail-source?title={}",
        port, encoded_title
    );

    tracing::debug!("Headless browser navigating to: {}", url);
    tab.navigate_to(&url)
        .map_err(|e| format!("Failed to navigate: {:?}", e))?;

    tab.wait_until_navigated()
        .map_err(|e| format!("Navigation timed out: {:?}", e))?;

    // Wait for the WebGL paper shader to load and paint
    std::thread::sleep(Duration::from_millis(750));

    let png_bytes = tab
        .capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
            None,
            Some(headless_chrome::protocol::cdp::Page::Viewport {
                x: 0.0,
                y: 0.0,
                width: 1200.0,
                height: 630.0,
                scale: 1.0,
            }),
            true,
        )
        .map_err(|e| format!("Failed to capture screenshot: {:?}", e))?;

    Ok(png_bytes)
}

fn percent_encode(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
            result.push(c);
        } else {
            let mut buf = [0; 4];
            for &b in c.encode_utf8(&mut buf).as_bytes() {
                result.push_str(&format!("%{:02X}", b));
            }
        }
    }
    result
}

fn is_safe_filename(filename: &str) -> bool {
    if !filename.ends_with(".png") {
        return false;
    }
    let stem = &filename[..filename.len() - 4];
    if stem.is_empty() {
        return false;
    }
    stem.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
