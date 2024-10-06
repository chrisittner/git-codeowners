//! A git subcommand to query and validate CODEOWNERS.
//! See [https://github.com/chrisittner/git-owners](https://github.com/chrisittner/git-owners)

extern crate codeowners;
use atty::Stream;
use clap::Parser;
use std::io::{self, BufRead};
use std::path;
use std::process;

/// git-owners - List owners of files
/// based on the CODEOWNERS file of the current repository
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// One or more file paths for which to check ownership.
    /// Can also be provided via pipe/stdin.
    /// Each path should be relative to the git repository root
    /// -- this makes it easy to do e.g. `git ls-files | git owners`
    #[clap(required(true))]
    paths: Vec<String>,
}

fn main() {
    if data_was_piped_in() {
        let stdin = io::stdin();
        let lines_from_stdin = stdin.lock().lines();

        let paths: Vec<String> = lines_from_stdin
            .map(|l| l.unwrap().trim().to_string())
            .collect::<Vec<_>>();

        check_ownership_and_exit(&paths);
    } else {
        let args = Args::parse();

        check_ownership_and_exit(&args.paths);
    }
}

fn data_was_piped_in() -> bool {
    !atty::is(Stream::Stdin)
}

fn check_ownership_and_exit(paths: &Vec<String>) {
    let codeowners_path = match get_codeowners_path() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    let codeowners = codeowners::from_path(codeowners_path);

    let mut unowned_files_found = false;

    for path in paths {
        if !check_and_print_owners(&codeowners, &path) {
            unowned_files_found = true
        }
    }

    if unowned_files_found {
        process::exit(1);
    } else {
        process::exit(0);
    }
}

fn check_and_print_owners(owners: &codeowners::Owners, path: &String) -> bool {
    match owners.of(&path) {
        None => {
            println!("{: <40} (unowned)", path);
            false
        }
        Some(owners) => {
            let owner_str = owners
                .into_iter()
                .map(|owner| owner.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            println!("{: <40} {}", path, owner_str);
            true
        }
    }
}

fn get_codeowners_path() -> io::Result<path::PathBuf> {
    let repo_root = get_current_repo_root()?;

    let co_repo_root = repo_root.join("CODEOWNERS");
    let co_docs = repo_root.join("docs").join("CODEOWNERS");
    let co_dotgithub = repo_root.join(".github").join("CODEOWNERS");

    if co_repo_root.exists() {
        Ok(co_repo_root)
    } else if co_docs.exists() {
        Ok(co_docs)
    } else if co_dotgithub.exists() {
        Ok(co_dotgithub)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "fatal: no CODEOWNERS file found. checked {}, {}, {}",
                co_dotgithub.display(),
                co_docs.display(),
                co_repo_root.display()
            ),
        ))
    }
}

fn get_current_repo_root() -> io::Result<path::PathBuf> {
    let res = process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .stdout(process::Stdio::piped())
        .output()?;

    if res.status.success() {
        let repo_root = String::from_utf8(res.stdout).unwrap();
        Ok(path::PathBuf::from(repo_root.trim()))
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            String::from_utf8(res.stderr).unwrap(),
        ))
    }
}
