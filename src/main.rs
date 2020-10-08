//! Yang is a code generator for [Yin](https://crates.io/crates/zamm_yin).

use clap::clap_app;
use zamm_yin::concepts::{ArchetypeTrait, Tao};
use zamm_yin::wrappers::CommonNodeTrait;

use zamm_yang::codegen::handle_implementation;
use zamm_yang::concepts::{initialize_kb, Documentable, Implement};

/// The entry-point to this code generation tool.
fn main() {
    let args = clap_app!(yang =>
        (version: "0.0.3")
        (author: "Amos Ng <me@amos.ng>")
        (about: "Code generator for Yin.")
        (@arg CONCEPT: +required "Name of concept to generate code for.")
        (@arg ID: -i --id +takes_value +required "ID offset from Yin's max id.")
        (@arg DOC: -d --documentation +takes_value "Documentation string for concept.")
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
    );
}
