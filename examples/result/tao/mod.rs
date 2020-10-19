pub mod attribute {
    pub mod target_form;
    pub use target_form::Target;
}

use zamm_yin::graph::{bind_in_memory_graph, Graph, InjectionGraph};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::attribute::Inherits;

pub fn initialize_kb() {
    // Initialize Yin KB with new type
    bind_in_memory_graph();
    let mut ig = InjectionGraph::new();
    zamm_yin::initialize_type!(ig, (attribute::Target));
}
