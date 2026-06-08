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
    let tab = browser.new_tab()?;

    let golden_dir = Path::new("tests/golden");
    if !golden_dir.exists() {
        fs::create_dir_all(golden_dir)?;
    }

    let port = 3000;
    let base_url = format!("http://localhost:{}", port);
    let mut all_matched = true;

    // 1. Capture Home Page Components
    all_matched &= pages::home::assert_home_group(&tab, &base_url, override_main)?;

    // 2. Capture Thumbnail Page
    all_matched &= pages::thumbnail::assert_thumbnail_group(&tab, &base_url, override_main)?;

    // 3. Capture Welcome Article Components
    all_matched &= pages::article::assert_article_group(&tab, &base_url, override_main)?;

    // 4. Capture 404 Page Components
    all_matched &= pages::not_found::assert_not_found_group(&tab, &base_url, override_main)?;

    if !all_matched {
        eprintln!("Golden test failed: visual mismatches detected.");
        std::process::exit(1);
    }

    println!("All golden images generated and verified successfully.");
    Ok(())
}
