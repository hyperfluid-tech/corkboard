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
  - [Deploying with Docker (Recommended)](#deploying-with-docker)
  - [Running Locally](#running-running-locally)
- [Writing Articles](#writing-articles)
  - [Using Local Assets](#using-local-assets)
- [Tech Stack](#tech-stack)
- [Configuration](#configuration)
- [Contributing](#contributing)
- [Testing (Golden Tests)](TESTING.md)
- [License](#license)

## Getting Started

### Deploying with Docker (Recommended)

Corkboard is available in two Docker image variants:
1. **Default/Lean Variant (`gilnobrega/corkboard:latest`):** A lightweight image designed only for loading local Markdown files.
2. **Git-Enabled Variant (`gilnobrega/corkboard:latest-git`):** A variant compiled with the Git feature to automatically pull articles from a remote Git repository.

Create a `docker-compose.yml` file using your preferred variant (e.g., the lean version):

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
      - ./assets:/app/assets
    environment:
      - PUID=1000
      - PGID=1000
      - CORKBOARD_BLOG_TITLE=My blog
      - CORKBOARD_BLOG_AUTHOR=Author
      - CORKBOARD_BASE_URL=http://your-domain-here.com:8080
```

Then start the service:

```bash
docker-compose up -d
```

Once the container is up, navigate to `http://localhost:8080` in your browser.

The `articles` directory will be created if it does not exist, and a welcome post will be generated automatically. You can configure any setting using environment variables prefixed with `CORKBOARD_`. For more advanced control, see the reference [docker/docker-compose.yml](docker/docker-compose.yml).

### Running Locally

*Requires [Rust and Cargo](https://rustup.rs/) and [Node.js and NPM](https://nodejs.org/) installed on your machine.*

First, install the frontend dependencies and compile the Tailwind CSS stylesheet:

```bash
npm install
npm run build:css
```

Then run the application (defaults to local-only files):

```bash
cargo run
```

If you want to run the application with the Git data source enabled:

```bash
cargo run --features git
```

If you are modifying HTML templates or styles, you can run the CSS compiler in watch mode in a separate terminal:

```bash
npm run watch:css
```

Once running, navigate to `http://localhost:3000` in your browser.

## Writing Articles

Place Markdown files inside the `articles/` directory. Each file needs a YAML frontmatter block:

```markdown
---
title: "My First Post"
date: "2026-01-15"
description: "An optional description shown below the title"
thumbnail: "/assets/my-thumbnail.webp"
---

Your content goes here. Standard Markdown is supported:
**bold**, *italic*, `inline code`, lists, tables, images,
footnotes, and fenced code blocks with syntax highlighting.
```

Articles are sorted by date (newest first) and displayed on a single scrollable page. Long articles are truncated with a "Read more" link to their dedicated page.

### Using Local Assets

You can reference local images and files from your articles by placing them in the `assets/` directory at the project root (or `/app/assets` inside a Docker container).

Reference assets in your Markdown using the `/assets/` path prefix:

```markdown
---
title: "My Post with Images"
date: "2026-01-15"
thumbnail: "/assets/my-thumbnail.webp"
---

Here is a photo from my local assets folder:

![A beautiful sunset](/assets/photos/sunset.jpg "Sunset over the mountains")
```

You can also organize assets into subdirectories:

```
assets/
├── photos/
│   ├── sunset.jpg
│   └── portrait.png
├── diagrams/
│   └── architecture.svg
└── my-thumbnail.webp
```

> **Security note:** Only asset files that are explicitly referenced in your Markdown articles (via `![](...)` image syntax or the `thumbnail` frontmatter field) will be served. Any file placed in `assets/` that is not referenced by an article will return a `403 Forbidden` response. This prevents accidental exposure of unreferenced files.

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
base_url = "https://your-domain-here.com"
articles_dir = "articles"
port = 3000
truncate_lines = 15
thumbnail_show_articles = false

# social_links = [
#     "https://github.com/your-username",
#     "https://linkedin.com/in/your-username",
#     "https://twitter.com/your-username"
# ]

# cors_allowed_origins = ["http://localhost:3000"]
# csp_allowed_origins = [
#     "http://localhost:3000",
#     "https://fonts.googleapis.com",
#     "https://fonts.gstatic.com"
# ]

# [git]
# link = "https://github.com/user/repo"
# folder = ""
# username = "user"
# password = "pat"
# branch = "main"
```

Every setting can be configured via a `config.toml` file or overridden using an environment variable prefixed with `CORKBOARD_`. For nested configuration fields like `git`, the environment variable should use a double underscore separator (`__`), for example: `CORKBOARD_GIT__LINK`.

<details>
<summary><strong>Advanced Configuration & Environment Variables</strong></summary>

### Core Settings

| Setting | Environment Variable | Default | Description |
| --- | --- | --- | --- |
| `blog_title` | `CORKBOARD_BLOG_TITLE` | `My blog` | Shown in the header and page title |
| `blog_author` | `CORKBOARD_BLOG_AUTHOR` | `Author` | Shown in the footer copyright |
| `blog_license` | `CORKBOARD_BLOG_LICENSE` | `CC 4.0 BY-SA` | License name in the footer |
| `blog_license_url` | `CORKBOARD_BLOG_LICENSE_URL` | *(CC link)* | URL the license links to |
| `base_url` | `CORKBOARD_BASE_URL` | `https://your-domain-here.com` | The public/external URL of the blog (used for RSS, sitemaps, social cards) |
| `articles_dir` | `CORKBOARD_ARTICLES_DIR` | `articles` | Directory to scan for `.md` files |
| `port` | `CORKBOARD_PORT` | `3000` | Internal HTTP port the application listens on |
| `truncate_lines` | `CORKBOARD_TRUNCATE_LINES` | `15` | Markdown lines shown per card before truncation |
| `thumbnail_show_articles` | `CORKBOARD_THUMBNAIL_SHOW_ARTICLES` | `false` | Whether to show article snippets in the blog's generated thumbnail |
| `social_links` | `CORKBOARD_SOCIAL_LINKS` | *(empty list)* | List of social/external URLs to show in the footer (comma-separated in env) |
| `cors_allowed_origins` | `CORKBOARD_CORS_ALLOWED_ORIGINS` | *(base_url)* | List of allowed CORS origins (comma-separated list in env) |
| `csp_allowed_origins` | `CORKBOARD_CSP_ALLOWED_ORIGINS` | *(base_url + Google Fonts)* | List of allowed CSP origins (comma-separated list in env) |

### Git Settings
> [!NOTE]
> These settings require building with the `git` feature (e.g. `cargo run --features git`), or using the `latest-git` Docker image.

| Setting | Environment Variable | Default | Description |
| --- | --- | --- | --- |
| `git.link` | `CORKBOARD_GIT__LINK` | *(none)* | Git repository HTTPS/SSH URL to clone and load remote articles from |
| `git.folder` | `CORKBOARD_GIT__FOLDER` | `""` | Subfolder within the git repository containing articles (defaults to root) |
| `git.assets_folder` | `CORKBOARD_GIT__ASSETS_FOLDER` | `""` | Subfolder within the git repository containing assets to be copied to the local assets folder |
| `git.username` | `CORKBOARD_GIT__USERNAME` | *(none)* | Optional username for basic auth (only needed if not public repository) |
| `git.password` | `CORKBOARD_GIT__PASSWORD` | *(none)* | Optional password or PAT (Personal Access Token) for authenticated access |
| `git.branch` | `CORKBOARD_GIT__BRANCH` | `main` | Target git branch name to check out |

</details>

## Contributing

Contributions are always welcome! Whether it's a bug report, a new feature, or a typo fix, feel free to open an issue or submit a pull request.

Please see [TESTING.md](TESTING.md) for guidelines on how to run and update the visual regression tests when changing layout or styling elements.

## License

This project is licensed under the [MPL 2.0 License](LICENSE).

The paper texture shader (`templates/paper-shader.js`) is adapted from [@paper-design/shaders](https://github.com/paper-design/shaders) under the [PolyForm Shield License 1.0.0](THIRD-PARTY-LICENSES.md).
