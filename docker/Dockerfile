# Stage 1: Builder
FROM rust:1.80-slim AS builder
WORKDIR /app

# Install compilation essentials
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real source files and templates (Askama compiles templates into the binary)
COPY src ./src
COPY templates ./templates
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Minimal runner
FROM debian:bookworm-slim AS runner
WORKDIR /app

# Install SSL certificates and basic utilities
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy artifacts from builder
COPY --from=builder /app/target/release/carbon /app/carbon
COPY --from=builder /app/templates /app/templates
COPY --from=builder /app/Cargo.toml /app/Cargo.toml
COPY docker/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 8080

# Environment configurations
ENV RUST_LOG=carbon=info,axum=info
ENV CARBON_PORT=8080
ENV CARBON_ARTICLES_DIR=/app/articles

ENTRYPOINT ["/app/entrypoint.sh"]
