use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_not_found_group(
    tab: &Tab,
    base_url: &str,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Navigate to a non-existent URL to trigger the 404 page
    let url_404 = format!("{}/this-page-does-not-exist", base_url);
    println!("Navigating to: {}", url_404);
    tab.navigate_to(&url_404)?;

    // Wait for the WebGL paper shader to signal it has finished rendering
    tab.wait_for_element("body[data-shaders-ready]")?;

    // Capture the 404 card element
    capture_selector_padded(
        tab,
        ".article-card-wrapper",
        "not_found_card",
        16.0,
        override_main,
    )
}
