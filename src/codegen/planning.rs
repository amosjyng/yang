use crate::codegen::string_format::{OWNER_FORM_KEY, VALUE_FORM_KEY};
use crate::codegen::{CodeConfig, CodegenConfig, StructConfig};
use crate::tao::archetype::CodegenFlags;
use crate::tao::form::data::{Data, DataExtension};
use crate::tao::form::{BuildInfo, DefinedMarker};
use crate::tao::Implement;
use heck::{CamelCase, SnakeCase};
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryFrom;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, OwnerArchetype, ValueArchetype};

fn in_own_submodule(target: &Archetype) -> bool {
    // todo: filter by type, once Yin has that functionality
    target.root_node_logic_activated() || !target.child_archetypes().is_empty()
}

fn ancestor_path(target: &Archetype, separator: &str, force_own_module: bool) -> String {
    let build_info = BuildInfo::from(target.id());
    match build_info.import_path() {
        Some(existing_path) => {
            let modules = existing_path.split("::").collect::<Vec<&str>>();
            modules
                .iter()
                .skip(1)
                .take(modules.len() - 2)
                .format(separator)
                .to_string()
        }
        None => {
            // parent path matters because we want to follow whatever convention the parent is
            // following
            let parent_path = if target.root_node_logic_activated() {
                None
            } else {
                // always produce own module for parents, because obviously they have a child in
                // the form of the current concept
                Some(ancestor_path(
                    &target.parents().first().unwrap(),
                    separator,
                    true,
                ))
            };

            let target_name = target
                .internal_name()
                .unwrap()
                .as_str()
                .to_snake_case()
                .to_ascii_lowercase();
            if force_own_module || in_own_submodule(target) {
                match parent_path {
                    Some(actual_parent_path) => {
                        format!("{}{}{}", actual_parent_path, separator, target_name)
                    }
                    None => target_name,
                }
            } else {
                parent_path.unwrap() // if not in own module, then parent must be it
            }
        }
    }
}

/// Get the output path for a given concept.
pub fn file_path(target: &Archetype) -> String {
    let snake_name = target
        .internal_name()
        .unwrap()
        .as_str()
        .to_snake_case()
        .to_ascii_lowercase();
    // append _form to filename to avoid
    // https://rust-lang.github.io/rust-clippy/master/index.html#module_inception
    format!(
        "src/{}/{}_form.rs",
        ancestor_path(target, "/", target.force_own_module()),
        snake_name
    )
}

/// Returns the import path, not including the crate itself.
///
/// `yin_override` needed for now because Yin is not yet fully described by its own yin.md.
/// todo: remove once Yin supports that
fn import_path(target: &Archetype, force_own_module: bool, yin_override: bool) -> String {
    let build_info = BuildInfo::from(target.id());
    match build_info.import_path() {
        Some(existing_path) => existing_path,
        None => {
            let yin_crate = if build_info.crate_name().is_some() {
                build_info.crate_name().unwrap()
            } else if yin_override || target.is_newly_defined() {
                "crate".to_owned()
            } else {
                "zamm_yin".to_owned()
            };
            let struct_name = target.internal_name().unwrap().as_str().to_camel_case();
            format!(
                "{}::{}::{}",
                yin_crate,
                ancestor_path(&target, "::", force_own_module),
                struct_name
            )
        }
    }
}

/// Turns a concept into a struct to be imported.
fn concept_to_struct(target: &Archetype, yin_override: bool) -> StructConfig {
    let build_info = BuildInfo::from(*target.essence());
    let name = build_info
        .implementation_name()
        .unwrap_or_else(|| target.internal_name().unwrap().as_str().to_camel_case());
    StructConfig {
        name,
        import: import_path(target, target.force_own_module(), yin_override),
    }
}

fn or_form_default(archetype: Archetype) -> Archetype {
    if archetype.root_node_logic_activated() {
        Archetype::try_from(Form::TYPE_NAME).unwrap() // allow user to override Form
    } else {
        archetype
    }
}

