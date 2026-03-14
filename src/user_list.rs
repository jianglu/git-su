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
