use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_image(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    capture_selector_padded(
        tab,
        ".prose .tipped-image-container",
        "welcome_article_image",
        16.0,
        override_main,
    )
}
