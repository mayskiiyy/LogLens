# Contributing to LogLens

Thank you for helping improve LogLens!

## Prerequisites

- **Rust**: 1.80+ (`rustup toolchain install stable`)
- **Node.js**: 20+
- **Docker**: (Optional, for self-hosted integration testing)

## Local Development Setup

```bash
# Clone repository
git clone https://github.com/mayskiiyy/LogLens.git
cd LogLens

# Install dependencies
npm install

# Run tests
cargo test --workspace --all-features
npm run test --workspace=apps/web
```

## Code Quality Standards

Before submitting a pull request, ensure all checks pass:

```bash
# Check formatting
cargo fmt --all --check
npm run format --workspace=apps/web

# Run linter
cargo clippy --workspace --all-targets --all-features -- -D warnings
npm run lint --workspace=apps/web

# Run full test suite
just test
```

## Adding a New Log Parser

See [`docs/parser-development.md`](docs/parser-development.md) for a step-by-step tutorial on implementing the `Parser` trait in `loglens-core`.

## Security Reporting

Please do NOT create public GitHub issues for security vulnerabilities. Follow the reporting guidelines in [`SECURITY.md`](SECURITY.md).
