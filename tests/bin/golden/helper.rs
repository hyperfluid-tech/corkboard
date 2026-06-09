use std::fs;
use std::path::Path;
use std::time::Duration;

pub const FUZZ_TOLERANCE: &str = "1%";

pub fn capture_selector_padded(
    tab: &headless_chrome::Tab,
    selector: &str,
    base_name: &str,
    padding: f64,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let names = crate::util::GoldenFileNames::new(base_name, override_main);
    let filename = names.target();
    println!("Capturing {} with padding {}...", filename, padding);
    tab.wait_for_element(selector)?;

    let js = format!(
        r#"(function() {{
            let el = document.querySelector('{}');
            if (!el) return "";
            let r = el.getBoundingClientRect();
            return `${{r.x + window.scrollX - {}}},${{r.y + window.scrollY - {}}},${{r.width + {}}},${{r.height + {}}}`;
        }})()"#,
        selector,
        padding,
        padding,
        padding * 2.0,
        padding * 2.0
    );

    let eval = tab.evaluate(&js, false)?;
    let val_str = eval.value.unwrap().as_str().unwrap().to_string();
    let parts: Vec<f64> = val_str
        .split(',')
        .map(|s| s.parse::<f64>().unwrap())
        .collect();
    let viewport = headless_chrome::protocol::cdp::Page::Viewport {
        x: parts[0],
        y: parts[1],
        width: parts[2],
        height: parts[3],
        scale: 1.0,
    };

    // Expand the headless browser's viewport so it can render the full height of the element
    // without clipping it to the default 1000px window height.
    let element_height = parts[3].ceil() as u32;
    tab.call_method(
        headless_chrome::protocol::cdp::Emulation::SetDeviceMetricsOverride {
            width: 1400,
            height: std::cmp::max(1000_u32, element_height),
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
        },
    )?;

    // Scroll to the element to make sure it's in the viewport, 
    // avoiding massive viewport heights that crash the Linux Chrome GPU process
    let scroll_js = format!("window.scrollTo({}, {});", parts[0], parts[1]);
    tab.evaluate(&scroll_js, false)?;

    // Give it a tiny moment to reflow the layout with the new viewport height
    std::thread::sleep(Duration::from_millis(200));

    let image_data = tab.capture_screenshot(
        headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
        None,
        Some(viewport),
        true,
    )?;

    let path = Path::new("tests/golden").join(&filename);
    fs::write(&path, image_data)?;

    let mut is_match = true;

    if !names.is_override() {
        let main_filename = names.main();
        let main_path = Path::new("tests/golden").join(&main_filename);

        let diffs_dir = Path::new("tests/golden/diffs");
        if !diffs_dir.exists() {
            fs::create_dir_all(diffs_dir)?;
        }
        let diff_path = diffs_dir.join(names.diff());

        if main_path.exists() {
            let output = std::process::Command::new("compare")
                .args(&[
                    "-metric",
                    "AE",
                    "-fuzz",
                    FUZZ_TOLERANCE,
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
                        println!(
                            "  ❌ {} MISMATCH! Diff saved to {}",
                            filename,
                            diff_path.display()
                        );
                        is_match = false;
                    }
                }
                Err(_) => {
                    println!(
                        "  ⚠️ ImageMagick 'compare' command not found. Skipped diff generation."
                    );
                }
            }
        } else {
            println!("  ⚠️ Reference image missing: {}", main_path.display());
        }
    }

    Ok(is_match)
}

pub fn capture_full_page(
    tab: &headless_chrome::Tab,
    base_name: &str,
    width: u32,
    height: u32,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let names = crate::util::GoldenFileNames::new(base_name, override_main);
    let filename = names.target();
    println!("Capturing full page {} ({}x{})...", filename, width, height);

    // Set device metrics to exactly match the desired thumbnail dimensions
    tab.call_method(
        headless_chrome::protocol::cdp::Emulation::SetDeviceMetricsOverride {
            width,
            height,
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
        },
    )?;

    std::thread::sleep(Duration::from_millis(200));

    let viewport = headless_chrome::protocol::cdp::Page::Viewport {
        x: 0.0,
        y: 0.0,
        width: width as f64,
        height: height as f64,
        scale: 1.0,
    };

    let image_data = tab.capture_screenshot(
        headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png,
        None,
        Some(viewport),
        true,
    )?;

    let path = Path::new("tests/golden").join(&filename);
    fs::write(&path, image_data)?;

    let mut is_match = true;

    if !names.is_override() {
        let main_filename = names.main();
        let main_path = Path::new("tests/golden").join(&main_filename);

        let diffs_dir = Path::new("tests/golden/diffs");
        if !diffs_dir.exists() {
            fs::create_dir_all(diffs_dir)?;
        }
        let diff_path = diffs_dir.join(names.diff());

        if main_path.exists() {
            let output = std::process::Command::new("compare")
                .args(&[
                    "-metric",
                    "AE",
                    "-fuzz",
                    FUZZ_TOLERANCE,
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
                        println!(
                            "  ❌ {} MISMATCH! Diff saved to {}",
                            filename,
                            diff_path.display()
                        );
                        is_match = false;
                    }
                }
                Err(_) => {
                    println!(
                        "  ⚠️ ImageMagick 'compare' command not found. Skipped diff generation."
                    );
                }
            }
        } else {
            println!("  ⚠️ Reference image missing: {}", main_path.display());
        }
    }

    // Restore viewport to the default golden test size
    tab.call_method(
        headless_chrome::protocol::cdp::Emulation::SetDeviceMetricsOverride {
            width: 1400,
            height: 1000,
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
        },
    )?;

    Ok(is_match)
}
