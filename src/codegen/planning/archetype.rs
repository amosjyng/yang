use super::concept_to_struct;
use super::imports::{in_own_submodule, root_node_or_equivalent};
use crate::codegen::docstring::into_docstring;
use crate::codegen::template::basic::{FileFragment, ImplementationFragment};
use crate::codegen::template::concept::attribute::{add_attr_fragments, AttributeFormatConfig};
use crate::codegen::template::concept::attribute_property::{
    add_attr_to_impl, AttributePropertyConfig,
};
use crate::codegen::template::concept::data::{add_data_fragments, DataFormatConfig};
use crate::codegen::template::concept::flag::{add_flag_to_impl, FlagConfig};
use crate::codegen::template::concept::form::add_form_fragment;
use crate::codegen::template::concept::tao::{tao_file_fragment, InternalNameConfig, TaoConfig};
use crate::codegen::CODE_WIDTH;
use crate::codegen::{CodegenConfig, StructConfig};
use crate::tao::form::data::DataExtension;
use crate::tao::form::{Crate, CrateExtension};
use crate::tao::perspective::{BuildInfo, BuildInfoExtension, KnowledgeGraphNode};
use crate::tao::Implement;
use heck::{KebabCase, SnakeCase};
use itertools::Itertools;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{
    Archetype, ArchetypeFormExtensionTrait, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype,
    AttributeArchetypeFormTrait,
};
use zamm_yin::tao::form::data::Data;
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::Attribute;

fn or_form_default(archetype: Archetype) -> Archetype {
    if root_node_or_equivalent(&archetype) {
        Archetype::try_from(Form::TYPE_NAME).unwrap() // allow user to override Form
    } else {
        archetype
    }
}

fn activate_archetype(target: &Archetype) -> bool {
    target == &Attribute::archetype().into()
        || target.has_ancestor(Attribute::archetype().into())
        || KnowledgeGraphNode::from(target.id()).is_attribute_analogue()
}

fn activate_data(target: &Archetype) -> bool {
    target.has_ancestor(Data::archetype())
        || KnowledgeGraphNode::from(target.id()).is_data_analogue()
}

