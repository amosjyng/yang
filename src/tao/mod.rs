/// Extensions to Yin archetypes.
pub mod archetype;
/// Yang-specific attribute concepts.
pub mod attribute {
    mod target_form;
    pub use target_form::Target;
}
/// Callbacks that should be triggered upon certain conditions in Yin being met.
pub mod callbacks;
/// Unary relations.
pub mod flag {
    mod newly_defined_form;
    mod own_module_form;
    mod uses_attribute_logic_form;

    pub use newly_defined_form::NewlyDefined;
    pub use own_module_form::OwnModule;
    pub use uses_attribute_logic_form::UsesAttributeLogic;
}
/// Form-related attributes.
pub mod form;
/// Command to implement something.
mod implement;

use attribute::Target;
use flag::{NewlyDefined, OwnModule, UsesAttributeLogic};
pub use implement::{Implement, ImplementConfig};
use zamm_yin::graph::{Graph, InjectionGraph};
use zamm_yin::initialize_type;
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::relation::attribute::Inherits;

/// Initialize Yin, including with Yang-specific concepts.
pub fn initialize_kb() {
    zamm_yin::tao::initialize_kb();
    let mut ig = InjectionGraph::new();
    initialize_type!(
        ig,
        (
            Implement,
            Target,
            UsesAttributeLogic,
            NewlyDefined,
            OwnModule
        )
    );
}
