// git-su: manage Git users, switch between users, pair programming
// Port of gitsu (Ruby) to Rust

use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;

use clap::Parser;
use dialoguer::{Input, Select};
use git_su::{Scope, Switcher, UserFile, UserList};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(name = "git-su")]
#[command(version = VERSION)]
#[command(about = "Manage your Git users. Switch between users, list, add, clear.")]
struct Args {
    /// List the configured users
    #[arg(short = 't', long = "list")]
    list: bool,

    /// Clear the current user (optionally specify scope with --local/--global/--system)
    #[arg(short = 'c', long = "clear")]
    clear: bool,

    /// Add a user in format "Name <email@example.com>"
    #[arg(short = 'a', long = "add")]
    add: Option<String>,

    /// Open the Gitsu config file in an editor
    #[arg(short = 'e', long = "edit")]
    edit: bool,

    /// Change/print user in local scope
    #[arg(short = 'l', long = "local")]
    local: bool,

    /// Change/print user in global scope
    #[arg(short = 'g', long = "global")]
    global: bool,

    /// Change/print user in system scope
    #[arg(short = 's', long = "system")]
    system: bool,

    /// User(s) to switch to (initials, name part, or "Name <email>")
    #[arg(trailing_var_arg = true)]
    users: Vec<String>,
}

fn gitsu_path() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".git-su")
}

fn main() {
    if let Err(e) = run() {
        let _ = writeln!(io::stderr(), "{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mut stdout = io::stdout().lock();

    let user_file = UserFile::new(gitsu_path());
    let user_list = UserList::new(user_file);
    let switcher = Switcher::new(&user_list);

    if args.list {
        switcher.list(&mut stdout);
        return Ok(());
    }

    if args.edit {
        switcher.edit_config(&gitsu_path());
        return Ok(());
    }

    let scopes: Vec<Scope> = {
        let mut s = vec![];
        if args.local {
            s.push(Scope::Local);
        }
        if args.global {
            s.push(Scope::Global);
        }
        if args.system {
            s.push(Scope::System);
        }
        s
    };

    if args.clear {
        switcher.clear(if scopes.is_empty() { &[] } else { &scopes }, &mut stdout);
        return Ok(());
    }

    if let Some(ref user_str) = args.add {
        switcher.add(user_str, &mut stdout);
        return Ok(());
    }

    if args.users.is_empty() {
        run_no_args(&user_list, &switcher, &scopes, &mut stdout)?;
        return Ok(());
    }

    let scope = if scopes.is_empty() {
        Scope::Default
    } else if scopes.len() == 1 {
        scopes[0]
    } else {
        Scope::Default
    };

    if scopes.len() > 1 {
        for scope in &scopes {
            switcher.request(*scope, &args.users, &mut stdout);
        }
    } else {
        switcher.request(scope, &args.users, &mut stdout);
    }

    Ok(())
}

fn run_no_args(
    user_list: &UserList,
    switcher: &Switcher,
    scopes: &[Scope],
    stdout: &mut impl Write,
) -> Result<(), Box<dyn std::error::Error>> {
    let users = user_list.list();
    let is_tty = std::io::stdin().is_terminal();

    if is_tty {
        if scopes.is_empty() {
            switcher.print_current(&[], stdout);
        } else {
            switcher.print_current(scopes, stdout);
        }
        let mut items: Vec<String> = vec![];
        items.extend(users.iter().map(|u| u.to_string()));
        items.push("(Add user)".to_string());

        let add_user_index = items.len() - 1;
        let _ = writeln!(stdout);
        let select_prompt = git_su::color::label("Select user:");
        let selection = Select::new()
            .with_prompt(&select_prompt)
            .items(&items)
            .default(0)
            .interact_opt()?;

        match selection {
            None => return Ok(()),
            Some(0) => {
                // Already shown at start, no need to repeat
                return Ok(());
            }
            Some(i) if i == add_user_index => {
                let name: String = Input::new()
                    .with_prompt("Name")
                    .allow_empty(false)
                    .interact_text()?;
                let email: String = Input::new()
                    .with_prompt("Email")
                    .allow_empty(false)
                    .interact_text()?;
                let user_str = format!("{} <{}>", name.trim(), email.trim());
                switcher.add(&user_str, stdout);
                return Ok(());
            }
            Some(i) => {
                let scope = if scopes.is_empty() {
                    Scope::Default
                } else if scopes.len() == 1 {
                    scopes[0]
                } else {
                    Scope::Default
                };
                let user_str = items[i].clone();
                if scopes.len() > 1 {
                    for s in scopes {
                        switcher.request(*s, &[user_str.clone()], stdout);
                    }
                } else {
                    switcher.request(scope, &[user_str], stdout);
                }
                return Ok(());
            }
        }
    }

    if scopes.is_empty() {
        switcher.print_current(&[], stdout);
    } else {
        switcher.print_current(scopes, stdout);
    }
    Ok(())
}
