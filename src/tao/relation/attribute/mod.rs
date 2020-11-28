//! Relations between two forms.

mod concept_id_form;
mod documentation_form;
mod dual_purpose_documentation_form;
mod has_member_form;
mod implementation_name_form;
mod import_path_form;
mod most_prominent_member_form;
mod re_exports_form;
mod rust_primitive_form;
mod supports_membership;
mod target_form;
mod version_form;

pub use concept_id_form::ConceptId;
pub use documentation_form::Documentation;
pub use dual_purpose_documentation_form::DualPurposeDocumentation;
pub use has_member_form::HasMember;
pub use implementation_name_form::ImplementationName;
pub use import_path_form::ImportPath;
pub use most_prominent_member_form::MostProminentMember;
pub use re_exports_form::ReExports;
pub use rust_primitive_form::RustPrimitive;
pub use supports_membership::SupportsMembership;
pub use target_form::Target;
pub use version_form::Version;
pub use zamm_yin::tao::relation::attribute::*;
