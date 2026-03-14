// Git: config get/set/unset, scopes local/global/system

use crate::shell;
use crate::user::User;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    Local,
    Global,
    System,
    Derived,
    Default,
}

impl Scope {
    pub fn as_flag(self) -> &'static str {
        match self {
            Scope::Local => "local",
            Scope::Global => "global",
            Scope::System => "system",
            Scope::Derived | Scope::Default => "",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Scope::Local => "local",
            Scope::Global => "global",
            Scope::System => "system",
            Scope::Derived => "derived",
            Scope::Default => "local",
        }
    }
}

pub struct Git;

impl Git {
    pub fn get_config(scope: Scope, key: &str) -> String {
        let suffix = format!("--get {}", key);
        let cmd = if scope == Scope::Derived {
            format!("git config {}", suffix)
        } else {
            format!("git config --{} {}", scope.as_flag(), suffix)
        };
        shell::capture(&cmd)
    }

    pub fn set_config(scope: Scope, key: &str, value: &str) -> bool {
        let escaped = value.replace("'", "'\\''");
        let suffix = format!("{} '{}'", key, escaped);
        let cmd = if scope == Scope::Derived {
            format!("git config {}", suffix)
        } else {
            format!("git config --{} {}", scope.as_flag(), suffix)
        };
        shell::execute(&cmd)
    }

    pub fn unset_config(scope: Scope, key: &str) -> bool {
        let suffix = format!("--unset {}", key);
        let cmd = if scope == Scope::Derived {
            format!("git config {}", suffix)
        } else {
            format!("git config --{} {}", scope.as_flag(), suffix)
        };
        shell::execute(&cmd)
    }

    pub fn list_config(scope: Scope) -> Vec<String> {
        let suffix = "--list";
        let cmd = if scope == Scope::Derived {
            format!("git config {}", suffix)
        } else {
            format!("git config --{} {}", scope.as_flag(), suffix)
        };
        let s = shell::capture(&cmd);
        if s.is_empty() {
            return vec![];
        }
        s.lines().map(str::to_string).collect()
    }

    pub fn remove_section(scope: Scope, section: &str) -> bool {
        let suffix = format!("--remove-section {} 2>/dev/null", section);
        let cmd = if scope == Scope::Derived {
            format!("git config {}", suffix)
        } else {
            format!("git config --{} {}", scope.as_flag(), suffix)
        };
        shell::execute(&cmd)
    }

    pub fn select_user(user: &User, scope: Scope) -> bool {
        Self::set_config(scope, "user.name", &user.name())
            && Self::set_config(scope, "user.email", &user.email())
    }

    pub fn selected_user(scope: Scope) -> User {
        let name = Self::get_config(scope, "user.name");
        if name.is_empty() {
            return User::none();
        }
        let email = Self::get_config(scope, "user.email");
        User::new(name, email)
    }

    pub fn clear_user(scope: Scope) {
        let current = Self::selected_user(scope);
        if current.is_none() {
            return;
        }
        Self::unset_config(scope, "user.name");
        Self::unset_config(scope, "user.email");
        let list = Self::list_config(scope);
        if list.iter().filter(|e| e.starts_with("user.")).count() == 0 {
            Self::remove_section(scope, "user");
        }
    }

    pub fn edit_gitsu_config(path: &std::path::Path) -> bool {
        let path_str = path.to_string_lossy();
        let cmd = format!("git config --edit --file {}", path_str);
        shell::execute(&cmd)
    }

    pub fn get_color(name: &str) -> String {
        let suffix = format!("--get-color '' '{}'", name);
        let cmd = format!("git config {}", suffix);
        shell::capture(&cmd)
    }

    pub fn color_output() -> bool {
        let cmd = "git config --get-colorbool color.ui";
        let v = shell::capture(cmd);
        v == "true" || v == "always"
    }

    pub fn render(user: &User) -> String {
        if Self::color_output() {
            let user_color = Self::get_color("blue");
            let email_color = Self::get_color("green");
            let reset = Self::get_color("reset");
            format!(
                "{}{} {}<{}>{}",
                user_color,
                user.name(),
                email_color,
                user.email(),
                reset
            )
        } else {
            user.to_string()
        }
    }
}
