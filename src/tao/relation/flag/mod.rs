//! Relations involving only one form.

mod attribute_analogue_form;
mod data_analogue_form;
mod newly_defined_form;
mod own_module_form;
mod uses_root_node_logic_form;

pub use attribute_analogue_form::AttributeAnalogue;
pub use data_analogue_form::DataAnalogue;
pub use newly_defined_form::NewlyDefined;
pub use own_module_form::OwnModule;
pub use uses_root_node_logic_form::UsesRootNodeLogic;
pub use zamm_yin::tao::relation::flag::*;
