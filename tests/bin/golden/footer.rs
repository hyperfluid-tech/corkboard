use headless_chrome::Tab;
use std::time::Duration;
use crate::helper::capture_selector_padded;

pub fn assert_footer(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    tab.evaluate("window.scrollTo(0, document.body.scrollHeight)", false)?;
    std::thread::sleep(Duration::from_millis(500));
    let name = if override_main { "footer_main.png" } else { "footer_gen.png" };
    capture_selector_padded(tab, "footer", name, 32.0)
}
