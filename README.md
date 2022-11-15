# git-codeowners

**A git subcommand to query and validate CODEOWNERS.**

List owners of files based on the [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) of the current repository.

```
> git codeowners src/main.rs
src/main.rs                    @weyland
```

```
> git codeowners `git ls-files`
.gitignore                     (unowned)
Cargo.lock                     @weyland
Cargo.toml                     @weyland
LICENSE                        (unowned)
README.md                      @weyland
src/main.rs                    @weyland
```

```
# oops, did the last commit add some unowned files?
> git diff --name-only --diff-filter=A | git codeowners
.gitignore                     (unowned)
LICENSE                        (unowned)
```

```
# I need stats about my big monorepo
> echo `git ls-files | git codeowners | grep "(unowned)"| wc -l` out of `git ls-files | git codeowners | wc -l` files in this repository do not have a corresponding CODEOWNERS entry
2 out of 6 files in this repository do not have a corresponding CODEOWNERS entry
```

## Installation

- Via pip: `pip install git-codeowners`
- Via Cargo: `cargo install git-owners`

## Usage

- Get owners of a file

  ```
  git codeowners some/file
  ```

- Get owners for a list of files

  ```
  git codeowners some/file some/other/file
  ```

- Get owners for every tracked file

  ```
  git ls-files | git codeowners
  ```

- Get owners for files modified in last five commits

  ```
  git diff --name-only HEAD~5 HEAD | git codeowners
  ```

- Congratulate the user if the current changeset does not add files without owner

  ```
  git diff --diff-filter=ACR --name-only | git codeowners && echo "Great job! No unowned files added!"
  ```

- Get an overview of your CODEOWNERS coverage
  ```
  echo `git ls-files | git codeowners | grep "(unowned)"| wc -l` out of `git ls-files | git codeowners | wc -l` files in this repository do not have a corresponding CODEOWNERS entry
  ```

## Features

- Detects the right CODEOWNERS file of the current git repository
- Is composable & scripting friendly
- Works well as a pre-commit hook
- Is fast & written in Rust

## Issues & Contributing

If you have any questions or problems, feel free to communicate using Github Issues.
