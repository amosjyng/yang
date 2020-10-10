use crate::codegen::track_autogen::{add_to_file, track_autogen};
use path_abs::{PathAbs, PathInfo, PathOps};
use std::io::Result;
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
    // todo: cut down on the number of file reads we're doing here
    track_autogen(gitignore.as_path().to_str().unwrap().to_owned());
    add_to_file(&gitignore, ".gitignore")?;
    track_autogen(file.as_path().to_str().unwrap().to_owned());
    add_to_file(&gitignore, filename)
}
