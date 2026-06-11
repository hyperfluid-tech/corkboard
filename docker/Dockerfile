FROM node:20-alpine AS tailwind-builder

WORKDIR /build

COPY package*.json ./
COPY tailwind.config.js ./
COPY templates ./templates
COPY src ./src

RUN npm ci && npm run build:css

FROM rust:1-slim-bookworm AS builder

WORKDIR /usr/src/corkboard

RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

COPY . .
COPY --from=tailwind-builder /build/templates/style/tailwind.css /usr/src/corkboard/templates/style/tailwind.css

ARG CARGO_FEATURES=""
RUN cargo build --release --target x86_64-unknown-linux-musl --features "${CARGO_FEATURES}"

FROM alpine:latest

RUN apk add --no-cache \
    ca-certificates \
    su-exec \
    chromium \
    nss \
    freetype \
    harfbuzz \
    ttf-freefont
RUN addgroup -S corkboard_group && adduser -S corkboard_user -G corkboard_group
RUN mkdir -p /app && chown -R corkboard_user:corkboard_group /app

WORKDIR /app

COPY --from=builder /usr/src/corkboard/target/x86_64-unknown-linux-musl/release/corkboard /usr/local/bin/corkboard
COPY --from=builder /usr/src/corkboard/templates /app/templates
COPY --from=builder /usr/src/corkboard/articles/welcome.md /app/welcome.md
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

ENV RUST_LOG=corkboard=info,axum=info
ENV CORKBOARD_PORT=3000
ENV CORKBOARD_BASE_URL="http://localhost:3000"
ENV CORKBOARD_ARTICLES_DIR=/app/articles
ENV CORKBOARD_TRUNCATE_LINES=15
ENV CORKBOARD_THUMBNAIL_SHOW_ARTICLES=false
ENV CORKBOARD_SOCIAL_LINKS=""
ENV CORKBOARD_BLOG_LICENSE="CC 4.0 BY-SA"
ENV CORKBOARD_BLOG_LICENSE_URL="https://creativecommons.org/licenses/by-sa/4.0/"
# Git Repository Settings (Optional)
# ENV CORKBOARD_GIT__LINK=""
# ENV CORKBOARD_GIT__FOLDER=""
# ENV CORKBOARD_GIT__USERNAME=""
# ENV CORKBOARD_GIT__PASSWORD=""
# ENV CORKBOARD_GIT__BRANCH="main"

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["corkboard"]
