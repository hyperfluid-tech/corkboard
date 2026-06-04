use crate::helper::capture_selector_padded;
use headless_chrome::Tab;

pub fn assert_headings(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let mut all_matched = true;

    all_matched &=
        capture_selector_padded(tab, ".prose h1", "welcome_article_h1", 16.0, override_main)?;
    all_matched &=
        capture_selector_padded(tab, ".prose h2", "welcome_article_h2", 16.0, override_main)?;
    all_matched &=
        capture_selector_padded(tab, ".prose h3", "welcome_article_h3", 16.0, override_main)?;
    all_matched &=
        capture_selector_padded(tab, ".prose h4", "welcome_article_h4", 16.0, override_main)?;
    all_matched &=
        capture_selector_padded(tab, ".prose h5", "welcome_article_h5", 16.0, override_main)?;
    all_matched &=
        capture_selector_padded(tab, ".prose h6", "welcome_article_h6", 16.0, override_main)?;

    Ok(all_matched)
}
