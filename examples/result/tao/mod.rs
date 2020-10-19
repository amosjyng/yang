pub mod attribute {
    pub mod target_form;
    pub use target_form::Target;
}

use zamm_yin::graph::{Graph, InjectionGraph};
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::attribute::Inherits;

pub fn initialize_kb() {
    zamm_yin::tao::initialize_kb();
    let mut ig = InjectionGraph::new();
    // Initialize Yin KB with new type
    zamm_yin::initialize_type!(ig, (attribute::Target));
}
