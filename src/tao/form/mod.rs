//! All things that can be interacted with have form.

pub mod data;

mod build_info_extension;
mod build_info_form;
mod crate_extension;
mod crate_form;
mod defined_marker;
mod module_extension;
mod module_form;

pub use build_info_extension::BuildInfoExtension;
pub use build_info_form::BuildInfo;
pub use crate_extension::CrateExtension;
pub use crate_form::Crate;
pub use defined_marker::DefinedMarker;
pub use module_extension::ModuleExtension;
pub use module_form::Module;
pub use zamm_yin::tao::form::*;
