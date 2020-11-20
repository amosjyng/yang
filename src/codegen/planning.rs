use super::CodegenConfig;
use crate::codegen::docstring::into_docstring;
use crate::codegen::template::concept::archetype_module::{
    code_archetype_module, ArchetypeModuleConfig,
};
use crate::codegen::template::concept::attribute::{code_attribute, AttributeFormatConfig};
use crate::codegen::template::concept::auto_init_kb::{code_init, KBInitConfig};
use crate::codegen::template::concept::data::{code_data_concept, DataFormatConfig};
use crate::codegen::template::concept::form::code_form;
use crate::codegen::template::concept::tao::{code_tao, TaoConfig};
use crate::codegen::{output_code, StructConfig};
use crate::tao::archetype::CodegenFlags;
use crate::tao::form::data::DataExtension;
use crate::tao::form::{BuildInfo, BuildInfoExtension};
use crate::tao::{Implement, ImplementExtension};
use heck::KebabCase;
use heck::{CamelCase, SnakeCase};
use itertools::Itertools;
use std::cell::Cell;
use std::convert::TryFrom;
use std::rc::Rc;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{
    Archetype, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype, AttributeArchetypeFormTrait,
};
use zamm_yin::tao::form::data::Data;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::Attribute;

thread_local! {
    static NEXT_IMPLEMENTATION_ID: Cell<usize> = Cell::new(0);
}

/// Grab a new implementation ID, thus incrementing the global ID counter.
fn grab_new_implementation_id(yin: bool) -> usize {
    NEXT_IMPLEMENTATION_ID.with(|id| {
        let mut return_id = id.get();
        id.set(return_id + 1);

        if !yin && return_id == 0 {
            // things that build on top of Yin are effectively one-indexed instead of zero-indexed
            // like Yin is
            return_id += 1;
            id.set(return_id + 1);
        }

        return_id
    })
}

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

fn snake_name(target: &Archetype) -> String {
    target
        .internal_name()
        .unwrap()
        .as_str()
        .to_snake_case()
        .to_ascii_lowercase()
}

/// Get the output path for a given concept.
pub fn archetype_file_path(target: &Archetype) -> String {
    // append _form to filename to avoid
    // https://rust-lang.github.io/rust-clippy/master/index.html#module_inception
    format!(
        "src/{}/{}_form.rs",
        ancestor_path(target, "/", target.force_own_module()),
        snake_name(target)
    )
}

/// Get the output path for a given concept.
pub fn module_file_path(target: &Archetype) -> String {
    // module path should always be forced if mod.rs is being generated for it
    assert!(target.force_own_module() || in_own_submodule(target));
    format!("src/{}/mod.rs", ancestor_path(target, "/", true))
}

