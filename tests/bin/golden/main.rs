use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Duration;

mod helper;
mod header;
mod sidebar;
mod footer;
mod article_card;
mod blockquote;
mod table;
mod codeblock;
mod image;
mod list;
mod headings;
mod separator;

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
    let mut all_matched = true;
    
    // 1. Capture Home Page Components
    let url_home = format!("http://localhost:{}/?deterministic=true", port);
    println!("Navigating to: {}", url_home);
    tab.navigate_to(&url_home)?;
    
    // Wait for page to load and procedural shaders to render
    std::thread::sleep(Duration::from_millis(2000));

    all_matched &= header::assert_header(&tab, override_main)?;
    all_matched &= sidebar::assert_sidebar(&tab, override_main)?;
    all_matched &= footer::assert_footer(&tab, override_main)?;
    all_matched &= article_card::assert_article_card(&tab, override_main)?;

    // 2. Capture Welcome Article Components
    let url_article = format!("http://localhost:{}/article/welcome-to-corkboard?deterministic=true", port);
    println!("Navigating to: {}", url_article);
    tab.navigate_to(&url_article)?;
    
    std::thread::sleep(Duration::from_millis(2000));

    all_matched &= blockquote::assert_blockquote(&tab, override_main)?;
    all_matched &= table::assert_table(&tab, override_main)?;
    all_matched &= codeblock::assert_codeblock(&tab, override_main)?;
    all_matched &= image::assert_image(&tab, override_main)?;
    all_matched &= list::assert_list(&tab, override_main)?;
    all_matched &= headings::assert_headings(&tab, override_main)?;
    all_matched &= separator::assert_separator(&tab, override_main)?;

    if !all_matched {
        eprintln!("Golden test failed: visual mismatches detected.");
        std::process::exit(1);
    }

    println!("All golden images generated and verified successfully.");
    Ok(())
}
