//! Relations involving only one form.

mod archetype_analogue_form;
mod attribute_analogue_form;
mod data_analogue_form;
mod imported_form;
mod newly_defined_form;
mod own_module_form;
mod root_analogue_form;
mod root_archetype_analogue_form;
mod unsized_form;

pub use archetype_analogue_form::ArchetypeAnalogue;
pub use attribute_analogue_form::AttributeAnalogue;
pub use data_analogue_form::DataAnalogue;
pub use imported_form::Imported;
pub use newly_defined_form::NewlyDefined;
pub use own_module_form::OwnModule;
pub use root_analogue_form::RootAnalogue;
pub use root_archetype_analogue_form::RootArchetypeAnalogue;
pub use unsized_form::Unsized;
pub use zamm_yin::tao::relation::flag::{Flag, IsIndividual, Meta, MultiValued, Nonhereditary};
