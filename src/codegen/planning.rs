use crate::codegen::string_format::{OWNER_FORM_KEY, VALUE_FORM_KEY};
use crate::codegen::{CodeConfig, CodegenConfig, StructConfig};
use crate::tao::archetype::CodegenFlags;
use crate::tao::form::DefinedMarker;
use crate::tao::Implement;
use heck::{CamelCase, SnakeCase};
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryFrom;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::attribute::{Attribute, OwnerArchetype, ValueArchetype};
use zamm_yin::tao::{Form, FormTrait, Tao};

fn in_own_submodule(target: &Archetype) -> bool {
    // todo: use children() instead of individuals(), and filter by type, once Yin has that
    // functionality
    !target.child_archetypes().is_empty() // todo: remove once Yin bug fixed
}

fn ancestor_names(target: &Archetype, separator: &str, force_own_module: bool) -> String {
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
    if force_own_module || in_own_submodule(target) {
        if !ancestors.is_empty() {
            path += separator;
        }
        path += &target.internal_name().unwrap().as_str().to_snake_case();
    }
    path.to_ascii_lowercase()
}

/// Get the output path for a given concept.
pub fn file_path(target: &Archetype) -> String {
    let snake_name = target.internal_name().unwrap().as_str().to_snake_case();
    // append _form to filename to avoid
    // https://rust-lang.github.io/rust-clippy/master/index.html#module_inception
    format!(
        "src/{}/{}_form.rs",
        ancestor_names(target, "/", target.force_own_module()),
        snake_name
    )
    .to_ascii_lowercase()
}

/// Returns the import path, not including the crate itself.
///
/// `yin_override` needed for now because Yin is not yet fully described by its own yin.md.
/// todo: remove once Yin supports that
fn import_path(target: &Archetype, force_own_module: bool, yin_override: bool) -> String {
    let yin_crate = if yin_override || target.as_form().is_newly_defined() {
        "crate"
    } else {
        "zamm_yin"
    };
    let struct_name = target.internal_name().unwrap().as_str().to_camel_case();
    format!(
        "{}::{}::{}",
        yin_crate,
        ancestor_names(&target, "::", force_own_module),
        struct_name
    )
}

/// Turns a concept into a struct to be imported.
fn concept_to_struct(target: &Archetype, yin_override: bool) -> StructConfig {
    StructConfig {
        name: target.internal_name().unwrap().as_str().to_camel_case(),
        import: import_path(target, target.force_own_module(), yin_override),
    }
}

fn or_form_default(archetype: Archetype) -> Archetype {
    if archetype == Tao::archetype() {
        Archetype::try_from(Form::TYPE_NAME).unwrap() // allow user to override Form
    } else {
        archetype
    }
}

