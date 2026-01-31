setenv() {
    if [ $# -eq 0 ] || [ "$1" = "list" ] || [ "$1" = "current" ] || [ "$1" = "edit" ] || [ "$1" = "new" ] || [ "$1" = "add" ] || [ "$1" = "help" ] || [ "$1" = "--help" ] || [ "$1" = "-h" ]; then
        command setenv "$@"
    else
        eval "$(command setenv "$@")"
    fi
}
