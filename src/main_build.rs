use crate::codegen::string_format::{code_main, MainConfig};
use crate::codegen::{output_code, CodegenConfig};
use crate::commands::run_command;
use std::env;
use std::path::PathBuf;
use std::process::exit;

/// Name to use for the subdirectory of the temp directory where we're outputting things.
const YANG_BUILD_SUBDIR: &str = "yang";

/// Name for the codegen binary. Be sure to change BUILD_TOML as well when changing this.
const CODEGEN_BINARY: &str = "intermediate-code-generator";

/// File contents for the intermediate cargo.toml that is only meant for generating the actual code
/// at the end.
const BUILD_TOML: &str = r#"[package]
name = "intermediate-code-generator"
version = "1.0.0"

[dependencies]
zamm_yin = "0.0.13"
zamm_yang = "0.0.10"
"#;

fn build_subdir() -> PathBuf {
    let mut tmp = env::temp_dir();
    tmp.push(PathBuf::from(YANG_BUILD_SUBDIR));
    tmp
}

/// Write code for the main function to a file.
fn output_main(cfg: &MainConfig) {
    let mut main_rs = build_subdir();
    main_rs.push("src/main.rs");
    output_code(
        &code_main(cfg),
        &main_rs.to_str().unwrap(),
        // no settings matter, we're generating an intermediate build file that then generates the
        // final code files
        &CodegenConfig::default(),
    );
}

/// Write the cargo.toml
fn output_cargo_toml() {
    let mut cargo_toml = build_subdir();
    cargo_toml.push("Cargo.toml"); // Cargo files are somehow uppercased by default
    output_code(
        BUILD_TOML,
        &cargo_toml.to_str().unwrap(),
        &CodegenConfig {
            comment_autogen: false, // only Rust-style comments supported at the moment
            ..CodegenConfig::default()
        },
    );
}

/// Set up the build directory for compilation of a program that will then go on to generate the
/// final code files.
fn output_build_dir(cfg: &MainConfig) {
    output_main(cfg);
    output_cargo_toml();
    println!("Finished generating codegen files.");
}

fn build_codegen_binary() -> String {
    let src_dir = env::current_dir().unwrap();
    let subdir = build_subdir();
    env::set_current_dir(&subdir).unwrap();

    println!(
        "Now building codegen binary in {} ...",
        subdir.to_str().unwrap()
    );
    let build_result = run_command("cargo", vec!["build"]);

    // Verify successful build
    let mut binary = subdir.clone();
    binary.push(format!("target/debug/{}", CODEGEN_BINARY));
    if cfg!(windows) {
        binary.set_extension("exe");
    }
    let binary_path = binary.to_str().unwrap();
    if !binary.exists() {
        println!(
            "Codegen binary was not found at expected location {}. Build output was:\n\n{}",
            binary_path, build_result
        );
        exit(1);
    }
    println!("Binary successfully built at {}.", binary_path);
    println!(
        "Returning to {} and running codegen...",
        src_dir.to_str().unwrap()
    );
    env::set_current_dir(&src_dir).unwrap();

    binary_path.to_owned()
}

/// Generate code using the code specified in `cfg`.
pub fn generate_final_code(cfg: &MainConfig) {
    output_build_dir(cfg);
    let binary_path = build_codegen_binary();
    println!("==================== RUNNING CODEGEN ====================");
    print!("{}", run_command(&binary_path, Vec::<&str>::new()));
}
