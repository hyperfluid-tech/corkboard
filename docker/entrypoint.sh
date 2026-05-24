#!/bin/bash
set -e

echo "Starting Carbon initialization..."

# Check if config.toml exists in the runtime environment, else write default
if [ ! -f "/app/config.toml" ]; then
    echo "config.toml not found, generating default..."
    cat <<EOF > /app/config.toml
blog_title = "My blog"
blog_author = "Author"
blog_license = "CC 4.0 BY-SA"
blog_license_url = "https://creativecommons.org/licenses/by-sa/4.0/"
articles_dir = "articles"
port = 8080
EOF
fi

# Ensure the articles directory is set up
ARTICLES_DIR=${CARBON_ARTICLES_DIR:-/app/articles}
mkdir -p "$ARTICLES_DIR"

# If no posts exist, create a default greeting post
if [ -z "$(ls -A "$ARTICLES_DIR")" ]; then
    echo "No markdown articles found in $ARTICLES_DIR. Creating a default welcome post..."
    cat <<EOF > "$ARTICLES_DIR/welcome.md"
---
title: "Welcome to The Rusty Ledger"
date: "$(date +%Y-%m-%d)"
---

Welcome to your new single-page skeuomorphic blog platform written in Rust!

This is an auto-generated welcome article.

## Features

*   **Skeuomorphic Design:** Beautiful leather mat background, ruled notebook index, and stationery pages.
*   **Compile-time Parsing:** All markdown files are compiled into html on startup.
*   **Syntax Highlighting:** Fenced code blocks are styled automatically at compile time using \`syntect\`.
*   **Ultra-lightweight:** Zero client-side JavaScript overhead (except for smooth scrolling controls).

Enjoy your writing journey!
EOF
fi

echo "Starting Carbon blog platform..."
exec /app/carbon
