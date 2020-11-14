use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use indoc::formatdoc;
use std::env;
use std::fs;
use std::fs::read_to_string;
use std::io::Error;
use std::process::exit;
use toml::Value;
use zamm_yang::codegen::string_format::MainConfig;
use zamm_yang::codegen::track_autogen::clean_autogen;
use zamm_yang::codegen::CodegenConfig;
use zamm_yang::commands::run_command;
use zamm_yang::main_build::generate_final_code;
use zamm_yang::parse::{find_file, parse_input};
use zamm_yang::tao::initialize_kb;

/// Help text to display for the input file argument.
const INPUT_HELP_TEXT: &str =
    "The input file containing relevant information to generate code for. Currently only Markdown \
    (extension .md) is supported. If no input file is provided, yang will look for a file named \
    `yin` with one of the above extensions, in the current directory.";

struct BuildConfig<'a> {
    input_file: Option<&'a str>,
    codegen_cfg: CodegenConfig,
}

/// Prepare for release build.
fn release_pre_build() -> Result<(), Error> {
    if !run_command("git", &["status", "--porcelain"]).is_empty() {
        eprintln!(
            "{}",
            "Git repo dirty, commit changes before releasing."
                .red()
                .bold()
        );
        exit(1);
    }
    clean_autogen()?;
    Ok(())
}

fn update_cargo_lock(package_name: &str, new_version: &str) -> Result<(), Error> {
    let cargo_lock = "Cargo.lock";
    let lock_contents = read_to_string(cargo_lock)?;
    let mut lock_cfg = lock_contents.parse::<Value>().unwrap();
    for table_value in lock_cfg["package"].as_array_mut().unwrap() {
        let table = table_value.as_table_mut().unwrap();
        if table["name"].as_str().unwrap() == package_name {
            table["version"] = toml::Value::String(new_version.to_owned());
        }
    }
    fs::write(cargo_lock, lock_cfg.to_string())?;
    Ok(())
}

/// Get version of the project in the current directory. Also removes any non-release tags from the
/// version (e.g. any "-beta" or "-alpha" suffixes).
fn local_project_version() -> Result<String, Error> {
    let cargo_toml = "Cargo.toml";
    let build_contents = read_to_string(cargo_toml)?;
    let mut build_cfg = build_contents.parse::<Value>().unwrap();
    let release_version = {
        let version = build_cfg["package"]["version"].as_str().unwrap();
        if !version.contains('-') {
            return Ok(version.to_owned());
        }
        // otherwise, get rid of tag
        version.split('-').next().unwrap().to_owned()
    };
    {
        build_cfg["package"]["version"] = toml::Value::String(release_version.clone());
        update_cargo_lock(
            build_cfg["package"]["name"].as_str().unwrap(),
            &release_version,
        )?;
    }
    fs::write(cargo_toml, build_cfg.to_string())?;
    Ok(release_version)
}

/// Destructively prepare repo for release after build.
fn release_post_build() -> Result<(), Error> {
    let version = local_project_version()?;

    // switch to new release branch
    let release_branch = format!("release/v{}", version);
    run_command("git", &["checkout", "-b", release_branch.as_str()]);
    // remove build.rs because it won't be useful on docs.rs anyways
    run_command("git", &["rm", "-f", "build.rs"]);
    // commit everything
    run_command("git", &["add", "."]);
    let commit_message = format!("Creating release v{}", version);
    run_command("git", &["commit", "-m", commit_message.as_str()]);

    Ok(())
}

