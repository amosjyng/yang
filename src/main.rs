use zamm_yin::concepts::ArchetypeTrait;
use zamm_yin::wrappers::CommonNodeTrait;

mod concepts;

use concepts::{initialize_kb, Documentable, Implement};

/// The entry-point to this code generation tool.
fn main() {
    initialize_kb();

    let mut implement = Implement::archetype();
    implement.set_documentation("Yooohooo");

    let mut implement_command = Implement::individuate();
    implement_command.set_target(implement);

    println!(
        "Will implement {} with documentation \"{}\"",
        implement_command.target().unwrap().internal_name().unwrap(),
        implement.documentation().unwrap()
    );
}
