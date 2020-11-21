use super::{Implement, ImplementExtension};
use crate::codegen::planning::{
    archetype_file_path, code_archetype, code_module, handle_init, module_file_path,
};
use crate::codegen::track_autogen::save_autogen;
use crate::codegen::{output_code, CodegenConfig};
use crate::tao::form::{Module, ModuleExtension};
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;

/// Retrieve implementation requests that pertain to archetypes.
fn archetypes_to_implement() -> Vec<Implement> {
    Implement::archetype()
        .individuals()
        .into_iter()
        .map(|i| Implement::from(i.id()))
        .filter(|i| !i.target().unwrap().has_ancestor(Module::archetype()))
        .collect()
}

/// Retrieve implementation requests that pertain to modules.
fn modules_to_implement() -> Vec<Implement> {
    Implement::archetype()
        .individuals()
        .into_iter()
        .map(|i| Implement::from(i.id()))
        .filter(|i| i.target().unwrap().has_ancestor(Module::archetype()))
        .collect()
}

/// Handle the implementation request for a new archetype.
fn handle_archetype_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let code = code_archetype(request, codegen_cfg);
    let target_type = Archetype::from(request.target().unwrap().id());
    output_code(&code, &archetype_file_path(&target_type), codegen_cfg);
}

/// Handle the implementation request for a new module.
fn handle_module_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let target_module = Module::from(request.target().unwrap().id());
    let primary_archetype = Archetype::from(target_module.most_prominent_member().unwrap().id());
    let code = code_module(request, target_module, primary_archetype);
    output_code(&code, &module_file_path(&primary_archetype), codegen_cfg);
}

/// Handle all defined implementation requests.
pub fn handle_all_implementations(codegen_cfg: &CodegenConfig) {
    let archetype_requests = archetypes_to_implement();
    // handle initialization first to ensure all concepts land with the right concept IDs
    handle_init(&archetype_requests, codegen_cfg);
    for implement_command in archetype_requests {
        handle_archetype_implementation(implement_command, codegen_cfg);
    }
    for implement_command in modules_to_implement() {
        handle_module_implementation(implement_command, codegen_cfg);
    }

    save_autogen();
}

#[cfg(test)]
mod tests {
    use crate::codegen::template::concept::attribute::{code_attribute, AttributeFormatConfig};
    use crate::codegen::template::concept::data::{code_data_concept, DataFormatConfig};
    use crate::codegen::template::concept::form::code_form;
    use crate::codegen::template::concept::tao::{code_tao, TaoConfig};
    use crate::codegen::StructConfig;
    use std::rc::Rc;

    #[test]
    fn integration_test_attribute_generation() {
        let code = code_attribute(&AttributeFormatConfig {
            tao_cfg: TaoConfig {
                this: StructConfig {
                    name: "MyNewAttr".to_owned(),
                    ..StructConfig::default()
                },
                parent_name: "MyAttr".to_owned(),
                archetype_name: "AttributeArchetype".to_owned(),
                ..TaoConfig::default()
            },
            owner_type: StructConfig {
                name: "MyOwner".to_owned(),
                import: "zamm_yin::tao::MyOwner".to_owned(),
            },
            owner_form: StructConfig {
                name: "MyOwner".to_owned(),
                import: "zamm_yin::tao::MyOwner".to_owned(),
            },
            value_type: StructConfig {
                name: "MyValue".to_owned(),
                import: "zamm_yin::tao::MyValue".to_owned(),
            },
            value_form: StructConfig {
                name: "MyValue".to_owned(),
                import: "zamm_yin::tao::MyValue".to_owned(),
            },
        });

        assert!(code.contains("AttributeArchetype"));
        assert!(code.contains("type OwnerForm = MyOwner"));
        assert!(code.contains("type ValueForm = MyValue"));
        assert!(code.contains("check_attribute_constraints"));
        assert!(code.contains(
            "assert_eq!(
            MyNewAttr::archetype().owner_archetype(),
            MyOwner::archetype().as_archetype()
        )"
        ));
    }

    #[test]
    fn integration_test_root_node_generation() {
        let code = code_tao(&TaoConfig {
            this: StructConfig {
                name: "MyRoot".to_owned(),
                ..StructConfig::default()
            },
            form: StructConfig {
                name: "MyForm".to_owned(),
                ..StructConfig::default()
            },
            parent_name: "Tao".to_owned(),
            ..TaoConfig::default()
        });
        assert!(!code.contains("impl FormTrait"));
        assert!(code.contains("type Form = MyForm;"));
    }

    #[test]
    fn integration_test_data_generation() {
        let code = code_data_concept(&DataFormatConfig {
            tao_cfg: TaoConfig {
                this: StructConfig {
                    name: "MyStr".to_owned(),
                    ..StructConfig::default()
                },
                parent_name: "MyData".to_owned(),
                ..TaoConfig::default()
            },
            rust_primitive_name: Rc::from("asdf"),
            default_value: Rc::from("bsdf"),
        });
        assert!(code.contains("impl FormTrait"));
        assert!(code.contains("set_value"));
    }

    #[test]
    fn integration_test_regular_generation() {
        let code = code_form(&TaoConfig {
            this: StructConfig {
                name: "Tao".to_owned(),
                ..StructConfig::default()
            },
            ..TaoConfig::default()
        });
        assert!(code.contains("impl FormTrait"));
        assert!(!code.contains("Attribute"));
    }
}
