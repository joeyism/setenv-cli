setenv() {
    if [ "$1" = "list" ] || [ "$1" = "current" ] || [ "$1" = "edit" ]; then
        command setenv "$@"
    else
        eval "$(command setenv "$@")"
    fi
}
