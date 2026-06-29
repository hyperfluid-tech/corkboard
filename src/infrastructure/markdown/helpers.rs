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