fn generate_code(build_cfg: &BuildConfig) -> Result<(), Error> {
    let found_input = find_file(build_cfg.input_file)?;
    let literate_rust_code = parse_input(found_input)?;

    let define_codegen_cfg = formatdoc! {r#"
        let codegen_cfg = CodegenConfig {{
            comment_autogen: {comment_autogen},
            add_rustfmt_attributes: {add_rustfmt_attributes},
            track_autogen: {track_autogen},
            yin: {yin},
            release: {release},
        }};
    "#, comment_autogen = build_cfg.codegen_cfg.comment_autogen,
    add_rustfmt_attributes = build_cfg.codegen_cfg.add_rustfmt_attributes,
    track_autogen = build_cfg.codegen_cfg.track_autogen,
    yin = build_cfg.codegen_cfg.yin,
    release = build_cfg.codegen_cfg.release};

    let kb_init = formatdoc! {r#"
        initialize_kb();
        // ------------------------ START OF LITERATE RUST -------------------------
        {}
        // -------------------------- END OF LITERATE RUST -------------------------
        handle_all_implementations(&codegen_cfg);
    "#, literate_rust_code.trim()};

    generate_final_code(&MainConfig {
        imports: vec![
            "zamm_yin::tao::Tao".to_owned(),
            "zamm_yin::tao::archetype::ArchetypeTrait".to_owned(),
            "zamm_yin::tao::archetype::ArchetypeFormTrait".to_owned(),
            "zamm_yin::tao::archetype::AttributeArchetype".to_owned(),
            "zamm_yin::tao::form::FormTrait".to_owned(),
            "zamm_yin::node_wrappers::CommonNodeTrait".to_owned(),
            "zamm_yang::codegen::CodegenConfig".to_owned(),
            "zamm_yang::tao::callbacks::handle_all_implementations".to_owned(),
            "zamm_yang::tao::initialize_kb".to_owned(),
            "zamm_yang::tao::Implement".to_owned(),
            "zamm_yang::tao::ImplementConfig".to_owned(),
            "zamm_yang::tao::archetype::CodegenFlags".to_owned(),
            "zamm_yang::tao::form::DefinedMarker".to_owned(),
            "zamm_yang::tao::archetype::CreateImplementation".to_owned(),
            "zamm_yang::define".to_owned(),
            "zamm_yang::helper::aa".to_owned(),
        ],
        lines: vec![define_codegen_cfg, kb_init],
    });

    Ok(())
}

/// Generate code from the input file.
fn build(args: &ArgMatches) -> Result<(), Error> {
    let build_cfg = BuildConfig {
        input_file: args.value_of("INPUT"),
        codegen_cfg: CodegenConfig {
            comment_autogen: args
                .value_of("COMMENT_AUTOGEN")
                .unwrap_or("true")
                .parse::<bool>()
                .unwrap(),
            add_rustfmt_attributes: true,
            track_autogen: args.is_present("TRACK_AUTOGEN"),
            yin: args.is_present("YIN"),
            release: false,
        },
    };

    generate_code(&build_cfg)?;
    Ok(())
}

fn release(args: &ArgMatches) -> Result<(), Error> {
    let build_cfg = BuildConfig {
        input_file: args.value_of("INPUT"),
        codegen_cfg: CodegenConfig {
            comment_autogen: false,
            add_rustfmt_attributes: true,
            track_autogen: false,
            yin: args.is_present("YIN"),
            release: true,
        },
    };

    release_pre_build()?;
    generate_code(&build_cfg)?;
    release_post_build()?;
    Ok(())
}

/// Clean all autogenerated files.
fn clean(_: &ArgMatches) -> Result<(), Error> {
    clean_autogen()?;
    Ok(())
}

/// Run various tests and checks.
fn test(args: &ArgMatches) -> Result<(), Error> {
    let yang = args.is_present("YANG");

    println!("Formatting...");
    run_command("cargo", &["fmt"]);
    println!("Running tests...");
    run_command("cargo", &["test"]);
    println!("Running lints...");
    run_command(
        "cargo",
        &[
            "clippy",
            "--all-features",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    );
    if yang {
        println!("Running yang build...");
        run_command("cargo", &["run", "build"]);
    }
    Ok(())
}

/// The entry-point to this code generation tool.
fn main() {
    // Avoid using clapp_app! macro due to a bug with the short arg name getting assigned only to
    // 'a'
    let args = App::new("yang")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .author("Amos Ng <me@amos.ng>")
        .about("Code generator for Yin.")
        .subcommand(
            SubCommand::with_name("build")
                .setting(AppSettings::ColoredHelp)
                .about("Generate code from an input file")
                .arg(
                    Arg::with_name("INPUT")
                        .value_name("INPUT")
                        .help(INPUT_HELP_TEXT)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("COMMENT_AUTOGEN")
                        .short("c")
                        .long("comment_autogen")
                        .value_name("COMMENT_AUTOGEN")
                        .help(
                            "Whether or not to add an autogeneration comment to each generated \
                            line of code. Defaults to true.",
                        )
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("TRACK_AUTOGEN")
                        .short("t")
                        .long("track-autogen")
                        .help(
                            "Whether or not we want Cargo to track autogenerated files and \
                            rebuild when they change. Can result in constant rebuilds.",
                        ),
                )
                .arg(
                    Arg::with_name("YIN")
                        .short("y")
                        .long("yin")
                        .help("Set to generate code for Yin instead"),
                ),
        )
        .subcommand(
            SubCommand::with_name("release")
                .setting(AppSettings::ColoredHelp)
                .about("Prepare repo for a Cargo release")
                .arg(
                    Arg::with_name("INPUT")
                        .value_name("INPUT")
                        .help(INPUT_HELP_TEXT)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("YIN")
                        .short("y")
                        .long("yin")
                        .help("Set to generate code for Yin instead"),
                ),
        )
        .subcommand(
            SubCommand::with_name("clean")
                .setting(AppSettings::ColoredHelp)
                .about("Clean up autogenerated files"),
        )
        .subcommand(
            SubCommand::with_name("test")
                .setting(AppSettings::ColoredHelp)
                .about("Make sure the project will pass CI tests")
                .arg(
                    Arg::with_name("YANG")
                        .short("y")
                        .long("yang")
                        .help("Set when testing yang itself"),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    initialize_kb();

    let result = if let Some(build_args) = args.subcommand_matches("build") {
        build(build_args)
    } else if let Some(release_args) = args.subcommand_matches("release") {
        release(release_args)
    } else if let Some(clean_args) = args.subcommand_matches("clean") {
        clean(clean_args)
    } else if let Some(test_args) = args.subcommand_matches("test") {
        test(test_args)
    } else {
        panic!("Arg not found. Did you reconfigure clap recently?");
    };

    exit(match result {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    })
}
