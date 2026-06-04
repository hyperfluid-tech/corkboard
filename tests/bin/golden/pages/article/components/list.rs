use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_list(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    capture_selector_padded(
        tab,
        ".prose ul",
        "welcome_article_list",
        16.0,
        override_main,
    )
}
