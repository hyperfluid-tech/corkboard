use crate::helper::capture_selector_padded;
use headless_chrome::Tab;
pub fn assert_not_found_group(
    tab: &headless_chrome::Tab,
    base_url: &str,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url_404 = format!("{}/this-page-does-not-exist", base_url);
    println!("Navigating to: {}", url_404);
    tab.navigate_to(&url_404)?;
    tab.wait_until_navigated()?;

    // 🔍 X-RAY: Dump the exact HTML Chrome is seeing to the GitHub Actions console
    if let Ok(eval) = tab.evaluate("document.documentElement.outerHTML", false) {
        if let Some(val) = eval.value {
            if let Some(html_str) = val.as_str() {
                println!(
                    "================ PAGE HTML DUMP ================\n{}\n================================================",
                    html_str
                );
            }
        }
    }

    println!("Waiting for shaders...");
    // If it times out, it will crash here. But we will have already printed the HTML!
    tab.wait_for_element("body[data-shaders-ready]")?;

    println!("Shaders ready! Capturing card...");
    crate::helper::capture_selector_padded(
        tab,
        ".article-card-wrapper",
        "not_found_card",
        16.0,
        override_main,
    )
}
