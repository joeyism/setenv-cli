# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-31

### Added
- Initial release
- Profile-based environment variable management
- Clean profile switching (unset old vars before setting new ones)
- Multi-shell support (bash, zsh, fish)
- Shell wrappers for seamless integration
- Commands: `setenv <profile>`, `setenv list`, `setenv current`, `setenv edit`
- TOML-based configuration at `~/.setenv/config.toml`
- Automatic config creation with default profile
- Variable name validation and sanitization
- Reserved name protection (SETENV_VARS, SETENV_PROFILE)
- Security: Command injection prevention through var name validation
- Comprehensive test suite
- GitHub Actions CI/CD pipeline
- Multi-platform binary releases (Linux, macOS, Windows)

[Unreleased]: https://github.com/joeyism/setenv-cli/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/joeyism/setenv-cli/releases/tag/v0.1.0
