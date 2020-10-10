use path_abs::{PathAbs, PathInfo, PathOps};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};
use std::process::Command;

/// Remove a file from Git
pub fn git_rm(filepath: &str) {
    Command::new("git")
        .args(&["rm", "-f", filepath])
        .output()
        .expect(
            format!(
                "Could not remove file from Git, please do so manually: {}",
                filepath
            )
            .as_str(),
        );
}

/// Add a file to gitignore in the same directory. Creates the .gitignore if it doesn't yet exist,
/// and untracks it.
pub fn git_ignore(file: &PathAbs) -> Result<()> {
    let filename = file.file_name().unwrap().to_str().unwrap();
    let gitignore = file.with_file_name(".gitignore");
    let mut already_in_gitignore = false;
    match File::open(gitignore.as_path()) {
        Ok(existing_ignore) => {
            for line in BufReader::new(existing_ignore).lines() {
                if line.unwrap() == filename {
                    already_in_gitignore = true;
                    break;
                }
            }
        }
        Err(_) => {
            writeln!(File::create(gitignore.as_path())?, ".gitignore")?;
        }
    }
    if !already_in_gitignore {
        let mut gitignore_file = OpenOptions::new().append(true).open(gitignore.as_path())?;
        writeln!(gitignore_file, "{}", filename)
    } else {
        Ok(())
    }
}
