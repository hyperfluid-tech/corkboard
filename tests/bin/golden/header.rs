use headless_chrome::Tab;
use crate::helper::capture_selector_padded;

pub fn assert_header(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let name = if override_main { "header_main.png" } else { "header_gen.png" };
    capture_selector_padded(tab, "header", name, 32.0)
}
