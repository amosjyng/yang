//! Relations involving only one form.

mod newly_defined_form;
mod nonhereditary_form;
mod own_module_form;
mod uses_attribute_logic_form;
mod uses_data_logic_form;
mod uses_root_node_logic_form;

pub use newly_defined_form::NewlyDefined;
pub use nonhereditary_form::Nonhereditary;
pub use own_module_form::OwnModule;
pub use uses_attribute_logic_form::UsesAttributeLogic;
pub use uses_data_logic_form::UsesDataLogic;
pub use uses_root_node_logic_form::UsesRootNodeLogic;
pub use zamm_yin::tao::relation::flag::*;
