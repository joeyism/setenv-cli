# Publishing Guide

This guide explains how to publish releases of setenv-cli.

## Prerequisites

### GitHub Repository Secrets

Set up `CARGO_TOKEN` secret in GitHub (Settings → Secrets and variables → Actions):

1. Go to [crates.io](https://crates.io/) and sign in with GitHub
2. Go to [Account Settings → API Tokens](https://crates.io/settings/tokens)
3. Create new token named "GitHub Actions"
4. Copy the token
5. In GitHub repo: Settings → Secrets → New repository secret
   - Name: `CARGO_TOKEN`
   - Value: [paste token]

## Creating a Release

### 1. Update Version

Edit `Cargo.toml`:
```toml
version = "0.1.0"  # Increment: patch (0.1.1), minor (0.2.0), or major (1.0.0)
```

### 2. Update CHANGELOG.md

Move unreleased changes to new version section with today's date.

### 3. Test Everything

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt -- --check
cargo build --release
cargo publish --dry-run
```

### 4. Commit and Tag

```bash
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.1.0"
git push

git tag v0.1.0
git push origin v0.1.0
```

### 5. GitHub Actions Runs Automatically

When you push the tag, GitHub Actions will:
- ✅ Run tests on Linux, macOS, Windows
- ✅ Build binaries for all platforms
- ✅ Create GitHub release with binaries
- ✅ Publish to crates.io

## Versioning (Semantic Versioning)

- **PATCH** (0.0.x): Bug fixes only
- **MINOR** (0.x.0): New features, backwards compatible
- **MAJOR** (x.0.0): Breaking changes

## Release Checklist

Before tagging:

- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG.md updated with date
- [ ] `cargo test` passes
- [ ] `cargo clippy` passes
- [ ] `cargo fmt` applied
- [ ] `cargo publish --dry-run` succeeds
- [ ] Changes committed and pushed

## Manual Publishing (if needed)

```bash
cargo login <your-token>
cargo publish
```

## Troubleshooting

### "Authentication failure"
- Check CARGO_TOKEN is set in GitHub secrets
- Token must be valid and not expired

### "Release already exists"
```bash
git tag -d v0.1.0
git push origin :refs/tags/v0.1.0
# Then recreate
```

## Post-Release

1. Check release on GitHub: https://github.com/joeyism/setenv-cli/releases
2. Verify on crates.io: https://crates.io/crates/setenv-cli
3. Test installation: `cargo install setenv-cli`
