# git-codeowners

**A git subcommand to query and validate CODEOWNERS.**

List owners of files based on the [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) file of the current repository.

```
> git codeowners src/main.rs
src/main.rs                              @weyland
```

```
> git codeowners `git ls-files`
.gitignore                               (unowned)
Cargo.lock                               @weyland
Cargo.toml                               @weyland
LICENSE                                  (unowned)
README.md                                @weyland
src/main.rs                              @weyland
```

```
# Oops, did the last commit add unowned files?
> git diff --name-only --diff-filter=A | git codeowners
.gitignore                               @bob
LICENSE                                  (unowned)
```

```
# I need stats about my repo
> echo `git ls-files | git codeowners | grep "(unowned)"| wc -l` out of `git ls-files | git codeowners | wc -l` files in this repository do not have a corresponding CODEOWNERS entry
2 out of 6 files in this repository do not have a CODEOWNERS entry
```

## Installation

- via [pip](https://pypi.org/project/git-codeowners/): `pip install git-codeowners`
- via [Cargo](https://crates.io/crates/git-owners): `cargo install git-owners`

  Note: The Cargo crate installs an equivalent `git-owners` command in addition to `git-codeowners`, for backwards compatibility.


## Usage

- Show owner of a file

  ```
  git codeowners some/file
  ```

- Show owners for a list of files

  ```
  git codeowners some/file some/other/file
  ```

> [!NOTE]
> Each path should be relative to the git repository root -- this makes it easy to combine with other git commands.

- Show owners for every tracked file in the repository

  ```
  git ls-files | git codeowners
  ```

- Show owners for files modified in last five commits

  ```
  git diff --name-only HEAD~5 HEAD | git codeowners
  ```

- Congratulate the user if the current changeset does not add files without owner

  ```
  git diff --diff-filter=ACR --name-only | git codeowners && echo "Great job! No unowned files added!"
  ```

- Get an overview of your CODEOWNERS coverage
  ```
  echo `git ls-files | git codeowners | grep "(unowned)"| wc -l` out of `git ls-files | git codeowners | wc -l` files in this repository do not have a CODEOWNERS entry
  ```

## Features

- Detects the right CODEOWNERS file of the current git repository
- Is composable & scripting friendly
- Works well as a pre-commit hook
- Is fast & written in Rust

## Issues & Contributing

If you have any questions or problems, feel free to communicate using Github Issues.
