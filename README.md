# helios-bash-completion

This program is intended to ship as `/usr/lib/helios-bash-completion` on a
Helios system.  It will generate bash shell source that should be evaluated
from within `.bashrc` to enable the use of bash tab completion for some system
commands; e.g.,

```
$ /usr/lib/helios-bash-completion -g
function _helios_bash_completion {
    eval $('/usr/lib/helios-bash-completion' "$COMP_CWORD" "${COMP_WORDS[@]}");
};

complete -F _helios_bash_completion 'zlogin';
complete -F _helios_bash_completion 'allocate';
complete -F _helios_bash_completion 'deallocate';
complete -F _helios_bash_completion 'ipfs';
complete -F _helios_bash_completion 'ipfstat';
complete -F _helios_bash_completion 'ipmon';
complete -F _helios_bash_completion 'ipnat';
complete -F _helios_bash_completion 'ippool';
complete -F _helios_bash_completion 'pgrep';
complete -F _helios_bash_completion 'pkill';
complete -F _helios_bash_completion 'ps';
complete -F _helios_bash_completion 'psrset';
complete -F _helios_bash_completion 'ptree';
complete -F _helios_bash_completion 'svccfg';
complete -F _helios_bash_completion 'svcadm';
complete -F _helios_bash_completion 'svcprop';
complete -F _helios_bash_completion 'svcs';
complete -F _helios_bash_completion 'zoneadm';
complete -F _helios_bash_completion 'zonecfg';

$ eval $(/usr/lib/helios-bash-completion -g)

$ complete
complete -F _helios_bash_completion deallocate
complete -F _helios_bash_completion allocate
complete -F _helios_bash_completion svcadm
complete -F _helios_bash_completion ipnat
complete -F _helios_bash_completion pgrep
complete -F _helios_bash_completion psrset
complete -F _helios_bash_completion pkill
complete -F _helios_bash_completion ippool
complete -F _helios_bash_completion zlogin
complete -F _helios_bash_completion zonecfg
complete -F _helios_bash_completion ipmon
complete -F _helios_bash_completion ipfstat
complete -F _helios_bash_completion svccfg
complete -F _helios_bash_completion svcs
complete -F _helios_bash_completion ipfs
complete -F _helios_bash_completion zoneadm
complete -F _helios_bash_completion ps
complete -F _helios_bash_completion svcprop
complete -F _helios_bash_completion ptree
```

The program attempts to be as unobtrusive as possible when it cannot guarantee
a useful match.  It is also somewhat context sensitive; i.e., it ideally will
not offer completions that don't make sense at the point where the user is
requesting them with the tab key.

## Testing

A limited facility for tinkering is available.  If you do not have any zones
available on your system, you can feed a set of test zone names to
the program through the environment.  For example:

```
$ export COMPLETE_TEST_ZONENAMES='samuel taylor coleridge'

$ ./target/release/helios-bash-completion 2 'zonecfg' '-z' ''
COMPREPLY=(
        'coleridge'
        'samuel'
        'taylor'
)

 $ ./target/release/helios-bash-completion 1 'zlogin' ''
COMPREPLY=(
        '-C'
        '-E'
        '-Q'
        '-S'
        '-d'
        '-e'
        '-l'
        '-n'
        'coleridge'
        'samuel'
        'taylor'
)
```

Bash appears to invoke the command for actual completion with the current
environment as well, so you can test interactively with the same overrides.

## Licence

Unless otherwise noted, all components are licenced under the [Mozilla Public
License Version 2.0](./LICENSE).
