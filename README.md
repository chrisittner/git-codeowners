# git-owners

**A git subcommand to query and validate CODEOWNERS.**

List owners of files based on the [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) of the current repository.

```
> git owners src/main.rs
src/main.rs                    @weyland
```

```
> git owners `git ls-files`
.gitignore                     (unowned)
Cargo.lock                     @weyland
Cargo.toml                     @weyland
LICENSE                        (unowned)
README.md                      @weyland
src/main.rs                    @weyland
```

```
# oops, did the last commit add some unowned files?
> git diff --name-only --diff-filter=A | git owners
.gitignore                     (unowned)
LICENSE                        (unowned)
```

```
# I need stats about my big monorepo
> echo `git ls-files | git owners | grep "(unowned)"| wc -l` out of `git ls-files | git owners | wc -l` files in this repository do not have a corresponding CODEOWNERS entry
2 out of 6 files in this repository do not have a corresponding CODEOWNERS entry
```

## Installation

- Via Cargo: `cargo install git-owners`

## Usage

- Get owners of a file

  ```
  git owners some/file
  ```

- Get owners for a list of files

  ```
  git owners some/file some/other/file
  ```

- Get owners for every tracked file

  ```
  git ls-files | git owners
  ```

- Get owners for files modified in last five commits

  ```
  git diff --name-only HEAD~5 HEAD | git owners
  ```

- Congratulate the user if the current changeset does not add files without owner

  ```
  git diff --diff-filter=ACR --name-only | git owners && echo "Great job! No unowned files added!"
  ```

- Get an overview of your CODEOWNERS coverage
  ```
  echo `git ls-files | git owners | grep "(unowned)"| wc -l` out of `git ls-files | git owners | wc -l` files in this repository do not have a corresponding CODEOWNERS entry
  ```

## Features

- Detects the right CODEOWNERS file of the current git repository
- Is composable & scripting friendly
- Works well as a pre-commit hook
- Is fast & written in Rust

## Issues & Contributing

If you have any questions or problems, feel free to communicate using Github Issues.
