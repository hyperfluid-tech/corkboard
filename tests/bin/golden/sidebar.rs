use headless_chrome::Tab;
use crate::helper::capture_selector_padded;

pub fn assert_sidebar(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let name = if override_main { "sidebar_main.png" } else { "sidebar_gen.png" };
    capture_selector_padded(tab, "#sidebar", name, 0.0)
}
