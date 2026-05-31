use crate::presentation::state::AppState;
use crate::presentation::templates::index::HeaderView;
use crate::presentation::templates::thumbnail::ThumbnailTemplate;
use axum::extract::State;
use axum::response::IntoResponse;

pub async fn thumbnail_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let current_title = state.settings.blog_title.clone();

    let header = HeaderView {
        blog_title: current_title.clone(),
        blog_author: state.settings.blog_author.clone(),
    };

    ThumbnailTemplate {
        header,
        current_title,
    }
}

pub async fn generate_startup_thumbnail(port: u16, _title: String) {
    let cache_path = std::path::Path::new("templates/thumbnails").join("thumbnail.webp");
    
    let result = tokio::task::spawn_blocking(move || capture_screenshot(port))
        .await
        .map_err(|e| format!("Task error generating startup thumbnail: {}", e))
        .and_then(|r| r.map_err(|e| format!("Failed to capture startup thumbnail: {}", e)));

    match result {
        Ok(image_bytes) => {
            if let Err(e) = std::fs::write(&cache_path, &image_bytes) {
                tracing::error!("Failed to save startup thumbnail {:?}: {}", cache_path, e);
            } else {
                tracing::info!("Successfully generated startup thumbnail: {:?}", cache_path);
            }
        }
        Err(err) => tracing::error!("{}", err),
    }
}

fn capture_screenshot(port: u16) -> Result<Vec<u8>, String> {
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

    let url = format!("http://127.0.0.1:{}/thumbnail", port);

    tracing::debug!("Headless browser navigating to: {}", url);
    tab.navigate_to(&url)
        .map_err(|e| format!("Failed to navigate: {:?}", e))?;

    tab.wait_until_navigated()
        .map_err(|e| format!("Navigation timed out: {:?}", e))?;

    // Wait for the WebGL paper shader to load and paint
    std::thread::sleep(Duration::from_millis(750));

    let image_bytes = tab
        .capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Webp,
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

    Ok(image_bytes)
}
