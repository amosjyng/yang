use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Which OS we're running on -- not the OS that we're targeting for potential cross-compilation.
#[cfg(target_os = "linux")]
const CURRENT_OS: &str = "linux";
#[cfg(target_os = "macos")]
const CURRENT_OS: &str = "mac";
#[cfg(target_os = "windows")]
const CURRENT_OS: &str = "windows";

/// The filename extension for the yang executable.
#[cfg(target_family = "unix")]
const BINARY_EXT: &str = "";
#[cfg(target_family = "windows")]
const BINARY_EXT: &str = ".exe";

/// The version of the Yang that will be used to generate build files.
const YANG_DEP_VERSION: &str = "0.0.9";

/// Call out to the commandline version of yang.
fn run_yang<I, S>(yang_binary: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let result = Command::new(yang_binary)
        .args(args)
        .output()
        .unwrap_or_else(|_| {
            panic!(
                "Could not generate attribute `Target` using yang binary located at {}",
                yang_binary
            )
        });

    if !result.status.success() {
        eprint!("{}", std::str::from_utf8(&result.stderr).unwrap());
        panic!("Yang binary command failed.");
    }

    print!("{}", std::str::from_utf8(&result.stdout).unwrap());
}

/// Build `yang` using itself.
fn main() {
    // no need to regenerate autogenerated files every time
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();
    let yang_binary = match env::var("YANG_BINARY") {
        Ok(custom_path) => {
            // custom paths useful when bootstrapping from scratch
            println!("Custom path for yang binary set to {}", custom_path);
            custom_path
        }
        Err(_) => format!("{}/yang-v{}{}", out_dir, YANG_DEP_VERSION, BINARY_EXT),
    };

    if Path::new(&yang_binary).exists() {
        println!("Yang executable already exists at {}", yang_binary);
    } else {
        let yang_url = format!(
            "https://bintray.com/amosjyng/zamm/download_file?file_path=yang%2F{}%2F{}%2Fyang{}",
            YANG_DEP_VERSION, CURRENT_OS, BINARY_EXT
        );

        println!("Bintray URL determined to be {}", yang_url);
        println!("Yang executable will be saved locally to {}", yang_binary);

        let mut binary_output = fs::File::create(&yang_binary).expect("Cannot create yang file");

        let yang_bytes = reqwest::blocking::get(&yang_url)
            .expect("Can't download yang from Bintray.")
            .bytes()
            .expect("Cannot get yang bytes after download.");
        binary_output
            .write_all(&yang_bytes)
            .expect("Cannot write yang bytes to binary.");
        binary_output
            .flush()
            .expect("Cannot flush yang bytes after write.");

        if fs::metadata(&yang_binary).unwrap().len() == 0 {
            fs::remove_file(&yang_binary).unwrap();
            panic!(
                "FAILURE: yang version downloaded from {} is empty.",
                yang_url
            );
        }
    }

    // downloaded executable can be run immediately on Windows, so skip this step.
    #[cfg(target_family = "unix")]
    Command::new("chmod")
        .args(&["+x", yang_binary.as_str()])
        .output()
        .expect("Could not add execute permissions to yang binary");

    println!("==================== RUNNING YANG ====================");

    run_yang(&yang_binary, &["build"]);
}
