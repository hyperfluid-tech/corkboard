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
base_url = "http://localhost:3000"
articles_dir = "articles"
port = 3000
truncate_lines = 15
thumbnail_show_articles = false

# social_links = [
#     "https://github.com/your-username",
#     "https://linkedin.com/in/your-username",
#     "https://twitter.com/your-username"
# ]
EOF
fi

ARTICLES_DIR=${CORKBOARD_ARTICLES_DIR:-/app/articles}
mkdir -p "$ARTICLES_DIR"
mkdir -p /app/assets


if [ -z "$(ls -A "$ARTICLES_DIR" 2>/dev/null)" ]; then
    echo "No markdown articles found in $ARTICLES_DIR. Creating a default welcome post..."
    cp /app/welcome.md "$ARTICLES_DIR/welcome.md"
    CURRENT_DATE=$(date +%Y-%m-%d)
    sed -i "s/date: .*/date: \"$CURRENT_DATE\"/" "$ARTICLES_DIR/welcome.md"
fi

chown -R $PUID:$PGID /app

echo "Starting Corkboard blog platform..."
exec su-exec $PUID:$PGID "$@"
