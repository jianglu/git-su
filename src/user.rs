// User: parse "Name <email>", combine for pairing, initials

use std::fmt;

#[derive(Clone, Debug)]
pub struct User {
    pub names: Vec<String>,
    pub emails: Vec<String>,
    group_email: Option<String>,
}

impl User {
    pub fn new(name: impl Into<String>, email: impl Into<String>) -> Self {
        User {
            names: vec![name.into()],
            emails: vec![email.into()],
            group_email: None,
        }
    }

    /// Parse "Name <email@example.com>" format.
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let s = s.trim();
        if let Some(open) = s.find('<') {
            if let Some(close) = s.find('>') {
                if close > open && close == s.len() - 1 {
                    let name = s[..open].trim().to_string();
                    let email = s[open + 1..close].trim().to_string();
                    if !name.is_empty() && !email.is_empty() {
                        return Ok(User::new(name, email));
                    }
                }
            }
        }
        Err(ParseError {
            input: s.to_string(),
        })
    }

    pub fn none() -> User {
        User {
            names: vec![],
            emails: vec![],
            group_email: None,
        }
    }

    pub fn name(&self) -> String {
        if self.is_none() {
            return "(none)".to_string();
        }
        self.names.join(" and ")
    }

    pub fn email(&self) -> String {
        if self.emails.is_empty() {
            return String::new();
        }
        if self.emails.len() == 1 {
            return self.emails[0].clone();
        }
        let group = self.group_email.as_deref().unwrap_or("dev@example.com");
        let group_prefix = group.split('@').next().unwrap_or("dev");
        let group_domain = group.split('@').nth(1).unwrap_or("example.com");
        let prefixes: Vec<&str> = self
            .emails
            .iter()
            .map(|e| e.split('@').next().unwrap_or(""))
            .collect();
        format!("{}+{}@{}", group_prefix, prefixes.join("+"), group_domain)
    }

    pub fn initials(&self) -> String {
        self.names
            .join(" ")
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .collect::<String>()
            .to_lowercase()
    }

    pub fn is_none(&self) -> bool {
        self.names.is_empty() && self.emails.is_empty()
    }

    pub fn combine(mut self, other: &User, group_email: &str) -> User {
        if self.is_none() {
            return other.clone();
        }
        if other.is_none() {
            return self;
        }
        self.names.extend(other.names.clone());
        self.emails.extend(other.emails.clone());
        self.group_email = Some(group_email.to_string());
        self
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.email() == other.email()
    }
}
impl Eq for User {}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name(), self.email())
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub input: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Couldn't parse '{}' as user (expected user in format: 'Jane Doe <jane@example.com>')",
            self.input
        )
    }
}

impl std::error::Error for ParseError {}
