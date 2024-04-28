/*
 * Copyright 2024 Oxide Computer Company
 */

use anyhow::{anyhow, bail, Result};

mod optparse;
mod shell;
mod sources;
use optparse::partial_getopt_parser;
use shell::Output;

struct Completion {
    cmd: &'static str,
    func: fn(usize, &[&String]) -> Result<()>,
}

const COMPLETIONS: &[Completion] = &[
    Completion { cmd: "zlogin", func: complete_zlogin },
    /*
     * Complete the names of running zones after a "-z" option for a variety
     * of base illumos commands:
     */
    Completion { cmd: "allocate", func: complete_dash_z },
    Completion { cmd: "deallocate", func: complete_dash_z },
    Completion { cmd: "ipfs", func: complete_dash_z },
    Completion { cmd: "ipfstat", func: complete_dash_z },
    Completion { cmd: "ipmon", func: complete_dash_z },
    Completion { cmd: "ipnat", func: complete_dash_z },
    Completion { cmd: "ippool", func: complete_dash_z },
    Completion { cmd: "pgrep", func: complete_dash_z },
    Completion { cmd: "pkill", func: complete_dash_z },
    Completion { cmd: "ps", func: complete_dash_z },
    Completion { cmd: "psrset", func: complete_dash_z },
    Completion { cmd: "ptree", func: complete_dash_z },
    Completion { cmd: "svccfg", func: complete_dash_z },
    Completion { cmd: "svcadm", func: complete_dash_z },
    Completion { cmd: "svcprop", func: complete_dash_z },
    Completion { cmd: "svcs", func: complete_dash_z },
    Completion { cmd: "zoneadm", func: complete_dash_z },
    Completion { cmd: "zonecfg", func: complete_dash_z },
];

fn main() -> Result<()> {
    sigpipe::reset();

    let opts = getopts::Options::new()
        .optflag("g", "", "generate completions")
        .parsing_style(getopts::ParsingStyle::StopAtFirstFree)
        .parse(std::env::args_os().skip(1))?;

    if opts.opt_present("g") {
        /*
         * Generate a shell invocation that will use this program for tab
         * completion.  It should be invoked from bash (e.g., in .bashrc) as:
         *
         *      eval $(/usr/lib/helios-bash-completion -g)
         */
        let prog = std::env::current_exe()?;
        let prog =
            prog.to_str().ok_or_else(|| anyhow!("invalid executable path"))?;

        println!("function _helios_bash_completion {{");
        println!("\teval $('{prog}' \"$COMP_CWORD\" \"${{COMP_WORDS[@]}}\");");
        println!("}};");
        println!();

        for cf in COMPLETIONS {
            println!("complete -F _helios_bash_completion \'{}\';", cf.cmd);
        }
        println!();
        return Ok(());
    }

    /*
     * Our generated shell function will pass us the COMP_CWORD index and the
     * entire COMP_WORDS array as positional arguments.  Even when completing
     * from an empty prompt, bash appears to give us at least one zero-length
     * word each time.
     */
    let cword = opts
        .free
        .first()
        .ok_or_else(|| anyhow!("missing COMP_CWORD argument"))?
        .parse()
        .map_err(|e| anyhow!("invalid COMP_CWORD argument: {e}"))?;

    let words = opts.free.iter().skip(1).collect::<Vec<_>>();
    if cword >= words.len() {
        bail!("COMP_CWORD {cword} is out of range ({} words!)", words.len());
    }

    let Some(cmd) = words.first() else {
        return Ok(());
    };

    for cf in COMPLETIONS {
        if cf.cmd == cmd.as_str() {
            return (cf.func)(cword, &words);
        }
    }

    /*
     * We don't expect to be called for a command we do not recognise, but if it
     * occurs we'll just ask bash to fall back to the default behaviour:
     */
    Output::emit_default(words[cword].as_str())
}

fn complete_dash_z(cword: usize, words: &[&String]) -> Result<()> {
    let Some(pgr) = partial_getopt_parser(cword, words, &['z']) else {
        return Ok(());
    };

    let mut out = Output::new();
    if cword < pgr.free_start && pgr.prev == "-z" {
        for z in sources::zonenames()? {
            out.add_word(z);
        }
    } else {
        out.use_defaults(true);
    }
    out.emit_eval(pgr.cur)
}

fn complete_zlogin(cword: usize, words: &[&String]) -> Result<()> {
    let Some(pgr) = partial_getopt_parser(cword, words, &['e', 'l']) else {
        return Ok(());
    };

    if pgr.prev == "-l" {
        /*
         * Complete usernames for -l, even though passwd(5) is disjoint between
         * the global and non-global zone.  It's the best we can do without more
         * complex chicanery.
         */
        println!("COMPREPLY=( $(compgen -u -- '{}') )", pgr.cur);
        return Ok(());
    }

    let mut out = Output::new();
    if pgr.prev == "-e" {
        /*
         * Don't complete anything as the option argument to the escape
         * character option.
         */
    } else if cword <= pgr.free_start {
        if cword < pgr.free_start {
            for w in ["-E", "-Q", "-e", "-l", "-d", "-C", "-n", "-S"] {
                out.add_word(w);
            }
        }
        for z in sources::zonenames()? {
            out.add_word(z);
        }
    } else {
        out.use_defaults(true);
    }
    out.emit_eval(pgr.cur)
}
