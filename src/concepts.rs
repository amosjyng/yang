use zamm_yin::concepts::attributes::Inherits;
use zamm_yin::concepts::ArchetypeTrait;
use zamm_yin::graph::bind_in_memory_graph;
use zamm_yin::graph::{Graph, InjectionGraph};
use zamm_yin::initialize_type;

/// Yang-specific attribute concepts.
mod attributes;
/// Concept documentation.
mod documentable;
/// Command to implement something.
mod implement;

use attributes::Target;
pub use documentable::Documentable;
pub use implement::Implement;

/// Initialize Yin, including with Yang-specific concepts.
pub fn initialize_kb() {
    bind_in_memory_graph();
    let mut ig = InjectionGraph::new();
    initialize_type!(ig, (Implement, Target));
}
