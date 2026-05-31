use headless_chrome::Tab;
use crate::helper::capture_selector_padded;

pub fn assert_blockquote(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let name = if override_main { "welcome_article_blockquote_main.png" } else { "welcome_article_blockquote_gen.png" };
    capture_selector_padded(tab, ".prose blockquote", name, 16.0)
}
