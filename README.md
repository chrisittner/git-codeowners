# git-codeowners

**A git subcommand to validate and query CODEOWNERS.**

git-codeowners - List code ownership of files based on the [CODEOWNERS](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) of the current repository.

## Examples

- Get owners of a file

  ```
  git codeowners some/file
  ```

- Get owners of a list of files

  ```
  git codeowners some/file some/other/file
  ```

- Get owners of every tracked file

  ```
  git ls-files | git codeowners
  ```

- Get owners of files modified in last five commits

  ```
  git diff --name-only HEAD~5 HEAD | git codeowners
  ```

- Congratulate the user if the current changeset does not add files without owner

  ```
  git diff --diff-filter=ACR --name-only | git codeowners && echo "Great job! No unowned files added!"
  ```

## Installation

- Via Cargo: `cargo install git-codeowners`

## Features

- Detects the right CODEOWNERS file of the current git repository
- Is composable & scripting friendly
- Works well as a pre-commit hook
- Is fast & written in Rust

## Issues & Contributing

If you have any questions or problems, feel free to communicate using Github Issues. PRs welcome.
