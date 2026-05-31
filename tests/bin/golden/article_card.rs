use headless_chrome::Tab;
use crate::helper::capture_selector_padded;

pub fn assert_article_card(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let name = if override_main { "article_card_main.png" } else { "article_card_gen.png" };
    capture_selector_padded(tab, ".article-card-wrapper", name, 16.0)
}
