use std::fs;
use std::path::Path;
use std::time::Duration;

pub const FUZZ_TOLERANCE: &str = "1%";

pub fn capture_selector_padded(
    tab: &headless_chrome::Tab,
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
