# git-su

This project is inspired by [gitsu](https://github.com/drrb/gitsu). It provides a flexible way to manage multiple Git users, quickly switch identities, and easily support multi-user (pair programming) commits, all implemented in Rust.

## Installation

```bash
cargo install --path .
```

If you see `No manual entry for git-su`, install the man page:

```bash
# User-level (recommended): copy to ~/.local/share/man/man1/ or your MANPATH
mkdir -p ~/.local/share/man/man1
cp man/man1/git-su.1 ~/.local/share/man/man1/
mandb -q 2>/dev/null || true

# Or system-wide (requires privileges):
# sudo cp man/man1/git-su.1 /usr/local/share/man/man1/
# sudo mandb
```

## Usage

- **Show current user**: `git su`
- **Switch user**: `git su "Jane Doe <jane@example.com>"` or add first, then switch by initials
- **Add user**: `git su --add "Jane Doe <jane@example.com>"`
- **Switch by initials or name**: `git su jd`, `git su bob`
- **Pair (multiple users)**: `git su jd bob`
- **List saved users**: `git su --list`
- **Clear current user**: `git su --clear` (optionally with `--local`/`--global`/`--system`)
- **Scopes**: `-l/--local`, `-g/--global`, `-s/--system`

The user list is stored in `~/.git-su` in TOML format. Example:

```toml
[[user]]
name = "Jane Doe"
email = "jane@example.com"

[[user]]
name = "Bob Smith"
email = "bob@example.com"
```

## Configuration (Git config)

- `gitsu.defaultSelectScope`: default scope when switching, one of `local` | `global` | `system`
- `gitsu.groupEmailAddress`: group email domain for pairing, default `dev@example.com`
