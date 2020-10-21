use super::Implement;
use crate::codegen::filesystem::{output_code, OutputConfig};
use crate::codegen::string_format::{OWNER_FORM_KEY, VALUE_FORM_KEY};
use crate::codegen::{code, CodeConfig, CodegenConfig, StructConfig};
use crate::tao::archetype::CodegenFlags;
use heck::{CamelCase, SnakeCase};
use itertools::Itertools;
use std::collections::HashMap;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::attribute::{Attribute, OwnerArchetype, ValueArchetype};
use zamm_yin::tao::{Form, FormTrait, Tao};

fn in_own_submodule(target: &Archetype) -> bool {
    // todo: use children() instead of individuals(), and filter by type, once Yin has that
    // functionality
    target.individuals().iter().any(|i| *i != target.as_form()) // todo: remove once Yin bug fixed
}

fn ancestor_names(target: &Archetype, separator: &str) -> String {
    let ancestors = target.ancestry();
    let mut path = ancestors
        .iter()
        .map(|a| {
            let name = a.internal_name().unwrap();
            // this means that paths will now feature singular instead of plural nouns
            name.as_str().to_snake_case()
        })
        .format(separator)
        .to_string();
    if in_own_submodule(target) {
        if !ancestors.is_empty() {
            path += separator;
        }
        path += &target.internal_name().unwrap().as_str().to_snake_case();
    }
    path.to_ascii_lowercase()
}

fn file_path(target: &Archetype) -> String {
    let snake_name = target.internal_name().unwrap().as_str().to_snake_case();
    // append _form to filename to avoid
    // https://rust-lang.github.io/rust-clippy/master/index.html#module_inception
    format!("src/{}/{}_form.rs", ancestor_names(target, "/"), snake_name).to_ascii_lowercase()
}

/// Returns the import path, not including the crate itself.
fn import_path(target: &Archetype) -> String {
    let struct_name = target.internal_name().unwrap().as_str().to_camel_case();
    format!("{}::{}", ancestor_names(target, "::"), struct_name)
}

/// Turns a concept into a struct to be imported.
fn concept_to_struct(target: &Archetype) -> StructConfig {
    StructConfig {
        name: target.internal_name().unwrap().as_str().to_camel_case(),
        import: import_path(target),
    }
}

fn code_cfg_for(request: Implement, codegen_cfg: &CodegenConfig) -> CodeConfig {
    let target = request.target().unwrap();
    let target_name = target.internal_name().unwrap();
    let ancestors = target.ancestry();
    let parent = ancestors.iter().last().unwrap();
    let parent_struct = concept_to_struct(parent);
    let activate_attribute = target == Attribute::archetype().as_archetype()
        || parent.has_ancestor(Attribute::archetype().as_archetype())
        || target.attribute_logic_activated();

    let all_attributes = target
        .attribute_archetypes()
        .iter()
        .map(|a| concept_to_struct(&a.as_archetype()))
        .collect();
    let introduced_attributes = target
        .introduced_attribute_archetypes()
        .iter()
        .map(|a| concept_to_struct(&a.as_archetype()))
        .collect();

    let mut attr_structs = HashMap::new();
    if activate_attribute {
        let target_attr = AttributeArchetype::from(target.id());
        let owner_type = target_attr.owner_archetype();
        let value_type = target_attr.value_archetype();
        attr_structs.insert(OwnerArchetype::TYPE_NAME, concept_to_struct(&owner_type));
        attr_structs.insert(ValueArchetype::TYPE_NAME, concept_to_struct(&value_type));

        let owner_form = if owner_type == Tao::archetype() {
            Form::archetype()
        } else {
            owner_type
        };
        let value_form = if value_type == Tao::archetype() {
            Form::archetype()
        } else {
            value_type
        };

        attr_structs.insert(OWNER_FORM_KEY, concept_to_struct(&owner_form));
        attr_structs.insert(VALUE_FORM_KEY, concept_to_struct(&value_form));
    }

    CodeConfig {
        name: target_name,
        parent: parent_struct,
        activate_attribute,
        all_attributes,
        introduced_attributes,
        attribute_structs: attr_structs,
        impl_cfg: request.config().unwrap(),
        codegen_cfg: *codegen_cfg,
    }
}

