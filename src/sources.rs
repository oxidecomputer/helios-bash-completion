/*
 * Copyright 2024 Oxide Computer Company
 */

use std::process::Command;

use anyhow::Result;

const ZONEADM: &str = "/usr/sbin/zoneadm";

pub fn test_overrides(name: &str) -> Option<Vec<String>> {
    if let Ok(v) = std::env::var(name) {
        Some(v.split_whitespace().map(str::to_string).collect())
    } else {
        None
    }
}

pub fn zonenames() -> Result<Vec<String>> {
    if let Some(v) = test_overrides("COMPLETE_TEST_ZONENAMES") {
        return Ok(v);
    }

    let out =
        Command::new(ZONEADM).env_clear().arg("list").arg("-cin").output()?;

    if !out.status.success() {
        return Ok(Default::default());
    }

    if let Ok(v) = String::from_utf8(out.stdout) {
        Ok(v.lines().map(str::to_string).collect())
    } else {
        Ok(Default::default())
    }
}
