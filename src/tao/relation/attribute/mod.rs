//! Relations between two forms.

pub mod has_property;

mod alias_form;
mod concept_id_form;
mod default_value_form;
mod documentation_form;
mod dual_purpose_documentation_form;
mod dummy_value_form;
mod implementation_name_form;
mod import_path_form;
mod member_form;
mod most_prominent_member_form;
mod re_export_form;
mod rust_primitive_form;
mod supports_membership;
mod target_form;
mod unboxed_representation_form;
mod version_form;

pub use alias_form::Alias;
pub use concept_id_form::ConceptId;
pub use default_value_form::DefaultValue;
pub use documentation_form::Documentation;
pub use dual_purpose_documentation_form::DualPurposeDocumentation;
pub use dummy_value_form::DummyValue;
pub use implementation_name_form::ImplementationName;
pub use import_path_form::ImportPath;
pub use member_form::Member;
pub use most_prominent_member_form::MostProminentMember;
pub use re_export_form::ReExport;
pub use rust_primitive_form::RustPrimitive;
pub use supports_membership::SupportsMembership;
pub use target_form::Target;
pub use unboxed_representation_form::UnboxedRepresentation;
pub use version_form::Version;
pub use zamm_yin::tao::relation::attribute::{
    Attribute, AttributeTrait, Inherits, MetaForm, Owner, OwnerArchetype, Value, ValueArchetype,
};
