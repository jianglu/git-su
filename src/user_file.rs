// UserFile: ~/.git-su storage, TOML format

use std::fs;
use std::path::Path;

use crate::user::User;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct UserEntry {
    name: String,
    email: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Config {
    user: Vec<UserEntry>,
}

pub struct UserFile {
    path: std::path::PathBuf,
}

impl UserFile {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            let _ = fs::write(&path, "");
        }
        UserFile { path }
    }

    pub fn write(&self, user: &User) -> std::io::Result<()> {
        let mut config = self.read_config();
        config.user.push(UserEntry {
            name: user.name(),
            email: user.email(),
        });
        self.write_config(&config)
    }

    pub fn read(&self) -> Vec<User> {
        self.read_config()
            .user
            .into_iter()
            .map(|e| User::new(e.name, e.email))
            .collect()
    }

    fn read_config(&self) -> Config {
        let s = match fs::read_to_string(&self.path) {
            Ok(s) => s,
            Err(_) => return Config { user: vec![] },
        };
        toml::from_str(&s).unwrap_or(Config { user: vec![] })
    }

    fn write_config(&self, config: &Config) -> std::io::Result<()> {
        let s = toml::to_string_pretty(config).unwrap_or_default();
        fs::write(&self.path, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_empty_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".git-su");
        fs::write(&path, "").unwrap();
        let uf = UserFile::new(&path);
        assert!(uf.read().is_empty());
    }

    #[test]
    fn write_and_read_toml() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".git-su");
        let uf = UserFile::new(&path);
        let u = User::new("Jane Doe", "jane@example.com");
        uf.write(&u).unwrap();
        let list = uf.read();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name(), "Jane Doe");
        assert_eq!(list[0].email(), "jane@example.com");
    }

    #[test]
    fn read_valid_toml() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".git-su");
        let toml = r#"[[user]]
name = "Bob"
email = "bob@example.com"
"#;
        fs::write(&path, toml).unwrap();
        let uf = UserFile::new(&path);
        let list = uf.read();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name(), "Bob");
        assert_eq!(list[0].email(), "bob@example.com");
    }
}
