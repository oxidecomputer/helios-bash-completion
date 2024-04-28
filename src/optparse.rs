/*
 * Copyright 2024 Oxide Computer Company
 */

pub struct PartialGetoptResult<'a> {
    pub prev: &'a str,
    pub cur: &'a str,
    pub free_start: usize,
}

pub fn partial_getopt_parser<'a>(
    cword: usize,
    words: &[&'a String],
    needs_opt_arg: &[char],
) -> Option<PartialGetoptResult<'a>> {
    if words.len() < 2 || cword < 1 {
        /*
         * We want there to be at least two words (the command name and at least
         * an empty argument to tab-complete) and we want the word we are
         * completing for to be anything but the command name word.
         */
        return None;
    }
    let cur = words[cword].as_str();
    let prev = words[cword - 1].as_str();

    /*
     * Attempt to locate where options end and free arguments begin.
     */
    let mut free_start = 1;
    let mut need_arg = false;
    loop {
        if free_start >= words.len() {
            break;
        }

        if words[free_start].starts_with('-') {
            /*
             * This is an option of some kind.
             */
            if words[free_start].starts_with("--") {
                /*
                 * All done with options processing after this word.
                 */
                free_start += 1;
                break;
            }

            if words[free_start]
                .chars()
                .skip(1)
                .any(|c| needs_opt_arg.contains(&c))
            {
                /*
                 * This option requires an option argument, so free arguments
                 * cannot begin in the next word.
                 */
                need_arg = true;
            }
        } else if need_arg {
            /*
             * Absorb this argument as the option argument for the previous
             * option.
             */
            need_arg = false;
        } else if !words[free_start].is_empty() {
            /*
             * This is the first free argument.
             */
            break;
        }

        free_start += 1;
    }

    Some(PartialGetoptResult { prev, cur, free_start })
}
