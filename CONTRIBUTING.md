# Contributing to Local Voice Type

Thanks for your interest in contributing!

## How to Contribute

### Reporting Bugs

- Open an issue describing the bug
- Include your OS, client version, and steps to reproduce

### Feature Requests

- Open an issue describing the feature
- Explain the use case

### Pull Requests

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Test on your platform
5. Commit with clear messages
6. Push and open a PR

### Code Style

**Rust (client):**
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings

**Python (server):**
- Follow PEP 8
- Keep it simple

## Development Setup

### Client

```bash
cd client
cargo build
cargo run
```

### Server

```bash
cd server
docker compose up
```

## Questions?

Open an issue - happy to help!