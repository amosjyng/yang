use super::Implement;
use crate::codegen::filesystem::{output_code, OutputConfig};
use crate::codegen::planning::{code_cfg_for, file_path};
use crate::codegen::{code, CodegenConfig};

/// Handle the implementation request for a new attribute archetype.
pub fn handle_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let code = code(&code_cfg_for(request, codegen_cfg));

    output_code(&OutputConfig {
        code: &code,
        file_path: &file_path(&request.target().unwrap()),
        git_ignore: !codegen_cfg.release,
        cargo_track: codegen_cfg.track_autogen,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::string_format::{OWNER_FORM_KEY, VALUE_FORM_KEY};
    use crate::codegen::{CodeConfig, CodegenConfig, StructConfig};
    use std::collections::HashMap;
    use std::rc::Rc;
    use zamm_yin::tao::archetype::ArchetypeTrait;
    use zamm_yin::tao::relation::attribute::{OwnerArchetype, ValueArchetype};

    #[test]
    fn integration_test_attribute_generation() {
        let mut attr_structs = HashMap::new();
        attr_structs.insert(
            OwnerArchetype::TYPE_NAME,
            StructConfig {
                name: "MyOwner".to_owned(),
                import: "zamm_yin::tao::MyOwner".to_owned(),
            },
        );
        attr_structs.insert(
            ValueArchetype::TYPE_NAME,
            StructConfig {
                name: "MyValue".to_owned(),
                import: "zamm_yin::tao::MyValue".to_owned(),
            },
        );
        attr_structs.insert(
            OWNER_FORM_KEY,
            StructConfig {
                name: "MyOwner".to_owned(),
                import: "zamm_yin::tao::MyOwner".to_owned(),
            },
        );
        attr_structs.insert(
            VALUE_FORM_KEY,
            StructConfig {
                name: "MyValue".to_owned(),
                import: "zamm_yin::tao::MyValue".to_owned(),
            },
        );
        let code = code(&CodeConfig {
            name: Rc::new("MyNewAttr".to_owned()),
            parent: StructConfig {
                name: "MyAttr".to_owned(),
                ..StructConfig::default()
            },
            activate_attribute: true,
            attribute_structs: attr_structs,
            codegen_cfg: CodegenConfig {
                comment_autogen: false,
                ..CodegenConfig::default()
            },
            ..CodeConfig::default()
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
    fn integration_test_non_attribute_generation() {
        assert!(!code(&CodeConfig {
            parent: StructConfig {
                name: "Tao".to_owned(),
                ..StructConfig::default()
            },
            ..CodeConfig::default()
        })
        .contains("Attribute"));
    }
}
