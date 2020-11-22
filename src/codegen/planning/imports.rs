use crate::codegen::StructConfig;
use crate::tao::archetype::CodegenFlags;
use crate::tao::form::{BuildInfo, BuildInfoExtension};
use heck::{CamelCase, SnakeCase};
use itertools::Itertools;
use std::rc::Rc;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait};
use zamm_yin::tao::form::FormTrait;

/// Whether or not the given archetype belongs in its own submodule.
pub fn in_own_submodule(target: &Archetype) -> bool {
    // todo: filter by type, once Yin has that functionality
    target.force_own_module()
        || target.root_node_logic_activated()
        // todo: this is a hack to check if the children are archetypes or not
        || target.child_archetypes().iter().any(|c| c.internal_name_str().is_some())
}

/// The import path for concepts, starting from Tao and leading to the given archetype.
pub fn ancestor_path(target: &Archetype, separator: &str) -> String {
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
                Some(ancestor_path(&target.parents().first().unwrap(), separator))
            };

            let target_name = target
                .internal_name_str()
                .unwrap()
                .to_snake_case()
                .to_ascii_lowercase();
            if in_own_submodule(target) {
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

fn snake_name(target: &Archetype) -> String {
    target
        .internal_name_str()
        .unwrap()
        .to_snake_case()
        .to_ascii_lowercase()
}

/// Get the output path for a given concept.
pub fn archetype_file_path(target: &Archetype) -> String {
    // append _form to filename to avoid
    // https://rust-lang.github.io/rust-clippy/master/index.html#module_inception
    format!(
        "src/{}/{}_form.rs",
        ancestor_path(target, "/"),
        snake_name(target)
    )
}

/// Get the output path for a given concept.
pub fn module_file_path(target: &Archetype) -> String {
    // module path should always be forced if mod.rs is being generated for it
    assert!(in_own_submodule(target));
    format!("src/{}/mod.rs", ancestor_path(target, "/"))
}

/// Returns the full import path, including the crate itself.
///
/// `yin_override` needed for now because Yin is not yet fully described by its own yin.md.
/// todo: remove once Yin supports that
pub fn import_path(target: &Archetype, yin_override: bool) -> String {
    let build_info = BuildInfo::from(target.id());
    match build_info.import_path() {
        Some(existing_path) => (*existing_path).to_owned(),
        None => {
            let yin_crate = if build_info.crate_name().is_some() {
                (*build_info.crate_name().unwrap()).to_owned()
            } else if yin_override || target.is_newly_defined() {
                "crate".to_owned()
            } else {
                "zamm_yin".to_owned()
            };
            let struct_name = target.internal_name_str().unwrap().to_camel_case();
            format!(
                "{}::{}::{}",
                yin_crate,
                ancestor_path(&target, "::"),
                struct_name
            )
        }
    }
}

/// Turns a concept into a struct to be imported.
pub fn concept_to_struct(target: &Archetype, yin_override: bool) -> StructConfig {
    let build_info = BuildInfo::from(target.id());
    let name = build_info
        .implementation_name()
        .unwrap_or_else(|| Rc::from(target.internal_name_str().unwrap().to_camel_case().as_str()));
    StructConfig {
        name: (*name).to_owned(),
        import: import_path(target, yin_override),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::ArchetypeTrait;
    use zamm_yin::tao::relation::attribute::{Attribute, Owner};
    use zamm_yin::tao::Tao;

    #[test]
    fn own_submodule_tao() {
        initialize_kb();
        assert!(in_own_submodule(&Tao::archetype()));
    }

    #[test]
    fn own_submodule_parent() {
        initialize_kb();
        let mut parent = Tao::archetype().individuate_as_archetype();
        parent.set_internal_name_str("parent");
        let mut child = parent.individuate_as_archetype();
        child.set_internal_name_str("child");
        assert!(in_own_submodule(&parent));
    }

    #[test]
    fn own_submodule_nested() {
        initialize_kb();
        let mut parent = Tao::archetype().individuate_as_archetype();
        parent.set_internal_name_str("parent");
        let mut child = parent.individuate_as_archetype();
        child.set_internal_name_str("child");
        assert!(!in_own_submodule(&child));
    }

    #[test]
    fn own_submodule_forced() {
        initialize_kb();
        let mut parent = Tao::archetype().individuate_as_archetype();
        parent.set_internal_name_str("parent");
        let mut child = parent.individuate_as_archetype();
        child.set_internal_name_str("child");
        // these are individuals, not subtypes, so don't count towards a submodule
        child.individuate_as_form();
        child.individuate_as_form();
        assert!(!in_own_submodule(&child));
    }

    #[test]
    fn folder_path_tao() {
        initialize_kb();
        assert_eq!(
            archetype_file_path(&Tao::archetype()),
            "src/tao/tao_form.rs"
        );
    }

    #[test]
    fn folder_path_attributes() {
        initialize_kb();
        assert_eq!(
            archetype_file_path(&Attribute::archetype().as_archetype()),
            "src/tao/relation/attribute/attribute_form.rs"
        );
    }

    #[test]
    fn folder_path_nested() {
        initialize_kb();
        assert_eq!(
            archetype_file_path(&Owner::archetype().as_archetype()),
            "src/tao/relation/attribute/owner_form.rs"
        );
    }

    #[test]
    fn folder_path_forced_own_module() {
        initialize_kb();
        let mut owner = Owner::archetype();
        owner.mark_own_module();
        assert_eq!(
            archetype_file_path(&owner.as_archetype()),
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
            archetype_file_path(&owner.as_archetype()),
            "src/tao/newfangled/module/attribute/owner/owner_form.rs"
        );
    }

    #[test]
    fn module_path_tao() {
        initialize_kb();
        assert_eq!(module_file_path(&Tao::archetype()), "src/tao/mod.rs");
    }

    #[test]
    fn module_path_attributes() {
        initialize_kb();
        assert_eq!(
            module_file_path(&Attribute::archetype().as_archetype()),
            "src/tao/relation/attribute/mod.rs"
        );
    }

    #[test]
    fn module_path_forced_own_module() {
        initialize_kb();
        let mut owner = Owner::archetype();
        owner.mark_own_module();
        assert_eq!(
            module_file_path(&owner.as_archetype()),
            "src/tao/relation/attribute/owner/mod.rs"
        );
    }

    #[test]
    fn import_path_tao() {
        initialize_kb();
        assert_eq!(import_path(&Tao::archetype(), false), "zamm_yin::tao::Tao");
    }

    #[test]
    fn import_path_attributes() {
        initialize_kb();
        assert_eq!(
            import_path(&Attribute::archetype().as_archetype(), false),
            "zamm_yin::tao::relation::attribute::Attribute"
        );
    }

    #[test]
    fn import_path_nested() {
        initialize_kb();
        assert_eq!(
            import_path(&Owner::archetype().as_archetype(), false),
            "zamm_yin::tao::relation::attribute::Owner"
        );
    }

    #[test]
    fn import_path_forced_own_module() {
        initialize_kb();
        Owner::archetype().mark_own_module();
        assert_eq!(
            import_path(&Owner::archetype().as_archetype(), false),
            "zamm_yin::tao::relation::attribute::owner::Owner"
        );
    }

    #[test]
    fn import_path_newly_defined() {
        initialize_kb();
        let mut owner = Owner::archetype();
        owner.mark_newly_defined();
        assert_eq!(
            import_path(&owner.as_archetype(), false),
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
            import_path(&owner.as_archetype(), false),
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
            import_path(&owner.as_archetype(), false),
            "mycrate::tao::newfangled::module::attribute::owner::Owner"
        );
    }

    #[test]
    fn import_path_multiple_descendants() {
        initialize_kb();
        let mut type1 = Tao::archetype().individuate_as_archetype();
        type1.set_internal_name_str("hello");
        type1.mark_newly_defined();
        let mut type2 = type1.individuate_as_archetype();
        type2.set_internal_name_str("world");
        type2.mark_newly_defined();
        type2.mark_own_module();
        assert_eq!(
            import_path(&type2, false),
            "crate::tao::hello::world::World"
        );
    }

    #[test]
    fn import_path_custom_root() {
        initialize_kb();
        let mut root = Tao::archetype().individuate_as_archetype();
        root.set_internal_name_str("my-root");
        root.mark_newly_defined();
        root.activate_root_node_logic();
        assert_eq!(import_path(&root, false), "crate::my_root::MyRoot");
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
}
