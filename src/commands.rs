use colored::*;
use itertools::Itertools;
use std::ffi::OsStr;
use std::process::{exit, Command};

/// Run a command. Exits on failure.
pub fn run_command<I, S>(command: &str, args: I) -> String
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<OsStr> + std::fmt::Display,
{
    let command_str = format!(
        "{} {}",
        command,
        &args.clone().into_iter().map(|s| s.to_string()).format(" ")
    );

    let result = Command::new(command)
        .args(args)
        .output()
        .unwrap_or_else(|_| panic!("Could not run command: {}", command_str));

    if !result.status.success() {
        eprintln!(
            "{}",
            format!("Command failed: {}", command_str).red().bold()
        );
        eprint!("{}", std::str::from_utf8(&result.stderr).unwrap());
        exit(1);
    }

    std::str::from_utf8(&result.stdout).unwrap().to_owned()
}
