use headless_chrome::Tab;
use std::time::Duration;

use crate::helper::capture_full_page;

pub fn assert_thumbnail(
    tab: &Tab,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url = format!("http://localhost:3000/thumbnail?deterministic=true");
    println!("Navigating to: {}", url);
    tab.navigate_to(&url)?;

    // Wait for the WebGL paper shader to render
    std::thread::sleep(Duration::from_millis(2000));

    capture_full_page(tab, "thumbnail", 1200, 630, override_main)
}
