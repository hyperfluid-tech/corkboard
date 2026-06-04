use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_article_card(
    tab: &Tab,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    capture_selector_padded(
        tab,
        ".article-card-wrapper",
        "article_card",
        16.0,
        override_main,
    )
}
