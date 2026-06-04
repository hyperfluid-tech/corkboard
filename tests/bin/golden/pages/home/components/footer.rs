use crate::helper::capture_selector_padded;
use headless_chrome::Tab;
use std::time::Duration;

pub fn assert_footer(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    tab.evaluate("window.scrollTo(0, document.body.scrollHeight)", false)?;
    std::thread::sleep(Duration::from_millis(500));
    capture_selector_padded(tab, "footer", "footer", 32.0, override_main)
}
