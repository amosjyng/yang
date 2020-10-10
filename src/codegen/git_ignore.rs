use std::process::Command;

pub fn git_rm(filepath: &str) {
    match Command::new("git").args(&["rm", "-f", filepath]).output() {
        Ok(_) => println!("Removed existing file from Git: {}", filepath),
        Err(e) => println!(
            "Could not remove file from Git, please do so manually: {}. Error was: {}",
            filepath, e
        ),
    }
}
