use crate::codegen::string_format::{code_main, MainConfig};
use crate::codegen::{output_code, CodegenConfig};
use crate::commands::run_command;
use indoc::formatdoc;
use itertools::Itertools;
use std::env;
use std::path::PathBuf;
use std::process::exit;

/// Default version of Yang to use if no local dev version found.
const YANG_BUILD_VERSION: &str = "0.0.12";

/// Name for the codegen binary. Be sure to change BUILD_TOML as well when changing this.
const CODEGEN_BINARY: &str = "intermediate-code-generator";

/// File contents for the intermediate cargo.toml that is only meant for generating the actual code
/// at the end.
fn toml_code() -> String {
    let yang_version = match env::var("YANG_DEV_DIR") {
        Ok(dir) => {
            println!("Linking intermediate binary to local yang dev.");
            format!("{{path = \"{}\"}}", dir).replace('\\', "/")
        }
        Err(_) => {
            println!("Linking intermediate binary to yang {}", YANG_BUILD_VERSION);
            format!("\"{}\"", YANG_BUILD_VERSION)
        }
    };
    // note that zamm_yin must be running on the same version as whatever version yang is built on,
    // *not* whatever version the user is building for, because otherwise different graphs will be
    // used and it won't be initialized properly.
    //
    // Put another way, the intermediate exe depends on this particular version of yang, which
    // depends on this version of yin, not the version that the user is building for.
    formatdoc! {r#"
        [package]
        name = "intermediate-code-generator"
        version = "1.0.0"
        edition = "2018"

        [dependencies]
        zamm_yin = "0.0.13"
        zamm_yang = {}
    "#, yang_version}
}

/// Directory where we're outputting things.
fn build_subdir() -> PathBuf {
    let mut tmp = env::current_dir().unwrap();
    tmp.push(".yang");
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
        &CodegenConfig {
            // skip because this gets applied to long doc strings, and attributes on expressions
            // are not supported. see https://github.com/rust-lang/rust/issues/15701
            add_rustfmt_attributes: false,
            ..CodegenConfig::default()
        },
    );
}

/// Write the cargo.toml
fn output_cargo_toml() {
    let mut cargo_toml = build_subdir();
    cargo_toml.push("Cargo.toml"); // Cargo files are somehow uppercased by default
    output_code(
        &toml_code(),
        &cargo_toml.to_str().unwrap(),
        &CodegenConfig {
            // skip because this gets applied to long doc strings, and attributes on expressions
            // are not supported. see https://github.com/rust-lang/rust/issues/15701
            add_rustfmt_attributes: false,
            comment_autogen: false, // only Rust-style comments supported at the moment
            ..CodegenConfig::default()
        },
    );
}

/// Set up the build directory for compilation of a program that will then go on to generate the
/// final code files.
fn output_build_dir(cfg: &MainConfig) {
    // coalesce into lines first, in case of multiline chunks
    let mut separated_cfg = separate_imports(&cfg.lines.iter().format("\n").to_string());
    for import in &cfg.imports {
        separated_cfg.imports.push(import.clone());
    }
    output_main(&separated_cfg);
    output_cargo_toml();
    println!("Finished generating codegen files.");
}

/// Separate imports embedded in the code, similar to how `rustdoc` does it.
fn separate_imports(code: &str) -> MainConfig {
    let mut imports = vec![];
    let mut lines = vec![];
    for line in code.split('\n') {
        if line.starts_with("use ") {
            imports.push(
                line.chars()
                    .skip(4)
                    .take(line.chars().count() - 5)
                    .collect(),
            );
        } else if !line.is_empty() {
            lines.push(line.to_owned());
        }
    }

    let mut combined_lines = vec![];
    if !lines.is_empty() {
        // combine lines together into one fragment to preserve indentation
        combined_lines.push(lines.iter().format("\n").to_string());
    }
    MainConfig {
        imports,
        lines: combined_lines,
    }
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
    let mut binary = subdir;
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_separate_imports_empty() {
        assert_eq!(
            separate_imports(""),
            MainConfig {
                imports: vec![],
                lines: vec![],
            }
        );
    }

    #[test]
    fn test_separate_imports_no_imports() {
        assert_eq!(
            separate_imports(indoc! {"
            let x = 1;
            let y = x + 1;"}),
            MainConfig {
                imports: vec![],
                lines: vec!["let x = 1;\nlet y = x + 1;".to_owned()],
            }
        );
    }

    #[test]
    fn test_separate_imports_imports_only() {
        assert_eq!(
            separate_imports(indoc! {"
            use std::rc::Rc;
            use crate::my::Struct;"}),
            MainConfig {
                imports: vec!["std::rc::Rc".to_owned(), "crate::my::Struct".to_owned()],
                lines: vec![],
            }
        );
    }

    #[test]
    fn test_separate_imports_subsequent() {
        assert_eq!(
            separate_imports(indoc! {"
            use std::rc::Rc;
            use crate::my::Struct;
            
            let x = 1;
            let y = x + 1;"}),
            MainConfig {
                imports: vec!["std::rc::Rc".to_owned(), "crate::my::Struct".to_owned()],
                lines: vec!["let x = 1;\nlet y = x + 1;".to_owned()],
            }
        );
    }

    #[test]
    fn test_separate_imports_mixed() {
        assert_eq!(
            separate_imports(indoc! {"
            use std::rc::Rc;
            
            let x = 1;
            use crate::my::Struct;
            let y = x + 1;"}),
            MainConfig {
                imports: vec!["std::rc::Rc".to_owned(), "crate::my::Struct".to_owned()],
                lines: vec!["let x = 1;\nlet y = x + 1;".to_owned()],
            }
        );
    }
}
