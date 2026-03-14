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
