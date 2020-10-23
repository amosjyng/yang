/// Extensions to Yin archetypes.
pub mod archetype;
/// Yang-specific attribute concepts.
pub mod attribute;
/// Callbacks that should be triggered upon certain conditions in Yin being met.
pub mod callbacks;
/// Data that actually exists as bits on the machine.
pub mod data {
    mod data_form;
    pub use data_form::Data;
}
mod flag_form;
/// Form-related attributes.
pub mod form;
/// Command to implement something.
mod implement;
mod newly_defined_form;
mod own_module_form;
mod uses_attribute_logic_form;

use attribute::Target;
use data::Data;
pub use flag_form::Flag;
pub use implement::{Implement, ImplementConfig};
pub use newly_defined_form::NewlyDefined;
pub use own_module_form::OwnModule;
pub use uses_attribute_logic_form::UsesAttributeLogic;
use zamm_yin::graph::{Graph, InjectionGraph};
use zamm_yin::initialize_type;
use zamm_yin::tao::archetype::ArchetypeTrait;
use zamm_yin::tao::attribute::Inherits;

/// Initialize Yin, including with Yang-specific concepts.
pub fn initialize_kb() {
    zamm_yin::tao::initialize_kb();
    let mut ig = InjectionGraph::new();
    initialize_type!(
        ig,
        (
            Implement,
            Target,
            Flag,
            UsesAttributeLogic,
            NewlyDefined,
            OwnModule,
            Data
        )
    );
}
