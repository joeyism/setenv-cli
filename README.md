# setenv

[![CI](https://github.com/joeyism/setenv-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/joeyism/setenv-cli/actions/workflows/ci.yml)
[![Release](https://github.com/joeyism/setenv-cli/actions/workflows/release.yml/badge.svg)](https://github.com/joeyism/setenv-cli/actions/workflows/release.yml)
[![Security Audit](https://github.com/joeyism/setenv-cli/actions/workflows/security.yml/badge.svg)](https://github.com/joeyism/setenv-cli/actions/workflows/security.yml)
[![Crates.io](https://img.shields.io/crates/v/setenv-cli.svg)](https://crates.io/crates/setenv-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A CLI tool for managing environment variable profiles with seamless shell integration.

## Features

- **Profile-based environment management**: Switch between different sets of environment variables with a single command
- **Clean profile switching**: Old profile variables are automatically unset when switching to a new profile
- **Multi-shell support**: Works with bash, zsh, and fish
- **Simple configuration**: TOML-based config file in `~/.setenv/config.toml`
- **No environment pollution**: Only manages variables it sets, leaves your other env vars untouched

## Installation

### Option 1: Install from crates.io (Recommended)

If you have Rust installed:

```bash
cargo install setenv-cli
```

### Option 2: Download pre-built binary

Download the latest release for your platform from the [releases page](https://github.com/joeyism/setenv-cli/releases):

- **Linux (x86_64)**: `setenv-linux-amd64`
- **Linux (musl)**: `setenv-linux-amd64-musl`
- **macOS (Intel)**: `setenv-macos-amd64`
- **macOS (Apple Silicon)**: `setenv-macos-arm64`
- **Windows**: `setenv-windows-amd64.exe`

Then move it to your PATH:

```bash
# Linux/macOS
sudo mv setenv-* /usr/local/bin/setenv
sudo chmod +x /usr/local/bin/setenv

# Windows
# Move setenv-windows-amd64.exe to a directory in your PATH
```

### Option 3: Build from source

```bash
git clone https://github.com/joeyism/setenv-cli.git
cd setenv-cli
cargo build --release
sudo cp target/release/setenv /usr/local/bin/
```

### Set up the shell wrapper

The tool requires a shell wrapper function to eval the output. Choose your shell:

#### Bash

Add to your `~/.bashrc`:

```bash
source /path/to/setenv-cli/wrappers/setenv.bash
```

Or copy the function directly:

```bash
setenv() {
    if [ "$1" = "list" ] || [ "$1" = "current" ] || [ "$1" = "edit" ]; then
        command setenv "$@"
    else
        eval "$(command setenv "$@")"
    fi
}
```

#### Zsh

Add to your `~/.zshrc` (same as bash):

```zsh
source /path/to/setenv-cli/wrappers/setenv.bash
```

#### Fish

Add to your `~/.config/fish/config.fish`:

```fish
source /path/to/setenv-cli/wrappers/setenv.fish
```

Or copy the function directly:

```fish
function setenv
    if test "$argv[1]" = "list"; or test "$argv[1]" = "current"; or test "$argv[1]" = "edit"
        command setenv $argv
    else
        eval (command setenv $argv)
    end
end
```

After adding the wrapper, reload your shell:

```bash
source ~/.bashrc    # for bash
source ~/.zshrc     # for zsh
source ~/.config/fish/config.fish  # for fish
```

## Configuration

The config file is located at `~/.setenv/config.toml`. It will be created automatically on first run with a default profile.

### Config Format

```toml
[profiles.default]
EXAMPLE_VAR = "example_value"
ANOTHER_VAR = "another_value"

[profiles.backend]
API_KEY = "backend-api-key"
DB_HOST = "backend.database.com"
DB_PORT = "5432"
DEBUG = "true"

[profiles.frontend]
API_KEY = "frontend-api-key"
API_URL = "https://api.example.com"
NODE_ENV = "development"
```

### Variable Name Rules

- Must start with a letter or underscore
- Can contain only letters, numbers, and underscores
- Cannot use reserved names: `SETENV_VARS`, `SETENV_PROFILE`

### Profile Name Rules

- Can contain alphanumeric characters, underscores, and dashes
- Cannot contain spaces or special characters

## Usage

### Switch to a profile

```bash
setenv backend
```

This will:
1. Unset all variables from the previous profile
2. Set all variables from the `backend` profile
3. Update `SETENV_PROFILE` and `SETENV_VARS` tracking variables

### List available profiles

```bash
setenv list
```

Output shows all profiles, with `*` marking the currently active one:

```
Available profiles:
  * backend
    default
    frontend
```

### Show current profile

```bash
setenv current
```

### Edit config file

```bash
setenv edit
```

Opens the config file in your `$EDITOR` (falls back to vim, vi, or nano).

## How It Works

### Clean Profile Switching

When you switch profiles, `setenv` ensures a clean transition:

1. **Tracks managed variables**: The tool stores which variables it manages in the `SETENV_VARS` environment variable
2. **Unsets old variables**: Before switching, all variables from the previous profile are unset
3. **Sets new variables**: Variables from the new profile are then exported
4. **Updates tracking**: `SETENV_PROFILE` and `SETENV_VARS` are updated

This ensures no variable "leakage" between profiles while leaving your other environment variables untouched.

### Example

Starting state:
```bash
$ echo $MY_VAR
my_value
```

Switch to backend profile:
```bash
$ setenv backend
$ echo $API_KEY
backend-api-key
$ echo $MY_VAR
my_value              # Your original vars are untouched
```

Switch to frontend profile:
```bash
$ setenv frontend
$ echo $API_KEY
frontend-api-key      # Changed to frontend value
$ echo $DB_HOST
                       # Backend's DB_HOST was unset
$ echo $MY_VAR
my_value              # Still untouched
```

## Troubleshooting

### "Command not found: setenv"

Make sure:
1. The binary is in your PATH
2. You've added the shell wrapper to your shell config
3. You've reloaded your shell configuration

### "Profile 'X' not found"

Check your config file:

```bash
setenv list    # See available profiles
setenv edit    # Edit config if needed
```

### Variables not persisting

Make sure you're using the shell wrapper function, not calling the binary directly. The wrapper uses `eval` to execute the commands in your current shell.

### Config file errors

If your config file has syntax errors:

```bash
setenv edit    # Fix the TOML syntax
```

Common issues:
- Missing quotes around values with spaces
- Invalid variable names (must start with letter/underscore)
- Using reserved names (`SETENV_VARS`, `SETENV_PROFILE`)

## Development

### Run tests

```bash
cargo test
```

### Build

```bash
cargo build
```

### Run without installing

```bash
cargo run -- <args>
```

Note: Without the shell wrapper, this will only print the commands, not execute them.

## License

MIT
