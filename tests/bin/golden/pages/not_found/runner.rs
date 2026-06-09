use crate::helper::capture_selector_padded;
use headless_chrome::Tab;
pub fn assert_not_found_group(
    tab: &headless_chrome::Tab,
    base_url: &str,
    _override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url_404 = format!("{}/this-page-does-not-exist", base_url);
    println!("Navigating to: {}", url_404);

    // 1. Navigate
    tab.navigate_to(&url_404)?;

    // Wait a brief moment to allow the server to respond and Chrome to parse
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 2. Dump the exact HTML that Chrome has loaded into the DOM
    if let Ok(eval) = tab.evaluate("document.documentElement.outerHTML", false) {
        if let Some(val) = eval.value {
            if let Some(html_str) = val.as_str() {
                println!(
                    "================ PAGE HTML DUMP ================\n{}\n================================================",
                    html_str
                );
            }
        }
    } else {
        println!(
            "================ PAGE HTML DUMP ================\nCOULD NOT EVALUATE JAVASCRIPT - PAGE MIGHT BE DEAD/BLANK\n================================================"
        );
    }

    // 3. Intentionally crash the runner so we can read the logs in GitHub Actions
    panic!("DEBUG: Check the 'PAGE HTML DUMP' above to see exactly what Chrome is looking at!");
}
