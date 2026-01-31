function setenv
    if test "$argv[1]" = "list"; or test "$argv[1]" = "current"; or test "$argv[1]" = "edit"
        command setenv $argv
    else
        eval (command setenv $argv)
    end
end
