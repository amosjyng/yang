/// Extensions to Yin archetypes.
pub mod archetype;
/// Callbacks that should be triggered upon certain conditions in Yin being met.
pub mod callbacks;
pub mod relation;
/// Form-related attributes.
pub mod form {
    /// Concepts that exist explicitly as bits.
    pub mod data {
        mod data_extension;

        pub use data_extension::DataExtension;
        pub use zamm_yin::tao::form::data::*;
    }

    mod build_info_extension;
    mod build_info_form;
    mod defined_marker;
    mod module_extension;
    mod module_form;

    pub use build_info_extension::BuildInfoExtension;
    pub use build_info_form::BuildInfo;
    #[allow(deprecated)]
    pub use defined_marker::DefinedMarker;
    pub use module_extension::ModuleExtension;
    pub use module_form::Module;
    pub use zamm_yin::tao::form::*;
}
mod auto_init;
mod implement_extension;
/// Command to implement something.
mod implement_form;
mod lens_form;

use auto_init::initialize_types;
pub use auto_init::YIN_MAX_ID;
use form::BuildInfo;
pub use implement_extension::ImplementExtension;
pub use implement_form::Implement;
pub use lens_form::Lens;
pub use zamm_yin::tao::*;

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
