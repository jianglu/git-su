// UserList: find users by initials, name segment, etc.

use crate::user::User;
use crate::user_file::UserFile;

pub struct UserList {
    user_file: UserFile,
}

impl UserList {
    pub fn new(user_file: UserFile) -> Self {
        UserList { user_file }
    }

    pub fn add(&self, user: &User) {
        let _ = self.user_file.write(user);
    }

    pub fn list(&self) -> Vec<User> {
        self.user_file.read()
    }

    /// Find a unique combination of users matching the given search terms.
    pub fn find(&self, search_terms: &[String]) -> Result<Vec<User>, String> {
        let all = self.list();
        if search_terms.is_empty() {
            return Ok(vec![]);
        }
        let mut matches_per_term: Vec<Vec<User>> = Vec::new();
        for term in search_terms {
            let m = Self::matching_users(&all, term);
            if m.is_empty() {
                return Err(format!("No user found matching '{}'", term));
            }
            matches_per_term.push(m);
        }
        Self::unique_combination(&matches_per_term)
            .ok_or_else(|| {
                format!(
                    "Couldn't find a combination of users matching {}",
                    search_terms
                        .iter()
                        .map(|s| format!("'{}'", s))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            })
    }

    fn matching_users(all: &[User], search_term: &str) -> Vec<User> {
        let term_lower = search_term.to_lowercase();
        let mut result: Vec<User> = all
            .iter()
            .filter(|u| {
                // Whole word of name
                u.name()
                    .split_whitespace()
                    .any(|w| w.to_lowercase() == term_lower)
                    || u.name().to_lowercase().split_whitespace().any(|w| {
                        w.starts_with(&term_lower) || term_lower.starts_with(&w.to_lowercase())
                    })
                    || u.initials().contains(&term_lower)
                    || u.name().to_lowercase().contains(&term_lower)
                    || u.email().to_lowercase().contains(&term_lower)
            })
            .cloned()
            .collect();
        result.dedup();
        result
    }

    fn unique_combination(term_matches: &[Vec<User>]) -> Option<Vec<User>> {
        let mut combinations: Vec<Vec<User>> = vec![vec![]];
        for matches in term_matches {
            let mut new_combos = Vec::new();
            for combo in &combinations {
                for u in matches {
                    if !combo.iter().any(|c| c == u) {
                        let mut extended = combo.clone();
                        extended.push(u.clone());
                        new_combos.push(extended);
                    }
                }
            }
            combinations = new_combos;
        }
        combinations.into_iter().find(|c| c.len() == term_matches.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_file::UserFile;
    use std::fs;

    fn list_with_users(users: &[(&str, &str)]) -> (UserList, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".git-su");
        fs::write(&path, "").unwrap();
        let uf = UserFile::new(&path);
        for (name, email) in users {
            uf.write(&User::new(*name, *email)).unwrap();
        }
        (UserList::new(uf), dir)
    }

    #[test]
    fn find_by_initials() {
        let (list, _dir) = list_with_users(&[
            ("Jane Doe", "jane@example.com"),
            ("Bob Smith", "bob@example.com"),
        ]);
        let found = list.find(&["jd".to_string()]).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name(), "Jane Doe");
    }

    #[test]
    fn find_by_name_part() {
        let (list, _dir) = list_with_users(&[("Alice Cooper", "alice@example.com")]);
        let found = list.find(&["alice".to_string()]).unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].name(), "Alice Cooper");
    }

    #[test]
    fn find_no_match() {
        let (list, _dir) = list_with_users(&[("Jane Doe", "jane@example.com")]);
        let r = list.find(&["xyz".to_string()]);
        assert!(r.is_err());
    }

    #[test]
    fn find_pair_two_terms() {
        let (list, _dir) = list_with_users(&[
            ("Jane Doe", "jane@example.com"),
            ("Bob Smith", "bob@example.com"),
        ]);
        let found = list.find(&["jd".to_string(), "bob".to_string()]).unwrap();
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].name(), "Jane Doe");
        assert_eq!(found[1].name(), "Bob Smith");
    }

    #[test]
    fn list_returns_added_users() {
        let (list, _dir) = list_with_users(&[("Jane Doe", "jane@example.com")]);
        let all = list.list();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].email(), "jane@example.com");
    }
}
