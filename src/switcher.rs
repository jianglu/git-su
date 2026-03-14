// Switcher: request switch, print current, list, add, clear, edit

use crate::config_repository::ConfigRepository;
use crate::git::{Git, Scope};
use crate::user::User;
use crate::user_list::UserList;
use std::path::Path;

pub struct Switcher<'a> {
    user_list: &'a UserList,
}

impl<'a> Switcher<'a> {
    pub fn new(user_list: &'a UserList) -> Self {
        Switcher { user_list }
    }

    pub fn request(&self, scope: Scope, user_strings: &[String], output: &mut impl std::io::Write) {
        let (parsed, not_parsed): (Vec<User>, Vec<String>) = user_strings
            .iter()
            .map(|s| {
                User::parse(s)
                    .map(|u| (Some(u), None))
                    .unwrap_or_else(|_| (None, Some(s.clone())))
            })
            .fold(
                (vec![], vec![]),
                |(mut parsed, mut not_parsed), (p, n)| {
                    if let Some(u) = p {
                        parsed.push(u);
                    }
                    if let Some(s) = n {
                        not_parsed.push(s);
                    }
                    (parsed, not_parsed)
                },
            );

        let found = match self.user_list.find(&not_parsed) {
            Ok(users) => users,
            Err(e) => {
                let _ = writeln!(output, "{}", crate::color::error(&e.to_string()));
                return;
            }
        };

        let group_email = ConfigRepository::group_email_address();
        let all_users: Vec<User> = parsed
            .iter()
            .chain(found.iter())
            .cloned()
            .collect();
        let combined = all_users
            .iter()
            .fold(User::none(), |acc, u| acc.combine(u, &group_email));

        let actual_scope = if scope == Scope::Default || scope == Scope::Derived {
            ConfigRepository::default_select_scope()
        } else {
            scope
        };

        if Git::select_user(&combined, actual_scope) {
            let _ = writeln!(
                output,
                "{}",
                crate::color::success(&format!(
                    "Switched {} user to {}",
                    actual_scope.display_name(),
                    Git::render(&combined)
                ))
            );
        }

        for u in &parsed {
            if !u.is_none() && !self.user_list.list().iter().any(|l| l == u) {
                self.user_list.add(u);
            }
        }
    }

    pub fn print_current(&self, scopes: &[Scope], output: &mut impl std::io::Write) {
        const LABEL_WIDTH: usize = 6; // Local, Global, System
        let show_all = scopes.is_empty();
        if show_all {
            let _ = writeln!(
                output,
                "{} {}",
                crate::color::label("Current user:"),
                Git::render(&Git::selected_user(Scope::Derived))
            );
            let _ = writeln!(output);
            for (scope_label, scope) in [
                ("Local", Scope::Local),
                ("Global", Scope::Global),
                ("System", Scope::System),
            ] {
                let padded = format!("{:>1$}", scope_label, LABEL_WIDTH);
                let _ = writeln!(
                    output,
                    "{}: {}",
                    crate::color::label(&padded),
                    Git::render(&Git::selected_user(scope))
                );
            }
        } else {
            for scope in scopes {
                let label = scope.display_name();
                let w = label.len().max(LABEL_WIDTH);
                let u = Git::selected_user(*scope);
                if !u.is_none() {
                    let padded = format!("{:<1$}", label, w);
                    let _ = writeln!(
                        output,
                        "{}: {}",
                        crate::color::label(&padded),
                        Git::render(&u)
                    );
                }
            }
        }
    }

    pub fn clear(&self, scopes: &[Scope], output: &mut impl std::io::Write) {
        let scopes: Vec<Scope> = if scopes.is_empty() {
            vec![Scope::Local, Scope::Global, Scope::System]
        } else {
            scopes.to_vec()
        };
        let scope_names: Vec<&str> = scopes.iter().map(|s| s.as_flag()).collect();
        let _ = writeln!(
            output,
            "{}",
            crate::color::dim(&format!(
                "Clearing Git user in {} scope(s)",
                scope_names.join(", ")
            ))
        );
        for scope in scopes {
            Git::clear_user(scope);
        }
    }

    pub fn list(&self, output: &mut impl std::io::Write) {
        for u in self.user_list.list() {
            let _ = writeln!(output, "{}", Git::render(&u));
        }
    }

    pub fn add(&self, user_string: &str, output: &mut impl std::io::Write) {
        let user = match User::parse(user_string) {
            Ok(u) => u,
            Err(e) => {
                let _ = writeln!(output, "{}", crate::color::error(&e.to_string()));
                return;
            }
        };
        if self.user_list.list().iter().any(|u| u == &user) {
            let _ = writeln!(
                output,
                "{}",
                crate::color::dim(&format!(
                    "User '{}' already in user list (try switching to them with 'git su {}')",
                    user,
                    user.initials()
                ))
            );
        } else {
            self.user_list.add(&user);
            let _ = writeln!(
                output,
                "{}",
                crate::color::success(&format!("User '{}' added to users", user))
            );
        }
    }

    pub fn edit_config(&self, path: &Path) -> bool {
        Git::edit_gitsu_config(path)
    }
}
