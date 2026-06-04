use headless_chrome::Tab;
use std::time::Duration;

use super::components;

pub fn assert_article_group(
    tab: &Tab,
    base_url: &str,
    override_main: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url_article = format!("{}/article/welcome-to-corkboard", base_url);
    println!("Navigating to: {}", url_article);
    tab.navigate_to(&url_article)?;

    // Wait for the page and shaders to render
    std::thread::sleep(Duration::from_millis(2000));

    let mut all_matched = true;
    all_matched &= components::blockquote::assert_blockquote(tab, override_main)?;
    all_matched &= components::table::assert_table(tab, override_main)?;
    all_matched &= components::codeblock::assert_codeblock(tab, override_main)?;
    all_matched &= components::image::assert_image(tab, override_main)?;
    all_matched &= components::list::assert_list(tab, override_main)?;
    all_matched &= components::headings::assert_headings(tab, override_main)?;
    all_matched &= components::separator::assert_separator(tab, override_main)?;

    Ok(all_matched)
}
