use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use itertools::Itertools;
use path_abs::{PathAbs, PathInfo};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::{exit, Command};
use toml::Value;
use zamm_yang::codegen::track_autogen::{clean_autogen, save_autogen};
use zamm_yang::codegen::CodegenConfig;
use zamm_yang::concepts::callbacks::handle_implementation;
use zamm_yang::concepts::{initialize_kb, Implement};
use zamm_yang::parse::{parse_md, parse_yaml};
use zamm_yin::concepts::{ArchetypeTrait, Tao};

/// All supported input filename extensions.
const SUPPORTED_EXTENSIONS: &[&str] = &["md", "yml", "yaml"];

/// Help text to display for the input file argument.
const INPUT_HELP_TEXT: &str =
    "The input file containing relevant information to generate code for. \
    Currently only Markdown (extension .md) and YAML (extensions .yml or \
    .yaml) are supported. If no input file is provided, yang will look for a \
    file named `yin` with one of the above extensions, in the current \
    directory.";

struct BuildConfig<'a> {
    input_file: Option<&'a str>,
    codegen_cfg: CodegenConfig,
}

/// Find the right input file.
fn find_file(specified_file: Option<&str>) -> Result<PathAbs, Error> {
    match specified_file {
        Some(filename) => {
            let path = PathAbs::new(Path::new(&filename))?;
            let path_str = path.as_path().to_str().unwrap();
            if path.exists() {
                println!("Using specified input file at {}", path_str);
                Ok(path)
            } else {
                Err(Error::new(
                    ErrorKind::NotFound,
                    format!("Specified input file was not found at {}", path_str),
                ))
            }
        }
        None => {
            for extension in SUPPORTED_EXTENSIONS {
                let path = PathAbs::new(Path::new(format!("yin.{}", extension).as_str()))?;
                if path.exists() {
                    println!(
                        "Using default input file at {}",
                        path.as_path().to_str().unwrap()
                    );
                    return Ok(path);
                }
            }
            let current_dir = env::current_dir()?;
            let current_dir_path = current_dir.to_str().unwrap();
            Err(Error::new(
                ErrorKind::NotFound,
                format!(
                    "No input file was specified, and no default inputs were found in the current \
                    directory of {}",
                    current_dir_path
                ),
            ))
        }
    }
}

fn parse_input(found_input: PathAbs) -> Result<Vec<Tao>, Error> {
    println!(
        "cargo:rerun-if-changed={}",
        found_input.as_os_str().to_str().unwrap()
    );
    let contents = read_to_string(&found_input)?;
    let extension = found_input
        .extension()
        .map(|e| e.to_str().unwrap())
        .unwrap_or("");
    match extension {
        "md" => Ok(parse_md(&contents)),
        "yaml" => Ok(parse_yaml(&contents)),
        "yml" => Ok(parse_yaml(&contents)),
        _ => Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "The extension \"{}\" is not recognized. Please see the help message for \
                    recognized extension types.",
                extension
            ),
        )),
    }
}

fn run_command<I, S>(command: &str, args: I) -> String
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
    parse_input(found_input)?;

    for implement_command in Implement::archetype().individuals() {
        handle_implementation(Implement::from(implement_command), &build_cfg.codegen_cfg);
    }

    save_autogen();
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
            SubCommand::with_name("build").setting(AppSettings::ColoredHelp)
            .about("Generate code from an input file. ")
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
                        "Whether or not to add an autogeneration comment to each generated line \
                        of code. Defaults to true.",
                    )
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("TRACK_AUTOGEN")
                    .short("t")
                    .long("track-autogen")
                    .help("Whether or not we want Cargo to track autogenerated files and rebuild \
                        when they change. Can result in constant rebuilds."))
            .arg(
                Arg::with_name("YIN")
                    .short("y")
                    .long("yin")
                    .help("Set to generate code for Yin instead.")
            )
        )
        .subcommand(
            SubCommand::with_name("release").setting(AppSettings::ColoredHelp)
            .about("Prepare repo for a Cargo release. ")
            .arg(
                Arg::with_name("INPUT")
                    .value_name("INPUT")
                    .help(INPUT_HELP_TEXT)
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("YIN")
                    .short("y")
                    .long("yin")
                    .help("Set to generate code for Yin instead.")
            )
        )
        .subcommand(SubCommand::with_name("clean")
        .setting(AppSettings::ColoredHelp)
        .about("Clean up autogenerated files."))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    initialize_kb();

    let result = if let Some(build_args) = args.subcommand_matches("build") {
        build(build_args)
    } else if let Some(release_args) = args.subcommand_matches("release") {
        release(release_args)
    } else if let Some(clean_args) = args.subcommand_matches("clean") {
        clean(clean_args)
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
