/*
 * Copyright 2024 Oxide Computer Company
 */

use std::collections::BTreeSet;

use anyhow::Result;

pub struct Output {
    use_defaults: bool,
    words: BTreeSet<String>,
}

impl Output {
    pub fn emit_default(cur: &str) -> Result<()> {
        println!("COMPREPLY=( $(compgen -o default -- '{}') )", cur);
        Ok(())
    }

    pub fn new() -> Output {
        Output { words: Default::default(), use_defaults: false }
    }

    pub fn add_word<S: ToString>(&mut self, word: S) {
        self.words.insert(word.to_string());
    }

    /**
     * If no words have been added as completion candidates, should we fall back
     * to the bash default behaviour to allow completing files and directories?
     */
    pub fn use_defaults(&mut self, use_defaults: bool) {
        self.use_defaults = use_defaults;
    }

    pub fn emit_eval(&self, cur: &str) -> Result<()> {
        if self.words.is_empty() {
            if self.use_defaults {
                /*
                 * Attempt to have bash fall back to the default completion
                 * behaviour from this point on.
                 */
                return Self::emit_default(cur);
            }

            /*
             * Don't print anything at all if we have no candidates to offer.
             */
            return Ok(());
        }

        let mut out = "COMPREPLY=(\n".to_string();
        for w in self.words.iter() {
            if !w.starts_with(cur) {
                continue;
            }
            out += "\t";
            out += &escape_string(w);
            out += "\n";
        }
        out += ")\n";

        println!("{out}");
        Ok(())
    }
}

/**
 * A crude attempt to emit a single-quoted string to the shell, where that
 * string might itself contain single quotes.
 */
fn escape_string(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\"'\"'"))
}
