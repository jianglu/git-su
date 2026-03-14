// ConfigRepository: gitsu.defaultSelectScope, gitsu.groupEmailAddress

use crate::git::{Git, Scope};

pub struct ConfigRepository;

impl ConfigRepository {
    pub fn default_select_scope() -> Scope {
        let v = Git::get_config(Scope::Derived, "gitsu.defaultSelectScope");
        let v = v.trim();
        if v.is_empty() {
            return Scope::Local;
        }
        match v {
            "local" => Scope::Local,
            "global" => Scope::Global,
            "system" => Scope::System,
            _ => Scope::Local,
        }
    }

    pub fn group_email_address() -> String {
        let v = Git::get_config(Scope::Derived, "gitsu.groupEmailAddress");
        let v = v.trim();
        if v.is_empty() {
            "dev@example.com".to_string()
        } else {
            v.to_string()
        }
    }
}
