use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_not_found_group(
    tab: &headless_chrome::Tab,
    base_url: &str,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url_404 = format!("{}/this-page-does-not-exist", base_url);
    println!("Navigating to: {}", url_404);

    // Navigate to a non-existent URL to trigger the 404 page
    tab.navigate_to(&url_404)?;

    // We do NOT use wait_until_navigated() here because rust-headless-chrome
    // throws an error when it encounters an HTTP 404 status code.
    // Instead, since this page runs in a fresh tab, we can safely rely
    // on wait_for_element to block until the new page loads and renders shaders.
    tab.wait_for_element("body[data-shaders-ready]")?;

    crate::helper::capture_selector_padded(
        tab,
        ".article-card-wrapper",
        "not_found_card",
        16.0,
        override_main,
    )
}
