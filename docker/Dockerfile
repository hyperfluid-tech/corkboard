# Stage 1: Builder
FROM rust:1-slim-bookworm AS builder

# Set working directory
WORKDIR /usr/src/carbon

# Install musl target and musl-tools
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

# Copy all source files
COPY . .

# Build the release binary using the musl target
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Runtime
FROM alpine:latest

# Install required runtime dependencies for alpine (including su-exec for dropping privileges)
RUN apk add --no-cache ca-certificates su-exec

# Create a non-root user initially
RUN addgroup -S carbon_group && adduser -S carbon_user -G carbon_group

# Create the /app directory and set permissions
RUN mkdir -p /app && chown -R carbon_user:carbon_group /app

# Note: We do NOT switch to the non-root user here. 
# We stay as ROOT so the entrypoint can dynamically chown the mounted volume 
# and switch to the provided PUID/PGID at runtime!
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/carbon/target/x86_64-unknown-linux-musl/release/carbon /usr/local/bin/carbon

# Copy templates to /app/templates for static asset serving at runtime
COPY --from=builder /usr/src/carbon/templates /app/templates

# Copy and setup our new entrypoint script
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Default environment configuration (safely overridable at runtime)
ENV RUST_LOG=carbon=info,axum=info
ENV CARBON_PORT=3000
ENV CARBON_ARTICLES_DIR=/app/articles
ENV CARBON_LINKEDIN_URL=""
ENV CARBON_GITHUB_URL=""
ENV CARBON_TWITTER_URL=""

# Expose the application port
EXPOSE 3000

# Use the entrypoint script to map PUID/PGID before dropping to the carbon command
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["carbon"]
