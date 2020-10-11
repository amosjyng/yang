use path_abs::PathAbs;
use std::cell::RefCell;
use std::fs::{remove_file, File, OpenOptions};
use std::io::{BufRead, BufReader, Result, Write};

thread_local! {
    static AUTOGEN_FILES: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

/// The path to the file that tracks autogenerated files.
pub const AUTOGEN_TRACKER: &str = ".autogen.txt";

/// Track a file as being autogenerated.
pub fn track_autogen(filename: String) {
    AUTOGEN_FILES.with(|f| f.borrow_mut().push(filename));
}

/// Output all autogenerated files to the tracker file.
pub fn save_autogen() {
    let tracker_path = PathAbs::new(AUTOGEN_TRACKER).expect("Cannot open autogen tracker");
    AUTOGEN_FILES.with(|f| {
        for filename in f.borrow().iter() {
            // todo: cut down on the number of file reads here
            add_to_file(&tracker_path, filename).expect("Cannot add to autogen tracker");
        }
    });
}

/// Clean up all autogenerated files.
pub fn clean_autogen() {
    match File::open(AUTOGEN_TRACKER) {
        Ok(existing_tracker) => {
            for line in BufReader::new(existing_tracker).lines() {
                let filename = line.unwrap();
                match remove_file(&filename) {
                    Ok(_) => (), // be silent on success
                    Err(_) => println!(
                        "Skipping deletion of {}. It might not exist anymore.",
                        filename
                    ),
                }
            }
        }
        Err(_) => println!("No autogenerated files to clean up."),
    }
    remove_file(AUTOGEN_TRACKER).unwrap_or(());
}

/// Ensures that the given line will be in the file.
pub fn add_to_file(file: &PathAbs, content: &str) -> Result<()> {
    let mut already_in_file = false;
    match File::open(file.as_path()) {
        Ok(existing_ignore) => {
            for line in BufReader::new(existing_ignore).lines() {
                if line.unwrap() == content {
                    already_in_file = true;
                    break;
                }
            }
        }
        Err(_) => {
            writeln!(File::create(file.as_path())?, "{}", content)?;
            already_in_file = true;
        }
    }
    if !already_in_file {
        let mut existing_file = OpenOptions::new().append(true).open(file.as_path())?;
        writeln!(existing_file, "{}", content)
    } else {
        Ok(())
    }
}