//! A git subcommand to query and validate CODEOWNERS.
//! See [https://github.com/chrisittner/git-codeowners](https://github.com/chrisittner/git-codeowners)

extern crate codeowners;
use atty::Stream;
use clap::{CommandFactory, Parser};
use clap_help::Printer;
use std::io::{self, BufRead};
use std::path;
use std::process;

/// git-codeowners - List owners of files
/// based on the CODEOWNERS file of the current repository
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, disable_help_flag = true)]
struct Args {
    /// One or more file paths for which to check ownership.
    /// Can also be provided via pipe/stdin.
    /// Each path should be relative to the git repository root
    /// -- this makes it easy to do e.g. `git ls-files | git codeowners`
    #[clap()]
    paths: Vec<String>,

    /// Print help
    #[arg(short, long)]
    help: bool,
}

static INTRO: &str = "List owners of files based on the CODEOWNERS file of the current repository";
static USAGE: &str = "
**Usage: ** `git codeowners [options]${positional-args}`
";

pub struct Example {
    pub title: &'static str,
    pub cmd: &'static str,
}

impl Example {
    pub const fn new(title: &'static str, cmd: &'static str) -> Self {
        Self { title, cmd }
    }
}

pub static EXAMPLES: &[Example] = &[
    Example::new("Show owners for every tracked file in the repository", "git ls-files | git codeowners"),
    Example::new(
        "Show owners for files modified in last five commits",
        "git diff --name-only HEAD~5 HEAD | git codeowners"
    ),
    Example::new(
        "Congratulate the user if the current changeset does not add files without owner",
        "git diff --diff-filter=ACR --name-only | git codeowners && echo \"Great job! No unowned files added!\""
    ),
    Example::new(
        "Get an overview of CODEOWNERS coverage",
        "echo `git ls-files | git codeowners | grep \"(unowned)\"| wc -l` out of `git ls-files | wc -l` files do not have a CODEOWNERS entry"
    ),
];

static EXAMPLES_TEMPLATE: &str = "
**Examples:**

${examples
**${example-number})** ${example-title}:
`${example-cmd}`

}
";

fn main() {
    let args = Args::parse();
    if args.help || args.paths.is_empty() {
        let mut printer = Printer::new(Args::command())
            .with("introduction", INTRO)
            .with("usage", USAGE)
            .without("title")
            .without("author");

        printer.template_keys_mut().push("examples");
        printer.set_template("examples", EXAMPLES_TEMPLATE);
        for (i, example) in EXAMPLES.iter().enumerate() {
            printer
                .expander_mut()
                .sub("examples")
                .set("example-number", i + 1)
                .set("example-title", example.title)
                .set("example-cmd", example.cmd);
        }
        printer.print_help();
        return;
    }

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
