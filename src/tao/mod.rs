/// Extensions to Yin archetypes.
pub mod archetype;
/// Yang-specific attribute concepts.
pub mod attribute;
/// Callbacks that should be triggered upon certain conditions in Yin being met.
pub mod callbacks;
/// Data that actually exists as bits on the machine.
mod data_form;
mod flag_form;
/// Form-related attributes.
pub mod form;
/// Command to implement something.
mod implement;
mod lens_form;
mod newly_defined_form;
mod own_module_form;
mod string_concept_form;
mod uses_attribute_logic_form;
mod uses_data_logic_form;

use attribute::{StructPath, Target};
pub use data_form::Data;
pub use flag_form::Flag;
use form::BuildInfo;
pub use implement::{Implement, ImplementConfig};
pub use lens_form::Lens;
pub use newly_defined_form::NewlyDefined;
pub use own_module_form::OwnModule;
pub use string_concept_form::StringConcept;
pub use uses_attribute_logic_form::UsesAttributeLogic;
pub use uses_data_logic_form::UsesDataLogic;
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
            Data,
            StringConcept,
            UsesDataLogic,
            StructPath,
            BuildInfo,
            Lens
        )
    );
}
