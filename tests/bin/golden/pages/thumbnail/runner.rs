use crate::helper::capture_full_page;
use headless_chrome::Tab;

pub fn assert_thumbnail_group(
    tab: &Tab,
    base_url: &str,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url = format!("{}/thumbnail", base_url);
    println!("Navigating to: {}", url);
    tab.navigate_to(&url)?;
    tab.wait_until_navigated()?;

    // Wait for the WebGL paper shader to signal it has finished rendering
    tab.wait_for_element("body[data-shaders-ready]")?;

    capture_full_page(tab, "thumbnail", 1200, 630, override_main)
}
