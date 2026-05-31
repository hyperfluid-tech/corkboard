use headless_chrome::Tab;
use crate::helper::capture_selector_padded;

pub fn assert_headings(tab: &Tab, override_main: bool) -> Result<bool, Box<dyn std::error::Error>> {
    let mut all_matched = true;

    let h1_name = if override_main { "welcome_article_h1_main.png" } else { "welcome_article_h1_gen.png" };
    all_matched &= capture_selector_padded(tab, ".prose h1", h1_name, 16.0)?;

    let h2_name = if override_main { "welcome_article_h2_main.png" } else { "welcome_article_h2_gen.png" };
    all_matched &= capture_selector_padded(tab, ".prose h2", h2_name, 16.0)?;

    let h3_name = if override_main { "welcome_article_h3_main.png" } else { "welcome_article_h3_gen.png" };
    all_matched &= capture_selector_padded(tab, ".prose h3", h3_name, 16.0)?;

    let h4_name = if override_main { "welcome_article_h4_main.png" } else { "welcome_article_h4_gen.png" };
    all_matched &= capture_selector_padded(tab, ".prose h4", h4_name, 16.0)?;

    let h5_name = if override_main { "welcome_article_h5_main.png" } else { "welcome_article_h5_gen.png" };
    all_matched &= capture_selector_padded(tab, ".prose h5", h5_name, 16.0)?;

    let h6_name = if override_main { "welcome_article_h6_main.png" } else { "welcome_article_h6_gen.png" };
    all_matched &= capture_selector_padded(tab, ".prose h6", h6_name, 16.0)?;

    Ok(all_matched)
}
