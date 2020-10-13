/// Yang-specific attribute concepts.
mod attributes;
/// Callbacks that should be triggered upon certain conditions in Yin being met.
pub mod callbacks;
/// Concept documentation.
mod documentable;
/// Command to implement something.
mod implement;

use attributes::{HasAttributeType, Target};
pub use documentable::{set_documentation, Documentable};
pub use implement::{Implement, ImplementConfig};
use zamm_yin::concepts::attributes::Inherits;
use zamm_yin::concepts::ArchetypeTrait;
use zamm_yin::graph::bind_in_memory_graph;
use zamm_yin::graph::{Graph, InjectionGraph};
use zamm_yin::initialize_type;

/// Initialize Yin, including with Yang-specific concepts.
pub fn initialize_kb() {
    bind_in_memory_graph();
    let mut ig = InjectionGraph::new();
    initialize_type!(ig, (Implement, Target, HasAttributeType));
}
