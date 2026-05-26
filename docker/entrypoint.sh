#!/bin/sh

PUID=${PUID:-1000}
PGID=${PGID:-1000}

if grep -q "^corkboard_group:" /etc/group; then
    sed -i "s/^corkboard_group:.*/corkboard_group:x:$PGID:/" /etc/group
else
    echo "corkboard_group:x:$PGID:" >> /etc/group
fi

if grep -q "^corkboard_user:" /etc/passwd; then
    sed -i "s/^corkboard_user:.*/corkboard_user:x:$PUID:$PGID:CORKBOARD User,,,:\/app:\/bin\/false/" /etc/passwd
else
    echo "corkboard_user:x:$PUID:$PGID:CORKBOARD User,,,:/app:/bin/false" >> /etc/passwd
fi

echo "Starting Corkboard initialization..."

if [ ! -f "/app/config.toml" ]; then
    echo "config.toml not found, generating default..."
    cat <<EOF > /app/config.toml
blog_title = "My blog"
blog_author = "Author"
blog_license = "CC 4.0 BY-SA"
blog_license_url = "https://creativecommons.org/licenses/by-sa/4.0/"
articles_dir = "articles"
port = 3000
truncate_lines = 15

# github_url = "https://github.com/your-username"
# linkedin_url = "https://linkedin.com/in/your-username"
# twitter_url = "https://twitter.com/your-username"
EOF
fi

ARTICLES_DIR=${CORKBOARD_ARTICLES_DIR:-/app/articles}
mkdir -p "$ARTICLES_DIR"

if [ -z "$(ls -A "$ARTICLES_DIR" 2>/dev/null)" ]; then
    echo "No markdown articles found in $ARTICLES_DIR. Creating a default welcome post..."
    cat <<EOF > "$ARTICLES_DIR/welcome.md"
---
title: "Welcome to Corkboard"
date: "$(date +%Y-%m-%d)"
---

Welcome to your new blog platform written in Rust!

This is an auto-generated welcome article.

## Features

*   **Skeuomorphic Design:** Paper textures, torn edges, and realistic shadows.
*   **Startup Parsing:** All markdown files are compiled into HTML on startup.
*   **Syntax Highlighting:** Fenced code blocks are highlighted using \`syntect\`.
*   **Lightweight:** Minimal client-side JavaScript.

Enjoy your writing journey!
EOF
fi

chown -R $PUID:$PGID /app

echo "Starting Corkboard blog platform..."
exec su-exec $PUID:$PGID "$@"
