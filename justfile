default:
  @just --list

setup:
  npm install
  cargo check --workspace

dev:
  npm run dev --prefix apps/web

dev-server:
  cargo run -p loglens-server

dev-web:
  npm run dev --prefix apps/web

test: test-rust test-web

test-rust:
  cargo test --workspace --all-features

test-web:
  npm run test --prefix apps/web

lint:
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  npm run lint --prefix apps/web

format:
  cargo fmt --all
  npm run format --prefix apps/web

check:
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace --all-features
  npm run check --prefix apps/web

e2e:
  npm run test:e2e --prefix apps/web

build:
  npm run build --prefix apps/web
  cargo build --workspace --release
