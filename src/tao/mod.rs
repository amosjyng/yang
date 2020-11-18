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
        mod uses_data_logic_form;
        mod uses_root_node_logic_form;

        pub use newly_defined_form::NewlyDefined;
        pub use own_module_form::OwnModule;
        pub use uses_attribute_logic_form::UsesAttributeLogic;
        pub use uses_data_logic_form::UsesDataLogic;
        pub use uses_root_node_logic_form::UsesRootNodeLogic;
    }

    /// Binary relations.
    pub mod attribute {
        mod concept_id_form;
        mod crate_form;
        mod documentation_form;
        mod implementation_name_form;
        mod import_path_form;
        mod rust_primitive_form;
        mod target_form;

        pub use concept_id_form::ConceptId;
        pub use crate_form::Crate;
        pub use documentation_form::Documentation;
        pub use implementation_name_form::ImplementationName;
        pub use import_path_form::ImportPath;
        pub use rust_primitive_form::RustPrimitive;
        pub use target_form::Target;
    }
}
/// Form-related attributes.
pub mod form {
    /// Concepts that exist explicitly as bits.
    pub mod data {
        mod data_extension;

        pub use data_extension::DataExtension;
    }

    mod build_info_extension;
    mod build_info_form;
    mod defined_marker;

    pub use build_info_extension::BuildInfoExtension;
    pub use build_info_form::BuildInfo;
    pub use defined_marker::DefinedMarker;
}
mod auto_init;
mod implement_extension;
/// Command to implement something.
mod implement_form;
mod lens_form;

use auto_init::initialize_types;
use form::BuildInfo;
pub use implement_extension::ImplementExtension;
pub use implement_form::Implement;
pub use lens_form::Lens;
use relation::flag::{
    NewlyDefined, OwnModule, UsesAttributeLogic, UsesDataLogic, UsesRootNodeLogic,
};
pub use auto_init::YIN_MAX_ID;

/// Only here for backwards compatibility reasons.
#[deprecated(
    since = "0.1.1",
    note = "Please use the Implement.document and Implement.set_implementation_id functions \
    instead."
)]
pub struct ImplementConfig;

/// Initialize Yin, including with Yang-specific concepts.
pub fn initialize_kb() {
    zamm_yin::tao::initialize_kb();
    initialize_types();
}
