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
