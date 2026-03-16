// Shell: run commands and capture output

use std::process::Command;

pub fn capture(cmd: &str) -> String {
    let out = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();
    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => String::new(),
    }
}

pub fn execute(cmd: &str) -> bool {
    Command::new("sh").arg("-c").arg(cmd).status().map(|s| s.success()).unwrap_or(false)
}

/// Run `git config [--scope] key value` with args as separate argv (no shell quoting).
/// Ensures HOME is inherited so --global writes to the same ~/.gitconfig as the current process.
pub fn git_config_set(scope_flag: Option<&str>, key: &str, value: &str) -> bool {
    let mut cmd = Command::new("git");
    if let Some(home) = std::env::var_os("HOME") {
        cmd.env("HOME", home);
    }
    cmd.arg("config");
    if let Some(flag) = scope_flag {
        cmd.arg(format!("--{}", flag));
    }
    cmd.arg(key).arg(value);
    cmd.status().map(|s| s.success()).unwrap_or(false)
}
