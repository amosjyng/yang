use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use path_abs::{PathAbs, PathInfo};
use std::env;
use std::fs::read_to_string;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::exit;
use zamm_yang::codegen::track_autogen::{clean_autogen, save_autogen};
use zamm_yang::codegen::{handle_implementation, CodegenConfig};
use zamm_yang::concepts::initialize_kb;
use zamm_yang::concepts::Implement;
use zamm_yang::parse::{parse_md, parse_yaml};
use zamm_yin::concepts::ArchetypeTrait;

/// All supported input filename extensions.
const SUPPORTED_EXTENSIONS: &[&str] = &["md", "yml", "yaml"];

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
                    format!("Specified input file was not found at {}", path_str).to_string(),
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

/// Generate code from the input file.
fn build(args: &ArgMatches) -> Result<(), Error> {
    let found_input = find_file(args.value_of("INPUT"))?;
    let contents = read_to_string(&found_input)?;
    let extension = found_input
        .extension()
        .map(|e| e.to_str().unwrap())
        .unwrap_or("");
    let found_parser = match extension {
        "md" => {
            parse_md(&contents);
            true
        }
        "yaml" => {
            parse_yaml(&contents);
            true
        }
        "yml" => {
            parse_yaml(&contents);
            true
        }
        _ => false,
    };

    if !found_parser {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "The extension \"{}\" is not recognized. Please see the help message for \
                recognized extension types.",
                extension
            ),
        ));
    }

    for implement_command in Implement::archetype().individuals() {
        handle_implementation(
            Implement::from(implement_command),
            &CodegenConfig {
                comment_autogen: args
                    .value_of("COMMENT_AUTOGEN")
                    .unwrap_or("true")
                    .parse::<bool>()
                    .unwrap(),
                track_autogen: args.is_present("TRACK_AUTOGEN"),
                yin: args.is_present("YIN"),
            },
        )
    }

    save_autogen();
    Ok(())
}

/// Clean all autogenerated files.
fn clean(_: &ArgMatches) {
    clean_autogen();
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
                    .help("The input file containing relevant information to generate code for. \
                    Currently only Markdown (extension .md) and YAML (extensions .yml or .yaml) \
                    are supported. If no input file is provided, yang will look for a file named \
                    `yin` with one of the above extensions, in the current directory.")
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
        .subcommand(SubCommand::with_name("clean")
        .setting(AppSettings::ColoredHelp)
        .about("Clean up autogenerated files."))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    initialize_kb();

    if let Some(generate_args) = args.subcommand_matches("build") {
        exit(match build(generate_args) {
            Ok(_) => 0,
            Err(e) => {
                eprintln!("{}", e);
                1
            }
        })
    } else if let Some(clean_args) = args.subcommand_matches("clean") {
        clean(clean_args);
    } else {
        panic!("Arg not found. Did you reconfigure clap recently?");
    }
}
