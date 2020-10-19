use super::Implement;
use crate::codegen::filesystem::{output_code, OutputConfig};
use crate::codegen::string_format::{OWNER_FORM_KEY, VALUE_FORM_KEY};
use crate::codegen::NameTransform;
use crate::codegen::{code, CodeConfig, CodegenConfig, StructConfig};
use itertools::Itertools;
use std::collections::HashMap;
use zamm_yin::concepts::archetype::attribute::AttributeArchetype;
use zamm_yin::concepts::archetype::ArchetypeFormTrait;
use zamm_yin::concepts::attributes::{Attribute, OwnerArchetype, ValueArchetype};
use zamm_yin::concepts::{Archetype, ArchetypeTrait, Form, FormTrait, Tao};
use zamm_yin::node_wrappers::CommonNodeTrait;

fn in_own_submodule(target: &Archetype) -> bool {
    // todo: use children() instead of individuals(), and filter by type, once Yin has that
    // functionality
    target
        .individuals()
        .iter()
        .any(|i| *i != target.ego_death()) // todo: remove once Yin bug fixed
}

fn ancestor_names(target: &Archetype, separator: &str) -> String {
    let ancestors = target.ancestry();
    let mut path = ancestors
        .iter()
        .map(|a| {
            let name = a.internal_name().unwrap();
            // this means that paths will now feature singular instead of plural nouns
            NameTransform::from(name.as_str()).to_snake_case()
        })
        .format(separator)
        .to_string();
    if in_own_submodule(target) {
        if !ancestors.is_empty() {
            path += separator;
        }
        path += &NameTransform::from(target.internal_name().unwrap().as_str()).to_snake_case();
    }
    path.to_ascii_lowercase()
}

fn file_path(target: &Archetype) -> String {
    let snake_name = NameTransform::from(target.internal_name().unwrap().as_str()).to_snake_case();
    // append _form to filename to avoid
    // https://rust-lang.github.io/rust-clippy/master/index.html#module_inception
    format!("src/{}/{}_form.rs", ancestor_names(target, "/"), snake_name).to_ascii_lowercase()
}

/// Returns the import path, not including the crate itself.
fn import_path(target: &Archetype) -> String {
    let struct_name = NameTransform::from(target.internal_name().unwrap().as_str()).to_camel_case();
    format!("{}::{}", ancestor_names(target, "::"), struct_name)
}

/// Turns a concept into a struct to be imported.
fn concept_to_struct(target: &Archetype) -> StructConfig {
    StructConfig {
        name: NameTransform::from(target.internal_name().unwrap().as_str()).to_camel_case(),
        import: import_path(target),
    }
}

/// Handle the implementation request for a new attribute archetype.
pub fn handle_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let target = request.target().unwrap();
    let target_name = target.internal_name().unwrap();
    let ancestors = target.ancestry();
    let parent = ancestors.iter().last().unwrap();
    let parent_struct = concept_to_struct(parent);

    let all_attributes = target
        .attribute_archetypes()
        .iter()
        .map(|a| concept_to_struct(&a.as_archetype()))
        .collect();
    let introduced_attributes = target
        .introduced_attribute_types()
        .iter()
        .map(|a| concept_to_struct(&a.as_archetype()))
        .collect();

    let mut attr_structs = HashMap::new();
    if target == Attribute::archetype() || target.has_ancestor(Attribute::archetype()) {
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

    let code = code(&CodeConfig {
        name: target_name.as_str(),
        parent: parent_struct,
        all_attributes,
        introduced_attributes,
        attribute_structs: attr_structs,
        impl_cfg: request.config().unwrap(),
        codegen_cfg: *codegen_cfg,
    });

    let file_path = file_path(&target);
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
    use crate::concepts::initialize_kb;
    use zamm_yin::concepts::attributes::{Attribute, Owner};
    use zamm_yin::concepts::{ArchetypeTrait, Tao};

    #[test]
    fn own_submodule_tao() {
        initialize_kb();
        assert!(in_own_submodule(&Tao::archetype()));
    }

    #[test]
    fn own_submodule_attributes() {
        initialize_kb();
        assert!(in_own_submodule(&Attribute::archetype()));
    }

    #[test]
    fn own_submodule_nested() {
        initialize_kb();
        assert!(!in_own_submodule(&Owner::archetype()));
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
            file_path(&Attribute::archetype()),
            "src/tao/attribute/attribute_form.rs"
        );
    }

    #[test]
    fn folder_path_nested() {
        initialize_kb();
        assert_eq!(
            file_path(&Owner::archetype()),
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
            import_path(&Attribute::archetype()),
            "tao::attribute::Attribute"
        );
    }

    #[test]
    fn import_path_nested() {
        initialize_kb();
        assert_eq!(import_path(&Owner::archetype()), "tao::attribute::Owner");
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
            parent: StructConfig {
                name: "Attribute".to_owned(),
                ..StructConfig::default()
            },
            attribute_structs: attr_structs,
            ..CodeConfig::default()
        });
        assert!(code.contains("type OwnerForm = MyOwner"));
        assert!(code.contains("type ValueForm = MyValue"));
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
