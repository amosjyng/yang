use clap::{App, Arg};
use zamm_yang::codegen::handle_implementation;
use zamm_yang::concepts::{initialize_kb, Documentable, Implement};
use zamm_yin::concepts::{ArchetypeTrait, Tao};
use zamm_yin::wrappers::CommonNodeTrait;

/// The entry-point to this code generation tool.
fn main() {
    // Avoid using clapp_app! macro due to a bug with the short arg name getting assigned only to 
    // 'a'
    let args = App::new("yang")
        .version("0.0.3")
        .author("Amos Ng <me@amos.ng>")
        .about("Code generator for Yin.")
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
                    "Whether or not to add an autogeneration comment to each generated line of \
                    code. Defaults to true.",
                )
                .takes_value(true),
        )
        .get_matches();

    initialize_kb();

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
    );
}
