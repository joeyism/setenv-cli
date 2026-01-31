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

## Quick Install

### Automatic (Recommended)

```bash
# 1. Install the binary
cargo install setenv-cli

# 2. Run the automatic wrapper installer
curl -fsSL https://raw.githubusercontent.com/joeyism/setenv-cli/master/install-wrapper.sh | bash

# 3. Reload your shell (or restart terminal)
source ~/.bashrc  # or ~/.zshrc for zsh
```

### Manual

**For bash/zsh users:**

```bash
# 1. Install the binary
cargo install setenv-cli

# 2. Add wrapper function to ~/.bashrc (or ~/.zshrc)
cat >> ~/.bashrc << 'EOF'

# setenv shell wrapper
setenv() {
    if [ $# -eq 0 ] || [ "$1" = "list" ] || [ "$1" = "current" ] || [ "$1" = "edit" ] || [ "$1" = "new" ] || [ "$1" = "add" ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        command setenv "$@"
    else
        eval "$(command setenv "$@")"
    fi
}
EOF

# 3. Reload your shell
source ~/.bashrc
```

Now you can use: `setenv <profile-name>`</thinking>



---

## Detailed Installation

### Step 1: Install the binary

#### Option A: Install from crates.io (Recommended)

If you have Rust installed:

```bash
cargo install setenv-cli
```

#### Option B: Download pre-built binary

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

#### Option C: Build from source

```bash
git clone https://github.com/joeyism/setenv-cli.git
cd setenv-cli
cargo build --release
sudo cp target/release/setenv /usr/local/bin/
```

### Step 2: Set up the shell wrapper (REQUIRED)

⚠️ **Important:** Without this step, `setenv` will only print commands but won't actually set variables.

The tool outputs shell commands that need to be evaluated in your current shell. Add this wrapper function to your shell config:

#### Bash

Add to your `~/.bashrc`:

```bash
source /path/to/setenv-cli/wrappers/setenv.bash
```

Or copy the function directly:

```bash
setenv() {
    if [ $# -eq 0 ] || [ "$1" = "list" ] || [ "$1" = "current" ] || [ "$1" = "edit" ] || [ "$1" = "new" ] || [ "$1" = "add" ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
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
    set -l cmd $argv[1]
    if test (count $argv) -eq 0; or test "$cmd" = "list"; or test "$cmd" = "current"; or test "$cmd" = "edit"; or test "$cmd" = "new"; or test "$cmd" = "add"; or test "$cmd" = "help"; or test "$cmd" = "--help"; or test "$cmd" = "-h"
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

> **⚠️ Before using:** Make sure you've completed both installation steps (binary + shell wrapper). If you only installed the binary, variables won't actually be set. See [Troubleshooting](#variables-not-persisting--only-seeing-export-commands) if you're seeing export commands instead of set variables.

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

### Create a new profile

```bash
setenv new <profile-name>
```

Creates a new empty profile. Example:

```bash
setenv new production
```

### Add a variable to a profile

```bash
setenv add <profile-name> <KEY> <VALUE>
```

Adds or updates a variable in an existing profile. Example:

```bash
setenv add production API_KEY "prod-api-key-123"
setenv add production DB_HOST "prod.database.com"
```

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

### Variables not persisting / Only seeing export commands

**Symptom:** When you run `setenv <profile>`, you see output like:
```bash
export API_KEY="value"
export DB_HOST="localhost"
...
```
But the variables aren't actually set in your shell.

**Cause:** You haven't set up the shell wrapper function (Step 2 of installation).

**Solution:** Add the wrapper function to your shell config and reload:

```bash
# For bash, add to ~/.bashrc:
setenv() {
    if [ $# -eq 0 ] || [ "$1" = "list" ] || [ "$1" = "current" ] || [ "$1" = "edit" ] || [ "$1" = "new" ] || [ "$1" = "add" ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        command setenv "$@"
    else
        eval "$(command setenv "$@")"
    fi
}

# Then reload
source ~/.bashrc
```

The wrapper is **required** because the binary can only output commands - it can't directly modify your shell's environment. The wrapper uses `eval` to execute those commands in your current shell.

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
