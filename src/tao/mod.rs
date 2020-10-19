/// Yang-specific attribute concepts.
pub mod attribute;
/// Callbacks that should be triggered upon certain conditions in Yin being met.
pub mod callbacks;
/// Command to implement something.
mod implement;

use attribute::Target;
pub use implement::{Implement, ImplementConfig};
use zamm_yin::graph::{Graph, InjectionGraph};
use zamm_yin::initialize_type;
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::attribute::Inherits;

/// Initialize Yin, including with Yang-specific concepts.
pub fn initialize_kb() {
    zamm_yin::tao::initialize_kb();
    let mut ig = InjectionGraph::new();
    initialize_type!(ig, (Implement, Target));
}
