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
COPY --from=tailwind-builder /build/templates/tailwind.css /usr/src/corkboard/templates/tailwind.css

RUN cargo build --release --target x86_64-unknown-linux-musl

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
ENV CORKBOARD_ARTICLES_DIR=/app/articles
ENV CORKBOARD_TRUNCATE_LINES=15
ENV CORKBOARD_THUMBNAIL_SHOW_ARTICLES=false
ENV CORKBOARD_LINKEDIN_URL=""
ENV CORKBOARD_GITHUB_URL=""
ENV CORKBOARD_TWITTER_URL=""

EXPOSE 3000

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["corkboard"]
