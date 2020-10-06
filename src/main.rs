use zamm_yin::concepts::{ArchetypeTrait, FormTrait};
use zamm_yin::wrappers::CommonNodeTrait;

mod concepts;

use concepts::{initialize_kb, Implement};

/// The entry-point to this code generation tool.
fn main() {
    // Initialize the knowledge-base
    initialize_kb();

    // Create a new concept
    let mut concept = Implement::individuate();
    assert!(concept.has_ancestor(Implement::archetype()));

    // Set a name for the concept
    concept.set_internal_name("Hello, world.".to_string());
    println!("{}", concept.internal_name().unwrap());
}