/// Handle the implementation request for a new attribute archetype.
pub fn handle_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let code = code(&code_cfg_for(request, codegen_cfg));

    let file_path = file_path(&request.target().unwrap());
    output_code(&OutputConfig {
        code: &code,
        file_path: &file_path,
        git_ignore: !codegen_cfg.release,
        cargo_track: codegen_cfg.track_autogen,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::string_format::{OWNER_FORM_KEY, VALUE_FORM_KEY};
    use crate::tao::initialize_kb;
    use crate::tao::ImplementConfig;
    use std::rc::Rc;
    use zamm_yin::tao::attribute::{Attribute, Owner};
    use zamm_yin::tao::Tao;

    #[test]
    fn own_submodule_tao() {
        initialize_kb();
        assert!(in_own_submodule(&Tao::archetype()));
    }

    #[test]
    fn own_submodule_attributes() {
        initialize_kb();
        assert!(in_own_submodule(&Attribute::archetype().as_archetype()));
    }

    #[test]
    fn own_submodule_nested() {
        initialize_kb();
        assert!(!in_own_submodule(&Owner::archetype().as_archetype()));
    }

    #[test]
    fn folder_path_tao() {
        initialize_kb();
        assert_eq!(file_path(&Tao::archetype()), "src/tao/tao_form.rs");
    }

    #[test]
    fn folder_path_attributes() {
        initialize_kb();
        assert_eq!(
            file_path(&Attribute::archetype().as_archetype()),
            "src/tao/attribute/attribute_form.rs"
        );
    }

    #[test]
    fn folder_path_nested() {
        initialize_kb();
        assert_eq!(
            file_path(&Owner::archetype().as_archetype()),
            "src/tao/attribute/owner_form.rs"
        );
    }

    #[test]
    fn import_path_tao() {
        initialize_kb();
        assert_eq!(import_path(&Tao::archetype()), "tao::Tao");
    }

    #[test]
    fn import_path_attributes() {
        initialize_kb();
        assert_eq!(
            import_path(&Attribute::archetype().as_archetype()),
            "tao::attribute::Attribute"
        );
    }

    #[test]
    fn import_path_nested() {
        initialize_kb();
        assert_eq!(
            import_path(&Owner::archetype().as_archetype()),
            "tao::attribute::Owner"
        );
    }

    #[test]
    fn code_cfg_for_attribute_not_activated() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyAttrType".to_owned());
        let mut implement = Implement::individuate();
        implement.set_target(target);
        implement.set_config(ImplementConfig::default());

        assert!(!code_cfg_for(implement, &CodegenConfig::default()).activate_attribute);
    }

    #[test]
    fn code_cfg_for_attribute_activated() {
        initialize_kb();
        let mut target = AttributeArchetype::from(Tao::archetype().individuate_as_archetype().id());
        target.set_internal_name("MyAttrType".to_owned());
        target.activate_attribute_logic();
        target.set_owner_archetype(Tao::archetype());
        target.set_value_archetype(Form::archetype());
        let mut implement = Implement::individuate();
        implement.set_target(target.as_archetype());
        implement.set_config(ImplementConfig::default());

        let codegen_cfg = CodegenConfig::default();
        let cfg = code_cfg_for(implement, &codegen_cfg);
        assert!(cfg.activate_attribute);
        assert_eq!(
            cfg.attribute_structs
                .get(OwnerArchetype::TYPE_NAME)
                .map(|a| a.name.as_str()),
            Some("Tao")
        );
        assert_eq!(
            cfg.attribute_structs
                .get(ValueArchetype::TYPE_NAME)
                .map(|a| a.name.as_str()),
            Some("Form")
        );
    }

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
