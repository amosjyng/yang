use std::ffi::OsStr;
use std::io;
use std::process::{exit, Command, Output, Stdio};
use std::str;

/// The version of the ZAMM that will be used to generate build files.
///
/// Lock this in so that if there are future releases of ZAMM that are not backwards-compatible,
/// this particular commit will still build.
const ZAMM_VERSION: &str = "0.0.1";

/// Call out to the commandline.
fn run_command<I, S>(streaming: bool, command: &str, args: I) -> io::Result<Output>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(command);
    if streaming {
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }
    command.args(args).output()
}

/// Run a command and assert that it succeeded.
fn assert_command<I, S>(streaming: bool, command: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    assert!(run_command(streaming, command, args)
        .unwrap()
        .status
        .success());
}

fn check_zamm_version() -> bool {
    match run_command(false, "zamm", vec!["--help"]) {
        Ok(output) => {
            let version_str = format!("zamm {}", ZAMM_VERSION);
            let output_str = str::from_utf8(&output.stdout).unwrap();
            let verified = output.status.success() && output_str.contains(&version_str);
            if verified {
                println!("Verified that ZAMM v{} is installed.", ZAMM_VERSION);
            } else {
                println!("ZAMM is installed but not at version {}.", ZAMM_VERSION);
            }
            verified
        }
        Err(_) => {
            println!("ZAMM not found on path.");
            false
        }
    }
}

/// Make sure the right version of ZAMM is installed.
fn ensure_zamm_installed() {
    if !check_zamm_version() {
        println!("Installing ZAMM v{} ...", ZAMM_VERSION);
        assert_command(
            true,
            "cargo",
            vec![
                "install",
                "--version",
                ZAMM_VERSION,
                "--force",
                "--",
                "zamm",
            ],
        );
        if !check_zamm_version() {
            eprintln!("Could not install ZAMM v{}", ZAMM_VERSION);
            exit(1);
        }
    }
}

/// Build `yang` using ZAMM.
fn main() {
    // no need to regenerate autogenerated files every time
    println!("cargo:rerun-if-changed=build.rs");
    ensure_zamm_installed();

    println!("==================== RUNNING ZAMM ====================");
    //assert_command(true, "zamm", &["build"]);
}
