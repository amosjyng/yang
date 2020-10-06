use zamm_yin::concepts::ArchetypeTrait;
use zamm_yin::wrappers::CommonNodeTrait;

mod concepts;

use concepts::{initialize_kb, Implement};

/// The entry-point to this code generation tool.
fn main() {
    initialize_kb();

    let mut implement_command = Implement::individuate();
    implement_command.set_target(Implement::archetype());
    println!(
        "Will implement {}",
        implement_command.target().unwrap().internal_name().unwrap()
    );
}
