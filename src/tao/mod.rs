/// Extensions to Yin archetypes.
pub mod archetype;
/// Callbacks that should be triggered upon certain conditions in Yin being met.
pub mod callbacks;
/// Yang-specific relations that link any number of nodes together 
pub mod relation {
    /// Unary relations.
    pub mod flag {
        mod newly_defined_form;
        mod own_module_form;
        mod uses_attribute_logic_form;

        pub use newly_defined_form::NewlyDefined;
        pub use own_module_form::OwnModule;
        pub use uses_attribute_logic_form::UsesAttributeLogic;
    }

    /// Binary relations.
    pub mod attribute {
        mod crate_form;
        mod implementation_name_form;
        mod import_path_form;
        mod target_form;

        pub use crate_form::Crate;
        pub use implementation_name_form::ImplementationName;
        pub use import_path_form::ImportPath;
        pub use target_form::Target;
    }
}
/// Data that actually exists as bits on the machine.
mod data_form;
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

use relation::flag::{NewlyDefined, OwnModule, UsesAttributeLogic};
pub use implement::{Implement, ImplementConfig};
use relation::attribute::{Crate, ImplementationName, ImportPath, Target};
pub use data_form::Data;
use form::BuildInfo;
pub use implement::{Implement, ImplementConfig};
pub use lens_form::Lens;
pub use string_concept_form::StringConcept;
pub use uses_data_logic_form::UsesDataLogic;
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
            OwnModule,
            Data,
            StringConcept,
            UsesDataLogic,
            ImportPath,
            BuildInfo,
            Lens,
            Crate,
            ImplementationName
        )
    );
}
