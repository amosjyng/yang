use zamm_yin::concepts::{ArchetypeTrait, FormTrait, Tao};
use zamm_yin::graph::bind_in_memory_graph;
use zamm_yin::wrappers::CommonNodeTrait;

fn main() {
    // Initialize the knowledge-base
    bind_in_memory_graph();

    // Create a new concept
    let mut concept = Tao::individuate();
    assert!(concept.has_ancestor(Tao::archetype()));

    // Set a name for the concept
    concept.set_internal_name("Hello, world.".to_string());
    println!("{}", concept.internal_name().unwrap());
}
