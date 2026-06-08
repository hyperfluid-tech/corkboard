use headless_chrome::Tab;

use super::components;

pub fn assert_home_group(
    tab: &Tab,
    base_url: &str,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url_home = format!("{}/", base_url);
    println!("Navigating to: {}", url_home);
    tab.navigate_to(&url_home)?;
    tab.wait_until_navigated()?;

    // Wait for the WebGL paper shader to signal it has finished rendering
    tab.wait_for_element("body[data-shaders-ready]")?;

    let mut all_matched = true;
    all_matched &= components::header::assert_header(tab, override_main)?;
    all_matched &= components::sidebar::assert_sidebar(tab, override_main)?;
    all_matched &= components::footer::assert_footer(tab, override_main)?;
    all_matched &= components::article_card::assert_article_card(tab, override_main)?;

    Ok(all_matched)
}
