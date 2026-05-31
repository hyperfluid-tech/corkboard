use headless_chrome::Tab;
use crate::helper::capture_selector_padded;

pub fn assert_table(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let name = if override_main { "welcome_article_table_main.png" } else { "welcome_article_table_gen.png" };
    capture_selector_padded(tab, ".prose table", name, 16.0)
}
