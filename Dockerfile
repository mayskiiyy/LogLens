# Stage 1: Build SvelteKit Web Frontend
FROM node:20-alpine AS frontend-builder
WORKDIR /app
COPY package.json package-lock.json ./
COPY apps/web/package.json ./apps/web/
RUN npm ci
COPY apps/web ./apps/web
RUN npm run build --workspace=apps/web

# Stage 2: Build Rust Backend
FROM rust:1.80-slim AS rust-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY migrations ./migrations
RUN cargo build --release -p loglens-server

# Stage 3: Minimal Production Runtime
FROM debian:bookworm-slim AS runner
WORKDIR /app

LABEL org.opencontainers.image.title="LogLens" \
      org.opencontainers.image.description="Local-first log exploration, search, grouping, and live monitoring" \
      org.opencontainers.image.licenses="Apache-2.0" \
      org.opencontainers.image.source="https://github.com/mayskiiyy/LogLens"

RUN apt-get update && apt-get install -y ca-certificates curl sqlite3 && rm -rf /var/lib/apt/lists/*
RUN groupadd -g 1000 loglens && useradd -u 1000 -g loglens -s /bin/sh -m loglens

RUN mkdir -p /data/uploads /data/exports /data/tmp && chown -R loglens:loglens /data /app

COPY --from=rust-builder --chown=loglens:loglens /app/target/release/loglens-server /app/loglens-server
COPY --from=frontend-builder --chown=loglens:loglens /app/apps/web/build /app/apps/web/build

USER loglens
EXPOSE 8080

ENV LOGLENS_HOST=0.0.0.0 \
    LOGLENS_PORT=8080 \
    LOGLENS_DATABASE_URL=sqlite:///data/loglens.db \
    LOGLENS_DATA_DIR=/data

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/api/v1/health || exit 1

ENTRYPOINT ["/app/loglens-server"]