/// Generate the CodeConfig for a given implementation request.
pub fn code_cfg_for(request: Implement, codegen_cfg: &CodegenConfig) -> CodeConfig {
    let target = request.target().unwrap();
    let target_name = target.internal_name().unwrap();
    let ancestors = target.ancestry();
    let parent = ancestors.iter().last().unwrap();
    let parent_struct = concept_to_struct(parent, codegen_cfg.yin);
    let activate_attribute = target == Attribute::archetype().as_archetype()
        || parent.has_ancestor(Attribute::archetype().as_archetype())
        || target.attribute_logic_activated();

    let all_attributes = target
        .attribute_archetypes()
        .iter()
        .map(|a| concept_to_struct(&a.as_archetype(), codegen_cfg.yin))
        .collect();
    let introduced_attributes = target
        .introduced_attribute_archetypes()
        .iter()
        .map(|a| concept_to_struct(&a.as_archetype(), codegen_cfg.yin))
        .collect();

    let mut attr_structs = HashMap::new();
    if activate_attribute {
        let target_attr = AttributeArchetype::from(target.id());
        let owner_type = target_attr.owner_archetype();
        let value_type = target_attr.value_archetype();
        attr_structs.insert(
            OwnerArchetype::TYPE_NAME,
            concept_to_struct(&owner_type, codegen_cfg.yin),
        );
        attr_structs.insert(
            ValueArchetype::TYPE_NAME,
            concept_to_struct(&value_type, codegen_cfg.yin),
        );

        let owner_form = or_form_default(owner_type);
        let value_form = or_form_default(value_type);

        attr_structs.insert(
            OWNER_FORM_KEY,
            concept_to_struct(&owner_form, codegen_cfg.yin),
        );
        attr_structs.insert(
            VALUE_FORM_KEY,
            concept_to_struct(&value_form, codegen_cfg.yin),
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use crate::tao::ImplementConfig;
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
    fn folder_path_forced_own_module() {
        initialize_kb();
        let mut owner = Owner::archetype();
        owner.mark_own_module();
        assert_eq!(
            file_path(&owner.as_archetype()),
            "src/tao/attribute/owner/owner_form.rs"
        );
    }

    #[test]
    fn import_path_tao() {
        initialize_kb();
        assert_eq!(
            import_path(&Tao::archetype(), false, false),
            "zamm_yin::tao::Tao"
        );
    }

    #[test]
    fn import_path_attributes() {
        initialize_kb();
        assert_eq!(
            import_path(&Attribute::archetype().as_archetype(), false, false),
            "zamm_yin::tao::attribute::Attribute"
        );
    }

    #[test]
    fn import_path_nested() {
        initialize_kb();
        assert_eq!(
            import_path(&Owner::archetype().as_archetype(), false, false),
            "zamm_yin::tao::attribute::Owner"
        );
    }

    #[test]
    fn import_path_forced_own_module() {
        initialize_kb();
        assert_eq!(
            import_path(&Owner::archetype().as_archetype(), true, false),
            "zamm_yin::tao::attribute::owner::Owner"
        );
    }

    #[test]
    fn import_path_newly_defined() {
        initialize_kb();
        let owner = Owner::archetype();
        owner.as_form().mark_newly_defined();
        assert_eq!(
            import_path(&owner.as_archetype(), false, false),
            "crate::tao::attribute::Owner"
        );
    }

    #[test]
    fn struct_config_tao() {
        initialize_kb();
        assert_eq!(
            concept_to_struct(&Tao::archetype(), false),
            StructConfig {
                name: "Tao".to_owned(),
                import: "zamm_yin::tao::Tao".to_owned(),
            }
        );
    }

    #[test]
    fn struct_config_attributes() {
        initialize_kb();
        assert_eq!(
            concept_to_struct(&Attribute::archetype().as_archetype(), false),
            StructConfig {
                name: "Attribute".to_owned(),
                import: "zamm_yin::tao::attribute::Attribute".to_owned(),
            }
        );
    }

    #[test]
    fn struct_config_nested() {
        initialize_kb();
        assert_eq!(
            concept_to_struct(&Owner::archetype().as_archetype(), false),
            StructConfig {
                name: "Owner".to_owned(),
                import: "zamm_yin::tao::attribute::Owner".to_owned(),
            }
        );
    }

    #[test]
    fn struct_config_forced_own_module() {
        initialize_kb();
        let mut owner = Owner::archetype();
        owner.mark_own_module();
        assert_eq!(
            concept_to_struct(&owner.as_archetype(), false),
            StructConfig {
                name: "Owner".to_owned(),
                import: "zamm_yin::tao::attribute::owner::Owner".to_owned(),
            }
        );
    }

    #[test]
    fn struct_config_newly_defined() {
        initialize_kb();
        let owner = Owner::archetype();
        owner.as_form().mark_newly_defined();
        assert_eq!(
            concept_to_struct(&owner.as_archetype(), false),
            StructConfig {
                name: "Owner".to_owned(),
                import: "crate::tao::attribute::Owner".to_owned(),
            }
        );
    }

    #[test]
    fn code_cfg_for_attribute_not_activated() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyAttrType".to_owned());
        target.as_form().mark_newly_defined();
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
        target.as_form().mark_newly_defined();
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
}
