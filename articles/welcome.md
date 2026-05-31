---
title: "Welcome to Corkboard"
subheading: "A fast, lightweight, and skeuomorphic blog platform built with Rust"
date: "2026-05-21"
thumbnail: "https://picsum.photos/800/600"
---

Welcome to Corkboard, a fast and lightweight blog platform written in Rust! 

This post serves as a live demonstration of Corkboard's skeuomorphic layout and rendering features.

### Core Features

*   **Skeuomorphic Design:** Realistically shaded paper cards, pushpins, chalkboard styling, and WebGL procedural canvas grains.
*   **Zero-Config Startup:** All articles are compiled from Markdown to HTML on startup. No database or client-side frameworks needed.
*   **Server-Side Syntax Highlighting:** Fenced code blocks are styled using `syntect` for rich, high-fidelity colorization.
*   **Fully Responsive:** The layout scales cleanly from mobile devices up to ultra-wide displays.

---

## 1. Heading Elements

Below is a demonstration of all heading levels available in Markdown.

# Heading 1 (H1)
## Heading 2 (H2)
### Heading 3 (H3)
#### Heading 4 (H4)
##### Heading 5 (H5)
###### Heading 6 (H6)

---

## 2. Text Formatting

Markdown allows for standard text decorations to emphasize your thoughts:

- **Bold Text**: Styled with `**Bold Text**` or `__Bold Text__` to emphasize key points.
- *Italic Text*: Styled with `*Italic Text*` or `_Italic Text_` for subtle emphasis.
- ***Bold & Italic***: Styled with `***Bold & Italic***` for strong emphasis.
- ~~Strikethrough~~: Styled with `~~Strikethrough~~` to denote deleted or deprecated content.
- Inline Code: Styled with backticks like `let x = 42;` to highlight code symbols in paragraphs.
- Links: Easily link to [external sites](https://google.com) or use auto-links: <https://github.com>.

---

## 3. Lists and Tasks

### Bulleted List (Unordered)
*   First level item
    *   Second level nested item
        *   Third level nested item
*   Another first level item

### Numbered List (Ordered)
1.  Verify the input parameters.
2.  Parse the incoming Markdown content:
    1.  Extract YAML frontmatter.
    2.  Tokenize using the parser.
3.  Render the final HTML structure.

### Interactive Task List
- [x] Implement Rust-based markdown processing
- [x] Apply vintage paper shader textures
- [ ] Add retro audio feedback for button clicks
- [ ] Add dark mode theme options

---

## 4. Blockquotes

Blockquotes are perfect for highlighting pull-quotes or references from external sources.

> "Good design is as little design as possible. Less, but better – because it concentrates on the essential aspects, and the products are not burdened with non-essentials."
> 
> — Dieter Rams

---

## 5. Rich Code Blocks

Here is how we set up a simple Axum handler in Rust:

```rust
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Skeuomorphic World!</h1>")
}
```

This code snippet is syntax highlighted using **syntect** at compile time! We can also highlight CSS:

```css
.paper-card {
    background-color: #ffffff;
    border: 1px solid #e4e2dd;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
}
```

And a simple plain text fallback without language:

```
Simple plain text without highlighting
Useful for logs or command output
```

---

## 6. Tabular Data

Tables are rendered with custom headers, borders, and support for alignment:

| Feature Name | Complexity | Skeuomorphic Style | Support Level |
| :--- | :---: | :---: | ---: |
| **Grid Tables** | Medium | Notebook Ledger | 100% |
| **Code Highlighting** | High | Torn Paper Edges | 95% |
| **Pushpins** | Low | Realistic Shadow Card | 100% |
| **Paper Shading** | High | WebGL Grain Shader | 90% |

---

## 7. Tactile Visuals

All images parsed from Markdown are automatically compiled into a skeuomorphic, tipped-in card style with waxy tape decorations and soft shadows:

![Random Internet Image](https://picsum.photos/800/600 "Skeuomorphic image display card")

---

## 8. Footnotes

Footnotes are excellent for providing extra context without cluttering the main body text[^1]. You can also have multiple footnotes in a single article[^2].

[^1]: This is the first footnote. It will appear at the bottom of the article card.
[^2]: This is the second footnote, demonstrating that footnotes can be defined anywhere and will be collected at the bottom.
