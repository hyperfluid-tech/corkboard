pub fn escape_html(text: &str) -> String {
    let mut escaped = String::new();
    for character in text.chars() {
        match character {
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '&' => escaped.push_str("&amp;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#x27;"),
            _ => escaped.push(character),
        }
    }
    escaped
}

pub fn close_open_fences(markdown: &str) -> String {
    let mut in_code_block = false;
    for line in markdown.lines() {
        if line.trim().starts_with("```") {
            in_code_block = !in_code_block;
        }
    }
    if in_code_block {
        format!("{}\n```", markdown)
    } else {
        markdown.to_string()
    }
}
