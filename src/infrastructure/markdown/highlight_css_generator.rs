use syntect::highlighting::ThemeSet;
use syntect::html::{ClassStyle, css_for_theme_with_class_style};

pub const CLASS_STYLE: ClassStyle = ClassStyle::SpacedPrefixed { prefix: "hl-" };

fn generate_highlight_css() -> Result<String, syntect::Error> {
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["InspiredGitHub"];
    css_for_theme_with_class_style(theme, CLASS_STYLE)
}

pub fn write_highlight_css(output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let css = generate_highlight_css()?;
    std::fs::write(output_path, css)?;
    Ok(())
}
