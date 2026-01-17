# Contributing to Truthlinked SDK

Thank you for your interest in contributing to the Truthlinked SDK! 

## Getting Started

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/YOUR_USERNAME/truthlinked-sdk.git`
3. **Install Rust**: https://rustup.rs/
4. **Run tests**: `cargo test`

## Development

### Prerequisites
- Rust 1.70+ (latest stable)
- A Truthlinked license key for testing

### Running Tests
```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

### Code Style
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Follow Rust naming conventions

## Pull Request Process

1. **Create a branch**: `git checkout -b feature/your-feature`
2. **Make changes** with tests
3. **Run tests**: `cargo test`
4. **Format code**: `cargo fmt`
5. **Check lints**: `cargo clippy`
6. **Commit**: Use clear, descriptive commit messages
7. **Push**: `git push origin feature/your-feature`
8. **Create PR** with description of changes

## Reporting Issues

- Use GitHub Issues: https://github.com/truth-linked/truthlinked-sdk/issues
- Include Rust version, OS, and error messages
- Provide minimal reproduction case
- **Never include license keys** in issues

## Security

For security issues, email: security@truthlinked.org

## License

By contributing, you agree your contributions will be licensed under MIT OR Apache-2.0.
