#!/bin/sh

# Default to UID 1000 and GID 1000 if not provided
PUID=${PUID:-1000}
PGID=${PGID:-1000}

# Robustly update or append group and user IDs 
# Bypasses Alpine's unpredictable adduser/deluser behavior
if grep -q "^carbon_group:" /etc/group; then
    sed -i "s/^carbon_group:.*/carbon_group:x:$PGID:/" /etc/group
else
    echo "carbon_group:x:$PGID:" >> /etc/group
fi

if grep -q "^carbon_user:" /etc/passwd; then
    sed -i "s/^carbon_user:.*/carbon_user:x:$PUID:$PGID:CARBON User,,,:\/app:\/bin\/false/" /etc/passwd
else
    echo "carbon_user:x:$PUID:$PGID:CARBON User,,,:/app:/bin/false" >> /etc/passwd
fi

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
port = 3000
EOF
fi

# Ensure the articles directory is set up
ARTICLES_DIR=${CARBON_ARTICLES_DIR:-/app/articles}
mkdir -p "$ARTICLES_DIR"

# If no posts exist, create a default greeting post
if [ -z "$(ls -A "$ARTICLES_DIR" 2>/dev/null)" ]; then
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

# Apply permissions on the app directory 
chown -R $PUID:$PGID /app

echo "Starting Carbon blog platform..."
# Drop privileges using direct numeric mapping
exec su-exec $PUID:$PGID "$@"
