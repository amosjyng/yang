use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};
use zamm_yang::codegen::handle_implementation;
use zamm_yang::codegen::track_autogen::{clean_autogen, save_autogen};
use zamm_yang::concepts::{initialize_kb, Documentable, Implement};
use zamm_yin::concepts::{ArchetypeTrait, Tao};
use zamm_yin::node_wrappers::CommonNodeTrait;

fn generate(args: &ArgMatches) {
    let mut target = Tao::archetype().individuate_as_archetype();
    target.set_internal_name(args.value_of("CONCEPT").unwrap().to_string());
    args.value_of("DOC").map(|d| target.set_documentation(d));

    let mut implement_command = Implement::individuate();
    implement_command.set_target(target);

    handle_implementation(
        implement_command,
        args.value_of("ID").unwrap().parse::<usize>().unwrap(),
        args.value_of("COMMENT_AUTOGEN")
            .unwrap_or("true")
            .parse::<bool>()
            .unwrap(),
        args.is_present("YIN"),
    );

    save_autogen();
}

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
            SubCommand::with_name("generate").setting(AppSettings::ColoredHelp)
            .about("Generate a new concept code file")
            .arg(
                Arg::with_name("CONCEPT")
                    .value_name("CONCEPT")
                    .help("Name of concept to generate code for.")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("ID")
                    .short("i")
                    .long("id")
                    .value_name("ID")
                    .help("ID offset from Yin's max id.")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("DOC")
                    .short("d")
                    .long("documentation")
                    .value_name("DOC")
                    .help("Documentation string for concept.")
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
                Arg::with_name("YIN")
                    .short("y")
                    .long("yin")
                    .help("Set to generate code for Yin instead."),
            )
            .setting(AppSettings::ArgRequiredElseHelp)
        )
        .subcommand(SubCommand::with_name("clean")
        .setting(AppSettings::ColoredHelp)
        .about("Clean up autogenerated files."))
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .get_matches();

    initialize_kb();

    if let Some(generate_args) = args.subcommand_matches("generate") {
        generate(generate_args);
    } else if let Some(clean_args) = args.subcommand_matches("clean") {
        clean(clean_args);
    }
}
