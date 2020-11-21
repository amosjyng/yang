pub mod archetype;
mod auto_init;
/// Callbacks that should be triggered upon certain conditions in Yin being met.
pub mod callbacks;
pub mod form;
mod implement_extension;
mod implement_form;
mod lens_form;
pub mod relation;

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
