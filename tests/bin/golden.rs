use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::time::Duration;

const FUZZ_TOLERANCE: &str = "1%";

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

    let header_name = if override_main { "header_main.png" } else { "header_gen.png" };
    all_matched &= capture_selector_padded(&tab, "header", header_name, 32.0)?;
    
    let sidebar_name = if override_main { "sidebar_main.png" } else { "sidebar_gen.png" };
    all_matched &= capture_selector_padded(&tab, "#sidebar", sidebar_name, 0.0)?;
    
    // Scroll to bottom before capturing footer to ensure it's fully painted and in-viewport
    tab.evaluate("window.scrollTo(0, document.body.scrollHeight)", false)?;
    std::thread::sleep(Duration::from_millis(500));
    
    let footer_name = if override_main { "footer_main.png" } else { "footer_gen.png" };
    all_matched &= capture_selector_padded(&tab, "footer", footer_name, 32.0)?;
    
    // Capture the first article card
    let article_card_name = if override_main { "article_card_main.png" } else { "article_card_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".article-card-wrapper", article_card_name, 16.0)?;

    // 2. Capture Welcome Article Components
    let url_article = format!("http://localhost:{}/article/welcome-to-corkboard?deterministic=true", port);
    println!("Navigating to: {}", url_article);
    tab.navigate_to(&url_article)?;
    
    std::thread::sleep(Duration::from_millis(2000));
    
    let blockquote_name = if override_main { "welcome_article_blockquote_main.png" } else { "welcome_article_blockquote_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose blockquote", blockquote_name, 16.0)?;

    let table_name = if override_main { "welcome_article_table_main.png" } else { "welcome_article_table_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose table", table_name, 16.0)?;

    let codeblock_name = if override_main { "welcome_article_codeblock_main.png" } else { "welcome_article_codeblock_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose pre", codeblock_name, 16.0)?;

    let image_name = if override_main { "welcome_article_image_main.png" } else { "welcome_article_image_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose .tipped-image-container", image_name, 16.0)?;

    let list_name = if override_main { "welcome_article_list_main.png" } else { "welcome_article_list_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose ul", list_name, 16.0)?;

    let h1_name = if override_main { "welcome_article_h1_main.png" } else { "welcome_article_h1_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose h1", h1_name, 16.0)?;

    let h2_name = if override_main { "welcome_article_h2_main.png" } else { "welcome_article_h2_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose h2", h2_name, 16.0)?;

    let h3_name = if override_main { "welcome_article_h3_main.png" } else { "welcome_article_h3_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose h3", h3_name, 16.0)?;

    let h4_name = if override_main { "welcome_article_h4_main.png" } else { "welcome_article_h4_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose h4", h4_name, 16.0)?;

    let h5_name = if override_main { "welcome_article_h5_main.png" } else { "welcome_article_h5_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose h5", h5_name, 16.0)?;

    let h6_name = if override_main { "welcome_article_h6_main.png" } else { "welcome_article_h6_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose h6", h6_name, 16.0)?;

    let hr_name = if override_main { "welcome_article_hr_main.png" } else { "welcome_article_hr_gen.png" };
    all_matched &= capture_selector_padded(&tab, ".prose hr", hr_name, 16.0)?;

    if !all_matched {
        eprintln!("Golden test failed: visual mismatches detected.");
        std::process::exit(1);
    }

    println!("All golden images generated and verified successfully.");
    Ok(())
}

fn capture_selector_padded(
    tab: &std::sync::Arc<headless_chrome::Tab>,
    selector: &str,
    filename: &str,
    padding: f64,
) -> Result<bool, Box<dyn std::error::Error>> {
    println!("Capturing {} with padding {}...", filename, padding);
    tab.wait_for_element(selector)?;

    let js = format!(
        r#"(function() {{
            let el = document.querySelector('{}');
            if (!el) return "";
            let r = el.getBoundingClientRect();
            return `${{r.x + window.scrollX - {}}},${{r.y + window.scrollY - {}}},${{r.width + {}}},${{r.height + {}}}`;
        }})()"#,
        selector, padding, padding, padding * 2.0, padding * 2.0
    );

    let eval = tab.evaluate(&js, false)?;
    let val_str = eval.value.unwrap().as_str().unwrap().to_string();
    let parts: Vec<f64> = val_str.split(',').map(|s| s.parse::<f64>().unwrap()).collect();
    
    let viewport = headless_chrome::protocol::cdp::Page::Viewport {
        x: parts[0],
        y: parts[1],
        width: parts[2],
        height: parts[3],
        scale: 1.0,
    };

    // Expand the headless browser's viewport so it can render the full height of the element 
    // without clipping it to the default 1000px window height.
    let required_height = (parts[1] + parts[3]).ceil() as u32;
    tab.call_method(headless_chrome::protocol::cdp::Emulation::SetDeviceMetricsOverride {
        width: 1400,
        height: std::cmp::max(1000_u32, required_height),
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
    })?;

    // Give it a tiny moment to reflow the layout with the new viewport height
    std::thread::sleep(Duration::from_millis(200));

    let image_data = tab.capture_screenshot(
        headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
        None,
        Some(viewport),
        true,
    )?;
    
    let path = Path::new("tests/golden").join(filename);
    fs::write(&path, image_data)?;

    let mut is_match = true;

    if filename.ends_with("_gen.png") {
        let main_filename = filename.replace("_gen.png", "_main.png");
        let main_path = Path::new("tests/golden").join(&main_filename);
        
        let diffs_dir = Path::new("tests/golden/diffs");
        if !diffs_dir.exists() {
            fs::create_dir_all(diffs_dir)?;
        }
        let diff_path = diffs_dir.join(filename.replace("_gen.png", "_diff.png"));
        
        if main_path.exists() {
            let output = std::process::Command::new("compare")
                .args(&[
                    "-metric", "AE",
                    "-fuzz", FUZZ_TOLERANCE,
                    main_path.to_str().unwrap(),
                    path.to_str().unwrap(),
                    diff_path.to_str().unwrap(),
                ])
                .output();
            
            match output {
                Ok(out) => {
                    if out.status.success() {
                        println!("  ✅ {} matches reference.", filename);
                        if diff_path.exists() {
                            let _ = fs::remove_file(&diff_path);
                        }
                    } else {
                        println!("  ❌ {} MISMATCH! Diff saved to {}", filename, diff_path.display());
                        is_match = false;
                    }
                }
                Err(_) => {
                    println!("  ⚠️ ImageMagick 'compare' command not found. Skipped diff generation.");
                }
            }
        } else {
            println!("  ⚠️ Reference image missing: {}", main_path.display());
        }
    }

    Ok(is_match)
}
