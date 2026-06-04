use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_blockquote(
    tab: &Tab,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    capture_selector_padded(
        tab,
        ".prose blockquote",
        "welcome_article_blockquote",
        16.0,
        override_main,
    )
}