fn generic_config(
    request: &Implement,
    target: &Archetype,
    parent: &Archetype,
    codegen_cfg: &CodegenConfig,
) -> TaoConfig {
    let this = concept_to_struct(&target, codegen_cfg.yin);
    let internal_name = this.name.to_kebab_case();
    let form = if KnowledgeGraphNode::from(target.id()).is_root_analogue() {
        // technically we should allow the user to customize this as well
        concept_to_struct(&Form::archetype(), codegen_cfg.yin)
    } else {
        this.clone()
    };

    let internal_name_cfg = if Crate::yin().version_at_least(0, 1, 4) {
        InternalNameConfig::YIN_AT_LEAST_0_1_4
    } else if Crate::yin().version_at_least(0, 1, 1) {
        InternalNameConfig::YIN_AT_LEAST_0_1_1
    } else {
        InternalNameConfig::DEFAULT
    };

    let doc = match &request.documentation() {
        Some(d) => format!("\n{}", into_docstring(&d, CODE_WIDTH)),
        None => String::new(),
    };

    // allow a default, especially for tests
    let initial_id = request.concept_id().unwrap_or_else(|| Rc::new(0));
    let id = if codegen_cfg.yin {
        format!("{}", initial_id)
    } else {
        format!("YIN_MAX_ID + {}", initial_id)
    };

    let imports = if codegen_cfg.yin {
        None
    } else {
        Some("zamm_yin::tao::YIN_MAX_ID".to_owned())
    };

    let parent_struct = concept_to_struct(parent, codegen_cfg.yin);

    let all_attribute_structs: Vec<StructConfig> = target
        .attributes()
        .into_iter()
        .map(|a| concept_to_struct(&a.into(), codegen_cfg.yin))
        .collect();
    let introduced_attribute_structs: Vec<StructConfig> = target
        .added_attributes()
        .into_iter()
        .map(|a| concept_to_struct(&a.into(), codegen_cfg.yin))
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

    let archetype_name = if activate_archetype(target) {
        "AttributeArchetype".to_owned()
    } else {
        "Archetype".to_owned()
    };

    TaoConfig {
        imports,
        this,
        internal_name,
        internal_name_cfg,
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

fn into_archetype_fn(yin: &Crate, archetype: &Archetype) -> String {
    if yin.version_at_least(0, 1, 4) {
        if activate_archetype(archetype) {
            ".into()".to_owned()
        } else {
            String::new()
        }
    } else {
        ".as_archetype()".to_owned()
    }
}

fn attribute_config(
    base_cfg: &TaoConfig,
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

    let yin = Crate::yin();
    let owner_into_archetype = into_archetype_fn(&yin, &owner_type_concept);
    let value_into_archetype = into_archetype_fn(&yin, &value_type_concept);

    AttributeFormatConfig {
        tao_cfg: base_cfg.clone(),
        owner_type,
        owner_form,
        value_type,
        value_form,
        owner_into_archetype,
        value_into_archetype,
    }
}

fn data_config(base_cfg: &TaoConfig, target: &Archetype) -> DataFormatConfig {
    DataFormatConfig {
        tao_cfg: base_cfg.clone(),
        rust_primitive_name: target.rust_primitive().unwrap(),
        default_value: target.default_value().unwrap(),
    }
}

fn flag_config(codegen_cfg: &CodegenConfig, target: &Archetype, flag: &Archetype) -> FlagConfig {
    FlagConfig {
        public: true,
        property_name: Rc::from(flag.internal_name_str().unwrap().to_snake_case()),
        doc: BuildInfo::from(flag.id()).dual_documentation().unwrap(),
        flag: concept_to_struct(flag, codegen_cfg.yin),
        owner_type: concept_to_struct(target, codegen_cfg.yin),
        hereditary: !AttributeArchetype::from(flag.id()).is_nonhereditary_attr(),
    }
}

fn attr_config(
    codegen_cfg: &CodegenConfig,
    target: &Archetype,
    attr: &AttributeArchetype,
) -> AttributePropertyConfig {
    let value_type = or_form_default(attr.value_archetype());
    if activate_data(&value_type) {
        assert!(
            value_type.rust_primitive().is_some(),
            "Data type {:?} has no defined Rust primitive.",
            value_type
        );
    }
    AttributePropertyConfig {
        public: true,
        property_name: Rc::from(attr.internal_name_str().unwrap().to_snake_case()),
        doc: BuildInfo::from(attr.id()).dual_documentation().unwrap(),
        attr: concept_to_struct(&(*attr).into(), codegen_cfg.yin),
        owner_type: concept_to_struct(target, codegen_cfg.yin),
        value_type: concept_to_struct(&value_type, codegen_cfg.yin),
        rust_primitive: value_type.rust_primitive(),
        primitive_test_value: value_type.default_value(),
        hereditary: !attr.is_nonhereditary_attr(),
    }
}

fn primary_parent(target: &Archetype) -> Archetype {
    *target.parents().first().unwrap()
}

fn add_struct_flag_fragments(
    target: &Archetype,
    cfg: &CodegenConfig,
    implementation: &mut ImplementationFragment,
    file: &mut FileFragment,
) {
    for flag in target.added_flags() {
        add_flag_to_impl(&flag_config(cfg, &target, &flag), implementation, file);
    }
}

fn add_struct_attr_fragments(
    target: &Archetype,
    cfg: &CodegenConfig,
    implementation: &mut ImplementationFragment,
    file: &mut FileFragment,
) {
    for attr in target.added_attributes() {
        add_attr_to_impl(&attr_config(cfg, &target, &attr), implementation, file);
    }
}

/// Generate code for a given concept. Post-processing still needed.
pub fn code_archetype(request: Implement, codegen_cfg: &CodegenConfig) -> String {
    let target = Archetype::from(request.target().unwrap().id());
    let parent = primary_parent(&target);

    let base_cfg = generic_config(&request, &target, &parent, codegen_cfg);

    let mut file = tao_file_fragment(&base_cfg);

    if !KnowledgeGraphNode::from(target.id()).is_root_analogue() {
        add_form_fragment(&base_cfg, &mut file);
    }

    if activate_archetype(&target) {
        add_attr_fragments(
            &attribute_config(&base_cfg, &target, codegen_cfg),
            &mut file,
        );
    } else if activate_data(&target) {
        add_data_fragments(&data_config(&base_cfg, &target), &mut file);
    }

    if !in_own_submodule(&target) {
        let mut implementation =
            ImplementationFragment::new_struct_impl(concept_to_struct(&target, codegen_cfg.yin));
        if !target.added_flags().is_empty() {
            add_struct_flag_fragments(&target, codegen_cfg, &mut implementation, &mut file);
        }
        if !target.added_attributes().is_empty() {
            add_struct_attr_fragments(&target, codegen_cfg, &mut implementation, &mut file);
        }
        if !implementation.content.borrow().appendages.is_empty() {
            file.append(Rc::new(RefCell::new(implementation)));
        }
    }

    file.set_current_crate(Crate::current().implementation_name().unwrap());
    file.generate_code()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use crate::tao::perspective::KnowledgeGraphNode;
    use indoc::indoc;
    use zamm_yin::tao::Tao;

    #[test]
    fn test_default_no_activations() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name_str("MyAttrType");
        let mut target_kgn = KnowledgeGraphNode::from(target.id());
        target_kgn.mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(&target.as_form());
        let cfg = generic_config(
            &implement,
            &target,
            &primary_parent(&target),
            &CodegenConfig::default(),
        );

        assert!(!target_kgn.is_root_analogue());
        assert!(!activate_archetype(&target));
        assert!(!activate_data(&target));

        assert!(cfg.id.contains("YIN_MAX_ID"));
    }

    #[test]
    fn code_cfg_for_yin() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name_str("MyDataType");
        KnowledgeGraphNode::from(target.id()).mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(&target.as_form());
        let cfg = generic_config(
            &implement,
            &target,
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
        target.set_internal_name_str("MyAttrType");
        KnowledgeGraphNode::from(target.id()).mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(&target.as_form());
        implement.set_documentation("One.\n\nTwo.".to_owned());
        let cfg = generic_config(
            &implement,
            &target,
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
        target.set_internal_name_str("MyRoot");
        let mut target_kgn = KnowledgeGraphNode::from(target.id());
        target_kgn.mark_newly_defined();
        target_kgn.mark_root_analogue();

        assert!(target_kgn.is_root_analogue());
        assert!(!activate_archetype(&target));
        assert!(!activate_data(&target));
    }

    #[test]
    fn code_cfg_for_attribute_activated() {
        initialize_kb();
        let mut target = AttributeArchetype::from(Tao::archetype().individuate_as_archetype().id());
        target.set_internal_name_str("MyAttrType");
        let mut kgn = KnowledgeGraphNode::from(target.id());
        kgn.mark_newly_defined();
        kgn.mark_attribute_analogue();
        // todo: reset after set_owner_archetype and set_value_archetype moved to
        // BackwardsCompatibility
        AttributeArchetypeFormTrait::set_owner_archetype(&mut target, Tao::archetype());
        AttributeArchetypeFormTrait::set_value_archetype(&mut target, Form::archetype());
        let mut implement = Implement::new();
        implement.set_target(&target.as_form());
        let parent = primary_parent(&target.into());
        let codegen_cfg = CodegenConfig::default();

        let attr_cfg = attribute_config(
            &generic_config(&implement, &target.into(), &parent, &codegen_cfg),
            &target.into(),
            &codegen_cfg,
        );

        assert!(!kgn.is_root_analogue());
        assert!(activate_archetype(&target.into()));
        assert!(!activate_data(&target.into()));

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
        target.set_internal_name_str("MyDataType");
        let mut kgn = KnowledgeGraphNode::from(target.id());
        kgn.mark_newly_defined();
        kgn.mark_data_analogue();

        assert!(!kgn.is_root_analogue());
        assert!(!activate_archetype(&target));
        assert!(activate_data(&target));
    }

    #[test]
    fn integration_test_root_node_generation() {
        initialize_kb();
        Crate::current().set_implementation_name("moo");
        let mut my_root = Tao::archetype().individuate_as_archetype();
        KnowledgeGraphNode::from(my_root.id()).mark_root_analogue();
        my_root.set_internal_name_str("my-root");
        let mut i = Implement::new();
        i.set_target(&my_root.as_form());
        let code = code_archetype(i, &CodegenConfig::default());
        assert!(!code.contains("impl FormTrait"));
    }
}
