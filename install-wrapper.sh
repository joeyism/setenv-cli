#!/bin/bash
# Automatic shell wrapper installer for setenv

set -e

echo "🔧 setenv Shell Wrapper Installer"
echo "================================="
echo ""

# Detect shell
SHELL_NAME=$(basename "$SHELL")

case "$SHELL_NAME" in
    bash)
        CONFIG_FILE="$HOME/.bashrc"
        WRAPPER='
# setenv shell wrapper
setenv() {
    if [ $# -eq 0 ] || [ "$1" = "list" ] || [ "$1" = "current" ] || [ "$1" = "edit" ] || [ "$1" = "new" ] || [ "$1" = "add" ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        command setenv "$@"
    else
        eval "$(command setenv "$@")"
    fi
}'
        ;;
    zsh)
        CONFIG_FILE="$HOME/.zshrc"
        WRAPPER='
# setenv shell wrapper
setenv() {
    if [ $# -eq 0 ] || [ "$1" = "list" ] || [ "$1" = "current" ] || [ "$1" = "edit" ] || [ "$1" = "new" ] || [ "$1" = "add" ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        command setenv "$@"
    else
        eval "$(command setenv "$@")"
    fi
}'
        ;;
    fish)
        CONFIG_FILE="$HOME/.config/fish/config.fish"
        WRAPPER='
# setenv shell wrapper
function setenv
    set -l cmd $argv[1]
    if test (count $argv) -eq 0; or test "$cmd" = "list"; or test "$cmd" = "current"; or test "$cmd" = "edit"; or test "$cmd" = "new"; or test "$cmd" = "add"; or test "$cmd" = "help"; or test "$cmd" = "--help"; or test "$cmd" = "-h"
        command setenv $argv
    else
        eval (command setenv $argv)
    end
end'
        ;;
    *)
        echo "❌ Unsupported shell: $SHELL_NAME"
        echo "Supported shells: bash, zsh, fish"
        exit 1
        ;;
esac

echo "Detected shell: $SHELL_NAME"
echo "Config file: $CONFIG_FILE"
echo ""

# Check if wrapper already exists
if grep -q "# setenv shell wrapper" "$CONFIG_FILE" 2>/dev/null; then
    echo "✅ Shell wrapper already installed in $CONFIG_FILE"
    echo ""
    echo "To reload your shell config, run:"
    echo "  source $CONFIG_FILE"
    exit 0
fi

# Add wrapper
echo "Adding wrapper function to $CONFIG_FILE..."
echo "$WRAPPER" >> "$CONFIG_FILE"

echo ""
echo "✅ Installation complete!"
echo ""
echo "To use setenv now, run:"
echo "  source $CONFIG_FILE"
echo ""
echo "Or restart your terminal."