/// Returns the import path, not including the crate itself.
///
/// `yin_override` needed for now because Yin is not yet fully described by its own yin.md.
/// todo: remove once Yin supports that
fn import_path(target: &Archetype, force_own_module: bool, yin_override: bool) -> String {
    let build_info = BuildInfo::from(target.id());
    match build_info.import_path() {
        Some(existing_path) => (*existing_path).clone(),
        None => {
            let yin_crate = if build_info.crate_name().is_some() {
                (*build_info.crate_name().unwrap()).clone()
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
    let build_info = BuildInfo::from(target.id());
    let name = build_info
        .implementation_name()
        .unwrap_or_else(|| Rc::new(target.internal_name().unwrap().as_str().to_camel_case()));
    StructConfig {
        name: (*name).clone(),
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

fn activate_archetype(target: &Archetype, parent: &Archetype) -> bool {
    target == &Attribute::archetype().as_archetype()
        || parent.has_ancestor(Attribute::archetype().as_archetype())
        || target.attribute_logic_activated()
}

fn activate_data(target: &Archetype) -> bool {
    target.has_ancestor(Data::archetype().as_archetype()) || target.data_logic_activated()
}

fn generic_config(
    request: &Implement,
    target: &Archetype,
    parent: &Archetype,
    codegen_cfg: &CodegenConfig,
) -> TaoConfig {
    let this = concept_to_struct(&target, codegen_cfg.yin);
    let internal_name = this.name.to_kebab_case();
    let form = if target.root_node_logic_activated() {
        // technically we should allow the user to customize this as well
        concept_to_struct(&Form::archetype(), codegen_cfg.yin)
    } else {
        this.clone()
    };

    let doc = match &request.documentation() {
        Some(d) => format!("\n{}", into_docstring(&d, 0)),
        None => String::new(),
    };

    // allow a default, especially for tests
    let initial_id = request.implementation_id().unwrap_or_else(|| Rc::new(0));
    let id = if codegen_cfg.yin {
        format!("{}", initial_id)
    } else {
        format!("YIN_MAX_ID + {}", initial_id)
    };

    let yin_crate = if codegen_cfg.yin { "crate" } else { "zamm_yin" };

    let imports = if codegen_cfg.yin {
        None
    } else {
        Some("zamm_yin::tao::YIN_MAX_ID".to_owned())
    };

    let parent_struct = concept_to_struct(parent, codegen_cfg.yin);

    let all_attribute_structs: Vec<StructConfig> = target
        .attribute_archetypes()
        .iter()
        .map(|a| concept_to_struct(&a.as_archetype(), codegen_cfg.yin))
        .collect();
    let introduced_attribute_structs: Vec<StructConfig> = target
        .introduced_attribute_archetypes()
        .iter()
        .map(|a| concept_to_struct(&a.as_archetype(), codegen_cfg.yin))
        .collect();
    let all_attributes = format!(
        "vec![{}]",
        all_attribute_structs
            .iter()
            .map(|s| format!("{}::archetype()", s.name))
            .format(", ")
    );
    let all_attribute_imports = all_attribute_structs
        .iter()
        .map(|s| s.import.clone())
        .collect();
    let introduced_attributes = format!(
        "vec![{}]",
        introduced_attribute_structs
            .iter()
            .map(|s| format!("{}::archetype()", s.name))
            .format(", ")
    );
    let introduced_attribute_imports = introduced_attribute_structs
        .iter()
        .map(|s| s.import.clone())
        .collect();

    let archetype_name = if activate_archetype(target, parent) {
        "AttributeArchetype".to_owned()
    } else {
        "Archetype".to_owned()
    };

    TaoConfig {
        yin_crate: yin_crate.to_owned(),
        imports,
        this,
        internal_name,
        form,
        parent_name: parent_struct.name,
        parent_import: parent_struct.import,
        all_attributes,
        all_attribute_imports,
        introduced_attributes,
        introduced_attribute_imports,
        archetype_name,
        doc,
        id,
    }
}

fn attribute_config(
    base_cfg: TaoConfig,
    target: &Archetype,
    codegen_cfg: &CodegenConfig,
) -> AttributeFormatConfig {
    let target_attr = AttributeArchetype::from(target.id());
    let owner_type_concept = target_attr.owner_archetype();
    let value_type_concept = target_attr.value_archetype();

    let owner_type = concept_to_struct(&owner_type_concept, codegen_cfg.yin);
    let value_type = concept_to_struct(&value_type_concept, codegen_cfg.yin);

    let owner_form = concept_to_struct(&or_form_default(owner_type_concept), codegen_cfg.yin);
    let value_form = concept_to_struct(&or_form_default(value_type_concept), codegen_cfg.yin);

    AttributeFormatConfig {
        tao_cfg: base_cfg,
        owner_type,
        owner_form,
        value_type,
        value_form,
    }
}

fn data_config(base_cfg: TaoConfig, target: &Archetype) -> DataFormatConfig {
    DataFormatConfig {
        tao_cfg: base_cfg,
        rust_primitive_name: target.rust_primitive().unwrap(),
        default_value: target.default_value().unwrap(),
    }
}

fn primary_parent(target: &Archetype) -> Archetype {
    *target.parents().first().unwrap()
}

/// Generate code for a given concept. Post-processing still needed.
pub fn code_archetype(request: Implement, codegen_cfg: &CodegenConfig) -> String {
    let target = Archetype::from(request.target().unwrap().id());
    let parent = primary_parent(&target);

    let base_cfg = generic_config(&request, &target, &parent, codegen_cfg);

    if target.root_node_logic_activated() {
        code_tao(&base_cfg)
    } else if activate_archetype(&target, &parent) {
        code_attribute(&attribute_config(base_cfg, &target, codegen_cfg))
    } else if activate_data(&target) {
        code_data_concept(&data_config(base_cfg, &target))
    } else {
        code_form(&base_cfg)
    }
}

/// Generate code for a given module. Post-processing still needed.
pub fn code_module(parent: Archetype) -> String {
    let mut archetype_names = vec![parent.internal_name().unwrap()];
    for child in parent.child_archetypes() {
        archetype_names.push(child.internal_name().unwrap());
    }

    code_archetype_module(&ArchetypeModuleConfig {
        archetype_names,
        ..ArchetypeModuleConfig::default()
    })
}

/// Create initialization file for newly defined concepts.
pub fn handle_init(archetype_requests: &[Implement], codegen_cfg: &CodegenConfig) {
    let yin_crate = if codegen_cfg.yin { "crate" } else { "zamm_yin" };
    let mut concepts_to_initialize = Vec::<StructConfig>::new();
    for implement_command in archetype_requests {
        let mut implement = Implement::from(implement_command.id());
        let target_type = Archetype::from(implement.target().unwrap().id());
        concepts_to_initialize.push(concept_to_struct(&target_type, codegen_cfg.yin));

        // only set ID for user if user hasn't already set it
        if implement.implementation_id().is_none() {
            implement.set_implementation_id(grab_new_implementation_id(codegen_cfg.yin));
        }
    }

    let code = code_init(&KBInitConfig {
        yin_crate: yin_crate.to_owned(),
        concepts_to_initialize,
    });
    output_code(&code, "src/tao/auto_init.rs", codegen_cfg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use indoc::indoc;
    use zamm_yin::tao::relation::attribute::{Attribute, Owner};
    use zamm_yin::tao::Tao;

    #[test]
    fn yin_id_zero_indexed() {
        assert_eq!(grab_new_implementation_id(true), 0);
        assert_eq!(grab_new_implementation_id(true), 1);
    }

    #[test]
    fn yang_yin_id_one_indexed() {
        assert_eq!(grab_new_implementation_id(false), 1);
        assert_eq!(grab_new_implementation_id(false), 2);
    }

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
    fn test_default_no_activations() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyAttrType".to_owned());
        target.mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(target.as_form());
        let cfg = generic_config(
            &implement,
            &target.as_archetype(),
            &primary_parent(&target),
            &CodegenConfig::default(),
        );

        assert!(!target.root_node_logic_activated());
        assert!(!activate_archetype(&target, &primary_parent(&target)));
        assert!(!activate_data(&target));

        assert!(cfg.id.contains("YIN_MAX_ID"));
    }

    #[test]
    fn code_cfg_for_yin() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyDataType".to_owned());
        target.mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(target.as_form());
        let cfg = generic_config(
            &implement,
            &target.as_archetype(),
            &primary_parent(&target),
            &CodegenConfig {
                yin: true,
                ..CodegenConfig::default()
            },
        );

        assert!(!cfg.id.contains("YIN_MAX_ID"));
    }

    #[test]
    fn test_default_doc_newline() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyAttrType".to_owned());
        target.mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(target.as_form());
        implement.document("One.\n\nTwo.");
        let cfg = generic_config(
            &implement,
            &target.as_archetype(),
            &primary_parent(&target),
            &CodegenConfig::default(),
        );

        assert_eq!(
            cfg.doc.trim(),
            indoc! {r"
            /// One.
            ///
            /// Two.
        "}
            .trim()
        );
    }

    #[test]
    fn code_cfg_for_root_node_activated() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyRoot".to_owned());
        target.mark_newly_defined();
        target.activate_root_node_logic();

        assert!(target.root_node_logic_activated());
        assert!(!activate_archetype(&target, &primary_parent(&target)));
        assert!(!activate_data(&target));
    }

    #[test]
    fn code_cfg_for_attribute_activated() {
        initialize_kb();
        let mut target = AttributeArchetype::from(Tao::archetype().individuate_as_archetype().id());
        target.set_internal_name("MyAttrType".to_owned());
        target.mark_newly_defined();
        target.activate_attribute_logic();
        // todo: reset after set_owner_archetype and set_value_archetype moved to
        // BackwardsCompatibility
        AttributeArchetypeFormTrait::set_owner_archetype(&mut target, Tao::archetype());
        AttributeArchetypeFormTrait::set_value_archetype(&mut target, Form::archetype());
        let mut implement = Implement::new();
        implement.set_target(target.as_form());
        let parent = primary_parent(&target.as_archetype());
        let codegen_cfg = CodegenConfig::default();

        let attr_cfg = attribute_config(
            generic_config(&implement, &target.as_archetype(), &parent, &codegen_cfg),
            &target.as_archetype(),
            &codegen_cfg,
        );

        assert!(!target.root_node_logic_activated());
        assert!(activate_archetype(&target.as_archetype(), &parent));
        assert!(!activate_data(&target.as_archetype()));

        assert_eq!(attr_cfg.owner_type.name, "Tao".to_owned());
        assert_eq!(attr_cfg.value_type.name, "Form".to_owned());
        assert_eq!(
            attr_cfg.tao_cfg.archetype_name,
            "AttributeArchetype".to_owned()
        );
    }

    #[test]
    fn code_cfg_for_data_activated() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyDataType".to_owned());
        target.mark_newly_defined();
        target.activate_data_logic();

        assert!(!target.root_node_logic_activated());
        assert!(!activate_archetype(&target, &primary_parent(&target)));
        assert!(activate_data(&target));
    }
}
