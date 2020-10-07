use clap::clap_app;
use zamm_yin::concepts::{ArchetypeTrait, Tao};
use zamm_yin::wrappers::CommonNodeTrait;

mod codegen;
mod concepts;

use concepts::{initialize_kb, Documentable, Implement};

/// The entry-point to this code generation tool.
fn main() {
    let args = clap_app!(yang =>
        (version: "0.0.1")
        (author: "Amos Ng <me@amos.ng>")
        (about: "Yin code generator.")
        (@arg CONCEPT: +required "Concept name to generate code for.")
        (@arg DOC: -d --documentation +takes_value "Documentation string for concept.")
    )
    .get_matches();

    initialize_kb();

    let mut target = Tao::archetype().individuate_as_archetype();
    target.set_internal_name(args.value_of("CONCEPT").unwrap().to_string());
    args.value_of("DOC").map(|d| target.set_documentation(d));

    let mut implement_command = Implement::individuate();
    implement_command.set_target(target);

    match target.documentation() {
        Some(doc) => println!(
            "Will implement {} with documentation \"{}\".",
            implement_command.target().unwrap().internal_name().unwrap(),
            doc
        ),
        None => println!(
            "Will implement {} without documentation.",
            implement_command.target().unwrap().internal_name().unwrap()
        ),
    }
}
