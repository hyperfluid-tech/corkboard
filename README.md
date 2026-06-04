# Corkboard

[![Docker Status](https://img.shields.io/docker/v/gilnobrega/corkboard?label=Docker%20Image&style=flat-square&color=blue)](https://hub.docker.com/r/gilnobrega/corkboard)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-lightgrey.svg?style=flat-square)](https://opensource.org/licenses/MPL-2.0)

✨ **Live Demo:** [corkboard.hyperfluid.tech](https://corkboard.hyperfluid.tech)

A skeuomorphic blog platform built with Rust. Write Markdown files, and Corkboard turns them into a fully styled website that looks and feels like paper notes pinned to a cork board.

- **Zero friction:** Drop `.md` files into a folder and run. No database, no build step, no JavaScript framework.
- **Rich Markdown:** Syntax-highlighted code blocks, chalkboard-styled tables, footnotes, task lists, and image cards — all out of the box.
- **Tactile design:** WebGL paper textures, procedural torn-paper edges, pushpin decorations, and highlighter-pen title effects.
- **Open-source & Self-hostable:** Freely fork, customize, and run your own deployment via Docker or Cargo.

## Table of Contents

- [Getting Started](#getting-started)
  - [Running Locally](#running-locally)
  - [Deploying with Docker](#deploying-with-docker)
- [Writing Articles](#writing-articles)
- [Tech Stack](#tech-stack)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [Testing (Golden Tests)](TESTING.md)
- [License](#license)

## Getting Started

### Running Locally

*Requires [Rust and Cargo](https://rustup.rs/) installed on your machine.*

```bash
cargo run
```

Once running, navigate to `http://localhost:3000` in your browser.

### Deploying with Docker

Create a `docker-compose.yml` file:

```yaml
version: '3.8'

services:
  corkboard:
    image: gilnobrega/corkboard:latest
    container_name: corkboard_blog
    restart: unless-stopped
    ports:
      - "8080:3000"
    volumes:
      - ./articles:/app/articles
    environment:
      - PUID=1000
      - PGID=1000
      - CORKBOARD_BLOG_TITLE=My blog
      - CORKBOARD_BLOG_AUTHOR=Author
```

Then start the service:

```bash
docker-compose up -d
```

Once the container is up, navigate to `http://localhost:8080` in your browser.

The `articles` directory will be created if it does not exist, and a welcome post will be generated automatically. You can configure any setting using environment variables prefixed with `CORKBOARD_`. For more advanced control, see the reference [docker/docker-compose.yml](docker/docker-compose.yml).

## Writing Articles

Place Markdown files inside the `articles/` directory. Each file needs a YAML frontmatter block:

```markdown
---
title: "My First Post"
date: "2026-01-15"
subheading: "An optional subtitle shown below the title"
---

Your content goes here. Standard Markdown is supported:
**bold**, *italic*, `inline code`, lists, tables, images,
footnotes, and fenced code blocks with syntax highlighting.
```

Articles are sorted by date (newest first) and displayed on a single scrollable page. Long articles are truncated with a "Read more" link to their dedicated page.

## Tech Stack

- **Rust ([Axum](https://github.com/tokio-rs/axum))**: Fast, async HTTP routing.
- **[Askama](https://github.com/djc/askama)**: Strongly-typed, compiled HTML templates.
- **[Syntect](https://github.com/trishume/syntect)**: Server-side syntax highlighting for fenced code blocks.
- **[Pulldown-cmark](https://github.com/raphlinus/pulldown-cmark)**: CommonMark-compliant Markdown parsing with extensions.
- **WebGL**: Procedural paper texture shader for realistic card surfaces.

## Configuration

All settings live in `config.toml` at the project root:

```toml
blog_title = "My blog"
blog_author = "Author"
blog_license = "CC 4.0 BY-SA"
blog_license_url = "https://creativecommons.org/licenses/by-sa/4.0/"
articles_dir = "articles"
port = 3000
truncate_lines = 15
thumbnail_show_articles = false

# github_url = "https://github.com/your-username"
# linkedin_url = "https://linkedin.com/in/your-username"
# twitter_url = "https://twitter.com/your-username"
```

Every setting can be configured via a `config.toml` file or overridden using an environment variable prefixed with `CORKBOARD_`.

| Setting | Environment Variable | Default | Description |
| --- | --- | --- | --- |
| `blog_title` | `CORKBOARD_BLOG_TITLE` | `My blog` | Shown in the header and page title |
| `blog_author` | `CORKBOARD_BLOG_AUTHOR` | `Author` | Shown in the footer copyright |
| `blog_license` | `CORKBOARD_BLOG_LICENSE` | `CC 4.0 BY-SA` | License name in the footer |
| `blog_license_url` | `CORKBOARD_BLOG_LICENSE_URL` | *(CC link)* | URL the license links to |
| `articles_dir` | `CORKBOARD_ARTICLES_DIR` | `articles` | Directory to scan for `.md` files |
| `port` | `CORKBOARD_PORT` | `3000` | HTTP port |
| `truncate_lines` | `CORKBOARD_TRUNCATE_LINES` | `15` | Markdown lines shown per card before truncation |
| `thumbnail_show_articles` | `CORKBOARD_THUMBNAIL_SHOW_ARTICLES` | `false` | Whether to show article snippets in the blog's generated thumbnail |
| `github_url` | `CORKBOARD_GITHUB_URL` | *(empty)* | GitHub link in the footer |
| `linkedin_url` | `CORKBOARD_LINKEDIN_URL` | *(empty)* | LinkedIn link in the footer |
| `twitter_url` | `CORKBOARD_TWITTER_URL` | *(empty)* | Twitter/X link in the footer |

## Contributing

Contributions are always welcome! Whether it's a bug report, a new feature, or a typo fix, feel free to open an issue or submit a pull request.

Please see [TESTING.md](TESTING.md) for guidelines on how to run and update the visual regression tests when changing layout or styling elements.

## License

This project is licensed under the [MPL 2.0 License](LICENSE).

The paper texture shader (`templates/paper-shader.js`) is adapted from [@paper-design/shaders](https://github.com/paper-design/shaders) under the [PolyForm Shield License 1.0.0](THIRD-PARTY-LICENSES.md).
