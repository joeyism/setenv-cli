# Contributing to setenv

Thank you for considering contributing to setenv! This document provides guidelines and instructions for contributing.

## Development Setup

### Prerequisites

- Rust (stable channel)
- Git

### Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/joeyism/setenv-cli.git
   cd setenv-cli
   ```
3. Create a branch:
   ```bash
   git checkout -b feature/my-feature
   ```

## Development Workflow

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Locally

```bash
cargo run -- <command>
```

Example:
```bash
cargo run -- list
```

### Code Quality

Before submitting a PR, ensure:

1. **Code is formatted**:
   ```bash
   cargo fmt
   ```

2. **No clippy warnings**:
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Tests pass**:
   ```bash
   cargo test
   ```

4. **Security audit passes**:
   ```bash
   cargo install cargo-audit
   cargo audit
   ```

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Update CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format
3. Add tests for new functionality
4. Ensure all CI checks pass
5. Request review from maintainers

## Commit Messages

Follow conventional commits format:

- `feat: add new feature`
- `fix: fix bug in X`
- `docs: update documentation`
- `test: add tests for X`
- `refactor: refactor X`
- `chore: update dependencies`

## Code Style

- Follow Rust standard style guidelines (enforced by `cargo fmt`)
- Keep functions small and focused
- Add comments for complex logic
- Write descriptive variable names

## Testing

- Write unit tests for new functionality
- Ensure existing tests pass
- Test manually with different shells (bash, zsh, fish)

## Reporting Issues

When reporting issues, please include:

- Operating system and version
- Shell type and version
- setenv version
- Steps to reproduce
- Expected behavior
- Actual behavior
- Error messages (if any)

## Feature Requests

Feature requests are welcome! Please:

- Check if the feature has already been requested
- Provide clear use cases
- Explain why it would benefit users

## Questions?

Feel free to open an issue for questions or join discussions.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
