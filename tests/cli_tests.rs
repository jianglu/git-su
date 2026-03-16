// Integration tests: run git-su binary with temp HOME

use std::process::Command;
use std::sync::Once;

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
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/debug/git-su")
}

fn run_with_home(home: &std::path::Path, args: &[&str]) -> (bool, String, String) {
    let out = Command::new(bin())
        .args(args)
        .env("HOME", home)
        .output()
        .expect("run git-su");
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (out.status.success(), stdout, stderr)
}

#[test]
fn list_empty() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path();
    std::fs::write(home.join(".git-su"), "").unwrap();
    let (ok, stdout, _) = run_with_home(home, &["--list"]);
    assert!(ok, "git su --list should succeed");
    assert!(stdout.trim().is_empty(), "empty list should output nothing");
}

#[test]
fn add_and_list() {
    let dir = tempfile::tempdir().unwrap();
    let home = dir.path();
    std::fs::write(home.join(".git-su"), "").unwrap();
    let (ok1, _, _) = run_with_home(home, &["--add", "Jane Doe <jane@example.com>"]);
    assert!(ok1, "git su --add should succeed");
    let (ok2, stdout, _) = run_with_home(home, &["--list"]);
    assert!(ok2);
    assert!(
        stdout.contains("Jane Doe") && stdout.contains("jane@example.com"),
        "list should contain added user: {}",
        stdout
    );
}

#[test]
fn global_writes_gitconfig() {
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
    let (ok, _, stderr) = run_with_home(home, &["--global", "jd"]);
    assert!(ok, "git su --global jd should succeed: {}", stderr);
    let gitconfig = home.join(".gitconfig");
    assert!(gitconfig.exists(), "~/.gitconfig should be created");
    let out = Command::new("git")
        .args(["config", "--global", "--get", "user.name"])
        .env("HOME", home)
        .output()
        .expect("run git config");
    let name = String::from_utf8_lossy(&out.stdout).trim().to_string();
    assert_eq!(name, "Jane Doe", "global user.name should be set");
    let out2 = Command::new("git")
        .args(["config", "--global", "--get", "user.email"])
        .env("HOME", home)
        .output()
        .expect("run git config");
    let email = String::from_utf8_lossy(&out2.stdout).trim().to_string();
    assert_eq!(email, "jane@example.com", "global user.email should be set");
}

#[test]
fn help_and_version() {
    let dir = tempfile::tempdir().unwrap();
    let (ok_help, stdout_help, _) = run_with_home(dir.path(), &["--help"]);
    assert!(ok_help);
    assert!(stdout_help.contains("git-su") || stdout_help.contains("Manage"));

    let (ok_ver, stdout_ver, _) = run_with_home(dir.path(), &["--version"]);
    assert!(ok_ver);
    assert!(stdout_ver.contains("git-su"));
}
