//! TUI integration tests for git-su.
//!
//! - `tui_no_menu_when_stdin_not_tty`: runs without PTY; verifies no interactive menu when stdin is not a TTY.
//! - Other tests use expectrl + PTY to simulate key input; they are `#[ignore]` by default (PTY may fail in sandbox/CI).
//!   Run them with: `cargo test --test tui_tests -- --ignored`

use std::process::Command;
use std::sync::Once;
use expectrl::{Expect, Eof, Regex};

static BUILD: Once = Once::new();

fn bin() -> std::path::PathBuf {
    BUILD.call_once(|| {
        let status = Command::new("cargo")
            .args(["build"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("cargo build");
        assert!(status.success(), "cargo build failed");
    });
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/debug/git-su")
}

/// When stdin is not a TTY, no menu is shown; only current user is printed.
#[test]
fn tui_no_menu_when_stdin_not_tty() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path();
    std::fs::write(home.join(".git-su"), "").unwrap();
    let out = Command::new(bin())
        .env("HOME", home)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .expect("run git-su");
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success());
    assert!(
        stdout.contains("Current user") || stdout.contains("Local") || stdout.contains("Global"),
        "should print current user when not TTY: {}",
        stdout
    );
    assert!(
        !stdout.contains("Select user"),
        "should not show Select menu when stdin is not TTY"
    );
}

/// Requires a real PTY (e.g. run with: cargo test --test tui_tests -- --ignored)
#[test]
#[ignore = "PTY spawn may fail in sandbox/CI; run with -- --ignored when PTY available"]
fn tui_shows_current_user_and_select_prompt() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path();
    std::fs::write(home.join(".git-su"), "").unwrap();
    let mut cmd = Command::new(bin());
    cmd.env("HOME", home);
    let mut p = expectrl::Session::spawn(cmd).expect("spawn git-su");
    p.set_expect_timeout(Some(std::time::Duration::from_secs(3)));
    p.expect(Regex("Current user|Local|Global|System"))
        .expect("should show current user or scopes");
    p.expect(Regex("Select user"))
        .expect("should show Select user prompt");
    // Empty list: only "(Add user)"; cancel with Escape to exit
    p.send("\x1b").unwrap();
    let _ = p.expect(Eof);
}

#[test]
#[ignore = "PTY spawn may fail in sandbox/CI; run with -- --ignored when PTY available"]
fn tui_select_first_user_switches() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path();
    std::fs::write(
        home.join(".git-su"),
        r#"[[user]]
name = "Jane Doe"
email = "jane@example.com"
"#,
    )
    .unwrap();
    let mut cmd = Command::new(bin());
    cmd.env("HOME", home);
    let mut p = expectrl::Session::spawn(cmd).expect("spawn git-su");
    p.set_expect_timeout(Some(std::time::Duration::from_secs(5)));
    p.expect(Regex("Select user")).expect("Select user prompt");
    // Index 0 is first user (no "Show current only" option)
    p.send_line("").unwrap();
    p.expect(Regex("Switched.*Jane Doe|jane@example.com"))
        .expect("should print Switched to user");
    let _ = p.expect(Eof);
}

#[test]
#[ignore = "PTY spawn may fail in sandbox/CI; run with -- --ignored when PTY available"]
fn tui_add_user_flow() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path();
    std::fs::write(home.join(".git-su"), "").unwrap();
    let mut cmd = Command::new(bin());
    cmd.env("HOME", home);
    let mut p = expectrl::Session::spawn(cmd).expect("spawn git-su");
    p.set_expect_timeout(Some(std::time::Duration::from_secs(8)));
    p.expect(Regex("Select user")).expect("Select user prompt");
    // Empty list: only "(Add user)" at index 0
    p.send_line("").unwrap();
    p.expect(Regex("(?i)name")).expect("Name prompt");
    p.send_line("Bob Smith").unwrap();
    p.expect(Regex("(?i)email")).expect("Email prompt");
    p.send_line("bob@example.com").unwrap();
    p.expect(Regex("added to users")).expect("added to users");
    let _ = p.expect(Eof);
}
