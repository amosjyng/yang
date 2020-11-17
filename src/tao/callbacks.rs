use super::Implement;
use crate::codegen::planning::{code, file_path};
use crate::codegen::track_autogen::save_autogen;
use crate::codegen::{output_code, CodegenConfig};
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};

/// Handle the implementation request for a new archetype.
pub fn handle_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let code = code(request, codegen_cfg);
    output_code(&code, &file_path(&request.target().unwrap()), codegen_cfg);
}

/// Handle all defined implementation requests.
pub fn handle_all_implementations(codegen_cfg: &CodegenConfig) {
    for implement_command in Implement::archetype().individuals() {
        handle_implementation(Implement::from(implement_command.id()), codegen_cfg);
    }

    save_autogen();
}

#[cfg(test)]
mod tests {
    use crate::codegen::string_format::attribute::code_attribute;
    use crate::codegen::string_format::tao::code_tao;
    use crate::codegen::string_format::{
        code_data_concept, code_form, AttributeFormatConfig, DataFormatConfig, FormatConfig,
    };
    use crate::codegen::StructConfig;
    use std::rc::Rc;

    #[test]
    fn integration_test_attribute_generation() {
        let code = code_attribute(&AttributeFormatConfig {
            tao_cfg: FormatConfig {
                this: StructConfig {
                    name: "MyNewAttr".to_owned(),
                    ..StructConfig::default()
                },
                parent_name: "MyAttr".to_owned(),
                archetype_name: "AttributeArchetype".to_owned(),
                ..FormatConfig::default()
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
            ..AttributeFormatConfig::default()
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
        let code = code_tao(&FormatConfig {
            this: StructConfig {
                name: "MyRoot".to_owned(),
                ..StructConfig::default()
            },
            form: StructConfig {
                name: "MyForm".to_owned(),
                ..StructConfig::default()
            },
            parent_name: "Tao".to_owned(),
            ..FormatConfig::default()
        });
        assert!(!code.contains("impl FormTrait"));
        assert!(code.contains("type Form = MyForm;"));
    }

    #[test]
    fn integration_test_data_generation() {
        let code = code_data_concept(&DataFormatConfig {
            tao_cfg: FormatConfig {
                this: StructConfig {
                    name: "MyStr".to_owned(),
                    ..StructConfig::default()
                },
                parent_name: "MyData".to_owned(),
                ..FormatConfig::default()
            },
            rust_primitive_name: Rc::new("asdf".to_owned()),
            default_value: Rc::new("bsdf".to_owned()),
        });
        assert!(code.contains("impl FormTrait"));
        assert!(code.contains("set_value"));
    }

    #[test]
    fn integration_test_regular_generation() {
        let code = code_form(&FormatConfig {
            this: StructConfig {
                name: "Tao".to_owned(),
                ..StructConfig::default()
            },
            ..FormatConfig::default()
        });
        assert!(code.contains("impl FormTrait"));
        assert!(!code.contains("Attribute"));
    }
}
