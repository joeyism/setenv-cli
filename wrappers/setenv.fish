function setenv
    set -l cmd $argv[1]
    if test (count $argv) -eq 0; or test "$cmd" = "list"; or test "$cmd" = "current"; or test "$cmd" = "edit"; or test "$cmd" = "new"; or test "$cmd" = "add"; or test "$cmd" = "help"; or test "$cmd" = "--help"; or test "$cmd" = "-h"
        command setenv $argv
    else
        eval (command setenv $argv)
    end
end
