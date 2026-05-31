use headless_chrome::Tab;
use crate::helper::capture_selector_padded;

pub fn assert_list(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let name = if override_main { "welcome_article_list_main.png" } else { "welcome_article_list_gen.png" };
    capture_selector_padded(tab, ".prose ul", name, 16.0)
}
