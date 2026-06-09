use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[path = "../../util/mod.rs"]
pub mod util;

mod helper;
mod pages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let override_main = args.contains(&"--override".to_string());

    println!("Starting golden image generation...");
    if override_main {
        println!("Running in override mode. Will overwrite *_main.png reference goldens directly.");
    }

    let options = LaunchOptionsBuilder::default()
        .headless(true)
        .enable_gpu(true)
        .window_size(Some((1400, 1000)))
        .args(vec![
            OsStr::new("--no-sandbox"),
            OsStr::new("--ignore-gpu-blocklist"),
            OsStr::new("--enable-webgl"),
        ])
        .build()?;

    let browser = Browser::new(options)?;

    let golden_dir = Path::new("tests/golden");
    if !golden_dir.exists() {
        fs::create_dir_all(golden_dir)?;
    }

    let port = 3000;
    let base_url = format!("http://localhost:{}", port);
    let mut all_matched = true;

    // 1. Capture Home Page Components
    let tab_home = browser.new_tab()?;
    all_matched &= pages::home::assert_home_group(&tab_home, &base_url, override_main)?;
    tab_home.close_with_unload()?;

    // 2. Capture Thumbnail Page
    let tab_thumb = browser.new_tab()?;
    all_matched &= pages::thumbnail::assert_thumbnail_group(&tab_thumb, &base_url, override_main)?;
    tab_thumb.close_with_unload()?;

    // 3. Capture Welcome Article Components
    let tab_article = browser.new_tab()?;
    all_matched &= pages::article::assert_article_group(&tab_article, &base_url, override_main)?;
    tab_article.close_with_unload()?;

    // 4. Capture 404 Page Components
    let tab_404 = browser.new_tab()?;
    all_matched &= pages::not_found::assert_not_found_group(&tab_404, &base_url, override_main)?;
    tab_404.close_with_unload()?;

    if !all_matched {
        eprintln!("Golden test failed: visual mismatches detected.");
        std::process::exit(1);
    }

    println!("All golden images generated and verified successfully.");
    Ok(())
}