/// Generate the CodeConfig for a given implementation request.
pub fn code_cfg_for(request: Implement, codegen_cfg: &CodegenConfig) -> CodeConfig {
    let target = request.target().unwrap();
    let target_struct = concept_to_struct(&target, codegen_cfg.yin);
    let form = if target.root_node_logic_activated() {
        // technically we should allow the user to customize this as well
        concept_to_struct(&Form::archetype(), codegen_cfg.yin)
    } else {
        target_struct.clone()
    };
    let ancestors = target.ancestry();
    let parent = ancestors.iter().last().unwrap();
    let parent_struct = concept_to_struct(parent, codegen_cfg.yin);

    let activate_root_node = target.root_node_logic_activated();
    let activate_attribute = target == Attribute::archetype().as_archetype()
        || parent.has_ancestor(Attribute::archetype().as_archetype())
        || target.attribute_logic_activated();
    let activate_data =
        target.has_ancestor(Data::archetype().as_archetype()) || target.data_logic_activated();

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
        target: target_struct,
        form,
        parent: parent_struct,
        activate_root_node,
        activate_attribute,
        activate_data,
        all_attributes,
        introduced_attributes,
        attribute_structs: attr_structs,
        rust_primitive_name: target.rust_primitive().unwrap_or_default(),
        default_value: target.default_value().unwrap_or_default(),
        impl_cfg: request.config().unwrap(),
        codegen_cfg: *codegen_cfg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use crate::tao::ImplementConfig;
    use zamm_yin::tao::relation::attribute::{Attribute, Owner};
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
            "src/tao/relation/attribute/attribute_form.rs"
        );
    }

    #[test]
    fn folder_path_nested() {
        initialize_kb();
        assert_eq!(
            file_path(&Owner::archetype().as_archetype()),
            "src/tao/relation/attribute/owner_form.rs"
        );
    }

    #[test]
    fn folder_path_forced_own_module() {
        initialize_kb();
        let mut owner = Owner::archetype();
        owner.mark_own_module();
        assert_eq!(
            file_path(&owner.as_archetype()),
            "src/tao/relation/attribute/owner/owner_form.rs"
        );
    }

    #[test]
    fn folder_path_custom_module() {
        initialize_kb();
        BuildInfo::from(Attribute::TYPE_ID)
            .set_import_path("zamm_yin::tao::newfangled::module::attribute::Attribute");
        let mut owner = Owner::archetype();
        owner.mark_own_module();
        assert_eq!(
            file_path(&owner.as_archetype()),
            "src/tao/newfangled/module/attribute/owner/owner_form.rs"
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
            "zamm_yin::tao::relation::attribute::Attribute"
        );
    }

    #[test]
    fn import_path_nested() {
        initialize_kb();
        assert_eq!(
            import_path(&Owner::archetype().as_archetype(), false, false),
            "zamm_yin::tao::relation::attribute::Owner"
        );
    }

    #[test]
    fn import_path_forced_own_module() {
        initialize_kb();
        assert_eq!(
            import_path(&Owner::archetype().as_archetype(), true, false),
            "zamm_yin::tao::relation::attribute::owner::Owner"
        );
    }

    #[test]
    fn import_path_newly_defined() {
        initialize_kb();
        let mut owner = Owner::archetype();
        owner.mark_newly_defined();
        assert_eq!(
            import_path(&owner.as_archetype(), false, false),
            "crate::tao::relation::attribute::Owner"
        );
    }

    #[test]
    fn import_path_custom_module() {
        initialize_kb();
        BuildInfo::from(Attribute::TYPE_ID)
            .set_import_path("zamm_yin::tao::newfangled::module::attribute::Attribute");
        let mut owner = Owner::archetype();
        owner.mark_own_module();
        owner.mark_newly_defined();
        assert_eq!(
            import_path(&owner.as_archetype(), true, false),
            "crate::tao::newfangled::module::attribute::owner::Owner"
        );
    }

    #[test]
    fn import_path_custom_crate() {
        initialize_kb();
        BuildInfo::from(Attribute::TYPE_ID)
            .set_import_path("zamm_yin::tao::newfangled::module::attribute::Attribute");
        let mut owner = Owner::archetype();
        owner.mark_own_module();
        owner.mark_newly_defined();
        // possible if we've defined a new type, but we did so only to tell yang that it's already
        // been implemented as part of a dependency
        BuildInfo::from(owner.id()).set_crate_name("mycrate");
        assert_eq!(
            import_path(&owner.as_archetype(), true, false),
            "mycrate::tao::newfangled::module::attribute::owner::Owner"
        );
    }

    #[test]
    fn import_path_multiple_descendants() {
        initialize_kb();
        let mut type1 = Tao::archetype().individuate_as_archetype();
        type1.set_internal_name("hello".to_owned());
        type1.mark_newly_defined();
        let mut type2 = type1.individuate_as_archetype();
        type2.set_internal_name("world".to_owned());
        type2.mark_newly_defined();
        assert_eq!(
            import_path(&type2, true, false),
            "crate::tao::hello::world::World"
        );
    }

    #[test]
    fn import_path_custom_root() {
        initialize_kb();
        let mut root = Tao::archetype().individuate_as_archetype();
        root.set_internal_name("my-root".to_owned());
        root.mark_newly_defined();
        root.activate_root_node_logic();
        assert_eq!(import_path(&root, false, false), "crate::my_root::MyRoot");
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
                import: "zamm_yin::tao::relation::attribute::Attribute".to_owned(),
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
                import: "zamm_yin::tao::relation::attribute::Owner".to_owned(),
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
                import: "zamm_yin::tao::relation::attribute::owner::Owner".to_owned(),
            }
        );
    }

    #[test]
    fn struct_config_newly_defined() {
        initialize_kb();
        let mut owner = Owner::archetype();
        owner.mark_newly_defined();
        assert_eq!(
            concept_to_struct(&owner.as_archetype(), false),
            StructConfig {
                name: "Owner".to_owned(),
                import: "crate::tao::relation::attribute::Owner".to_owned(),
            }
        );
    }

    #[test]
    fn struct_config_override() {
        initialize_kb();
        let mut tao_build = BuildInfo::from(Tao::TYPE_ID);
        tao_build.set_implementation_name("TaoStruct");
        tao_build.set_import_path("crate::TaoStruct");
        assert_eq!(
            concept_to_struct(&Tao::archetype(), false),
            StructConfig {
                name: "TaoStruct".to_owned(),
                import: "crate::TaoStruct".to_owned(),
            }
        );
    }

    #[test]
    fn code_cfg_for_not_activated() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyAttrType".to_owned());
        target.mark_newly_defined();
        let mut implement = Implement::individuate();
        implement.set_target(target);
        implement.set_config(ImplementConfig::default());

        let codegen_cfg = CodegenConfig::default();
        let cfg = code_cfg_for(implement, &codegen_cfg);
        assert!(!cfg.activate_attribute);
        assert!(!cfg.activate_data);
    }

    #[test]
    fn code_cfg_for_root_node_activated() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyRoot".to_owned());
        target.mark_newly_defined();
        target.activate_root_node_logic();
        let mut implement = Implement::individuate();
        implement.set_target(target.as_archetype());
        implement.set_config(ImplementConfig::default());

        assert!(code_cfg_for(implement, &CodegenConfig::default()).activate_root_node);
    }

    #[test]
    fn code_cfg_for_attribute_activated() {
        initialize_kb();
        let mut target = AttributeArchetype::from(Tao::archetype().individuate_as_archetype().id());
        target.set_internal_name("MyAttrType".to_owned());
        target.mark_newly_defined();
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
    fn code_cfg_for_data_activated() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyDataType".to_owned());
        target.mark_newly_defined();
        target.activate_data_logic();
        let mut implement = Implement::individuate();
        implement.set_target(target);
        implement.set_config(ImplementConfig::default());

        assert!(code_cfg_for(implement, &CodegenConfig::default()).activate_data);
    }
}
