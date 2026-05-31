use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_header(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    capture_selector_padded(tab, "header", "header", 32.0, override_main)
}
