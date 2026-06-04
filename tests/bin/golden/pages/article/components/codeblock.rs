use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_codeblock(
    tab: &Tab,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    capture_selector_padded(
        tab,
        ".prose pre",
        "welcome_article_codeblock",
        16.0,
        override_main,
    )
}
