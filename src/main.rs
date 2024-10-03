use std::path::PathBuf;
use anyhow::{anyhow, Context, Result};

const HELP: &str = "\
lscat

USAGE:
  lscat <PATH> [OPTIONS]

FLAGS:
  -h, --help            Prints help information

ARGS:
  <PATH>                The path to the file to list or cat

note: OPTIONS are passed to the ls or cat command that is executed.
";

#[derive(Debug)]
struct AppArgs {
    path: PathBuf,
    bypass_options: Vec<String>,
}

fn parse_args() -> Result<AppArgs> {
    let mut bypass_options = Vec::new();
    let mut path = None;

    for arg in std::env::args().skip(1) {
        if arg == "-h" || arg == "--help" {
            println!("{}", HELP);
            std::process::exit(0);
        }

        if arg.starts_with('-') {
            bypass_options.push(arg);
        } else {
            if path.is_some() {
                return Err(anyhow!("only one path can be specified"));
            }

            path = Some(PathBuf::from(arg));
        }
    }

    let path = path.ok_or(anyhow!("no path specified"))?;

    Ok(AppArgs {
        path,
        bypass_options,
    })
}

fn get_ls_command() -> String {
    let default = if cfg!(windows) {
        "dir".to_string()
    } else {
        "ls --color=always".to_string()
    };

    std::env::var("LSCAT_LS").unwrap_or(default)
}

fn get_cat_command() -> String {
    let default = if cfg!(windows) {
        "type".to_string()
    } else {
        "cat".to_string()
    };

    std::env::var("LSCAT_CAT").unwrap_or(default)
}

fn run(args: AppArgs) -> Result<()> {
    if !args.path.exists() {
        return Err(anyhow!("path does not exist"));
    }

    let command = if args.path.is_dir() {
        get_ls_command()
    } else {
        get_cat_command()
    };

    #[cfg(unix)]
    let words = shell_words::split(&command)?;
    #[cfg(windows)]
    let words = winsplit::split(&command);

    let (exec, opts) = words.split_first().context("command is empty")?;

    let status = std::process::Command::new(exec)
        .args(opts)
        .arg(&args.path)
        .args(&args.bypass_options)
        .status()?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run(args) {
        eprintln!("Error: {}.", e);
        std::process::exit(1);
    }
}
