use super::concept_to_struct;
use super::imports::{in_own_submodule, root_node_or_equivalent};
use crate::codegen::docstring::into_docstring;
use crate::codegen::template::basic::{FileFragment, ImplementationFragment};
use crate::codegen::template::concept::archetype::{add_archetype_fragment, ArchetypeFormatConfig};
use crate::codegen::template::concept::attribute::{add_attr_fragments, AttributeFormatConfig};
use crate::codegen::template::concept::attribute_property::{
    add_attr_to_impl, AttributePropertyConfig,
};
use crate::codegen::template::concept::data::{add_data_fragments, DataFormatConfig};
use crate::codegen::template::concept::flag::{add_flag_to_impl, FlagConfig};
use crate::codegen::template::concept::form::{add_form_fragment, FormFormatConfig};
use crate::codegen::template::concept::tao::{tao_file_fragment, TaoConfig};
use crate::codegen::CODE_WIDTH;
use crate::codegen::{CodegenConfig, StructConfig};
use crate::tao::archetype::rust_item_archetype::DataArchetype;
use crate::tao::form::rust_item::data::Data;
use crate::tao::form::rust_item::{Crate, CrateExtension};
use crate::tao::perspective::{BuildInfo, KnowledgeGraphNode};
use crate::tao::Implement;
use heck::{KebabCase, SnakeCase};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::rc::Rc;
use zamm_yin::node_wrappers::{BaseNodeTrait, CommonNodeTrait};
use zamm_yin::tao::archetype::{
    Archetype, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype, AttributeArchetypeFormTrait,
};
use zamm_yin::tao::form::{Form, FormTrait};
use zamm_yin::tao::relation::attribute::{Attribute, MetaForm};

fn or_form_default(archetype: Archetype) -> Archetype {
    if root_node_or_equivalent(&archetype) {
        Archetype::try_from(Form::TYPE_NAME).unwrap() // allow user to override Form
    } else {
        archetype
    }
}

fn activate_archetype(target: &Archetype) -> bool {
    target.has_ancestor(Archetype::archetype())
        || KnowledgeGraphNode::from(target.id()).is_archetype_analogue()
}

fn activate_attribute(target: &Archetype) -> bool {
    target == &Attribute::archetype().into()
        || target.has_ancestor(Attribute::archetype().into())
        || KnowledgeGraphNode::from(target.id()).is_attribute_analogue()
}

fn activate_data(target: &Archetype) -> bool {
    target.has_ancestor(Data::archetype().into())
        || KnowledgeGraphNode::from(target.id()).is_data_analogue()
}

fn form_for(target: &Archetype, codegen_cfg: &CodegenConfig) -> StructConfig {
    if KnowledgeGraphNode::from(target.id()).is_root_analogue() {
        // technically we should allow the user to customize this as well
        concept_to_struct(&Form::archetype(), codegen_cfg.yin)
    } else {
        concept_to_struct(target, codegen_cfg.yin)
    }
}

fn generic_config(
    request: &Implement,
    target: &Archetype,
    parent: &Archetype,
    codegen_cfg: &CodegenConfig,
) -> TaoConfig {
    let this = concept_to_struct(&target, codegen_cfg.yin);
    let internal_name = this.name.to_kebab_case();
    let form = form_for(target, codegen_cfg);

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
    let all_attributes = all_attribute_structs
        .iter()
        .map(|s| format!("{}::archetype()", s.name))
        .collect();
    let all_attribute_imports = all_attribute_structs
        .iter()
        .map(|s| s.import.clone())
        .collect();
    let introduced_attributes = introduced_attribute_structs
        .iter()
        .map(|s| format!("{}::archetype()", s.name))
        .collect();
    let introduced_attribute_imports = introduced_attribute_structs
        .iter()
        .map(|s| s.import.clone())
        .collect();

    let archetype = concept_to_struct(&target.meta_archetype(), codegen_cfg.yin);

    TaoConfig {
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
        archetype,
        doc,
        id,
    }
}

fn into_archetype_fn(archetype: &Archetype) -> String {
    if !KnowledgeGraphNode::from(archetype.id()).is_root_archetype_analogue() {
        ".into()".to_owned()
    } else {
        String::new()
    }
}

fn form_config(
    base_cfg: &TaoConfig,
    target: &mut Archetype,
    codegen_cfg: &CodegenConfig,
) -> FormFormatConfig {
    let meta_archetype = if target.has_specific_meta() {
        Some(concept_to_struct(&target.specific_meta(), codegen_cfg.yin))
    } else {
        None
    };

    let mut initial_ancestors = target.ancestry();
    let root_node_override = initial_ancestors
        .iter()
        .map(|a| KnowledgeGraphNode::from(a.id()))
        .any(|k| k.is_root_analogue());
    if root_node_override {
        // then we pretend that our current runtime Tao does not exist, and the root node analogue
        // is the real Tao
        initial_ancestors.remove(0);
    }
    let ancestors = initial_ancestors
        .into_iter()
        .map(|a| concept_to_struct(&a, codegen_cfg.yin))
        .collect();

    FormFormatConfig {
        tao_cfg: base_cfg.clone(),
        meta_archetype,
        ancestors,
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

    let owner_into_archetype = into_archetype_fn(&owner_type_concept);
    let value_into_archetype = into_archetype_fn(&value_type_concept);

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

fn archetype_config(
    base_cfg: &TaoConfig,
    target: &Archetype,
    codegen_cfg: &CodegenConfig,
) -> ArchetypeFormatConfig {
    // todo: use Yin's ArchetypeFromTrait::infra_archetype function once that's available
    let infra = Archetype::from(
        target
            .incoming_nodes(MetaForm::TYPE_ID)
            .last()
            .unwrap()
            .id(),
    );
    ArchetypeFormatConfig {
        tao_cfg: base_cfg.clone(),
        infra_archetype: form_for(&infra, codegen_cfg),
    }
}

fn data_config(base_cfg: &TaoConfig, target: &DataArchetype) -> DataFormatConfig {
    let rust_primitive_boxed_name = target.rust_primitive().unwrap();
    let rust_primitive_unboxed_name = match target.unboxed_representation() {
        Some(custom_name) => custom_name,
        None => rust_primitive_boxed_name.clone(),
    };
    DataFormatConfig {
        tao_cfg: base_cfg.clone(),
        rust_primitive_unboxed_name,
        rust_primitive_boxed_name,
        default_value: target.default_value().unwrap(),
    }
}

fn flag_config(codegen_cfg: &CodegenConfig, target: &Archetype, flag: &Archetype) -> FlagConfig {
    let doc = BuildInfo::from(flag.id())
        .dual_purpose_documentation()
        .unwrap();
    FlagConfig {
        public: true,
        property_name: Rc::from(flag.internal_name().unwrap().to_snake_case()),
        doc,
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
    let value_as_data = DataArchetype::from(value_type.id());
    let rust_primitive = value_as_data.rust_primitive();
    if activate_data(&value_type) {
        assert!(
            rust_primitive.is_some(),
            "Data type {:?} has no defined Rust primitive.",
            value_type
        );
    }
    let rust_primitive_unboxed = match value_as_data.unboxed_representation() {
        Some(unboxed) => Some(unboxed),
        None => rust_primitive.clone(),
    };
    let doc = BuildInfo::from(attr.id())
        .dual_purpose_documentation()
        .unwrap();

    AttributePropertyConfig {
        public: true,
        property_name: Rc::from(attr.internal_name().unwrap().to_snake_case()),
        doc,
        attr: concept_to_struct(&(*attr).into(), codegen_cfg.yin),
        owner_type: concept_to_struct(target, codegen_cfg.yin),
        value_type: concept_to_struct(&value_type, codegen_cfg.yin),
        rust_primitive,
        rust_primitive_unboxed,
        primitive_test_value: value_as_data.default_value(),
        dummy_test_value: value_as_data.dummy_value(),
        hereditary: !attr.is_nonhereditary_attr(),
        multi_valued: attr.is_multi_valued_attr(),
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
    let mut target = Archetype::from(request.target().unwrap().id());
    let parent = primary_parent(&target);

    let base_cfg = generic_config(&request, &target, &parent, codegen_cfg);

    let mut file = tao_file_fragment(&base_cfg);

    let kgn = KnowledgeGraphNode::from(target.id());
    assert!(!kgn.is_imported(), "Coding an imported archetype {:?}", kgn);
    if !kgn.is_root_analogue() {
        add_form_fragment(
            &form_config(&base_cfg, &mut target, &codegen_cfg),
            &mut file,
        );
    }

    if activate_archetype(&target) {
        add_archetype_fragment(
            &archetype_config(&base_cfg, &target, &codegen_cfg),
            &mut file,
        );
    } else if activate_attribute(&target) {
        add_attr_fragments(
            &attribute_config(&base_cfg, &target, codegen_cfg),
            &mut file,
        );
    } else if activate_data(&target) {
        add_data_fragments(
            &data_config(&base_cfg, &DataArchetype::from(target.id())),
            &mut file,
        );
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
        target.set_internal_name("MyAttrType");
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
        assert!(!activate_attribute(&target));
        assert!(!activate_data(&target));

        assert!(cfg.id.contains("YIN_MAX_ID"));
    }

    #[test]
    fn code_cfg_for_yin() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyDataType");
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
        target.set_internal_name("MyAttrType");
        KnowledgeGraphNode::from(target.id()).mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(&target.as_form());
        implement.set_documentation("One.\n\nTwo.");
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
        target.set_internal_name("MyRoot");
        let mut target_kgn = KnowledgeGraphNode::from(target.id());
        target_kgn.mark_newly_defined();
        target_kgn.mark_root_analogue();

        assert!(target_kgn.is_root_analogue());
        assert!(!activate_attribute(&target));
        assert!(!activate_data(&target));
    }

    #[test]
    fn code_cfg_for_attribute_activated() {
        initialize_kb();
        let mut target = AttributeArchetype::from(Tao::archetype().individuate_as_archetype().id());
        target.set_internal_name("MyAttrType");
        let mut kgn = KnowledgeGraphNode::from(target.id());
        kgn.mark_newly_defined();
        kgn.mark_attribute_analogue();
        // todo: reset after set_owner_archetype and set_value_archetype moved to
        // BackwardsCompatibility
        AttributeArchetypeFormTrait::set_owner_archetype(&mut target, &Tao::archetype());
        AttributeArchetypeFormTrait::set_value_archetype(&mut target, &Form::archetype());
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
        assert!(activate_attribute(&target.into()));
        assert!(!activate_data(&target.into()));

        assert_eq!(attr_cfg.owner_type.name, "Tao".to_owned());
        assert_eq!(attr_cfg.value_type.name, "Form".to_owned());
        // Caller is responsible for designating a custom archetype, if so desired. Simply marking
        // a node as an attribute analogue does not make its archetype an attribute analogue too.
        assert_eq!(attr_cfg.tao_cfg.archetype.name, "Archetype".to_owned());
    }

    #[test]
    fn default_meta() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyDataType");
        let mut kgn = KnowledgeGraphNode::from(target.id());
        kgn.mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(&target.as_form());
        let parent = primary_parent(&target);
        let codegen_cfg = CodegenConfig::default();
        // todo: this line is only needed because Yin doesn't set a default meta archetype
        Tao::archetype().set_meta_archetype(&Archetype::archetype());
        Crate::yang().set_version("0.1.8");

        let cfg = generic_config(&implement, &target, &parent, &codegen_cfg);
        assert_eq!(cfg.archetype.name, "Archetype");
    }

    #[test]
    fn specific_meta() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyDataType");
        let mut kgn = KnowledgeGraphNode::from(target.id());
        kgn.mark_newly_defined();
        let mut implement = Implement::new();
        implement.set_target(&target.as_form());
        let parent = primary_parent(&target);
        let codegen_cfg = CodegenConfig::default();
        // todo: this line is only needed because Yin doesn't set a default meta archetype
        Tao::archetype().set_meta_archetype(&Archetype::archetype());
        Crate::yang().set_version("0.1.8");

        // the only difference from the above test
        target
            .specific_meta()
            .set_internal_name("my-data-type-archetype");

        let cfg = generic_config(&implement, &target, &parent, &codegen_cfg);
        assert_eq!(cfg.archetype.name, "MyDataTypeArchetype");
    }

    #[test]
    fn code_cfg_for_data_activated() {
        initialize_kb();
        let mut target = Tao::archetype().individuate_as_archetype();
        target.set_internal_name("MyDataType");
        let mut kgn = KnowledgeGraphNode::from(target.id());
        kgn.mark_newly_defined();
        kgn.mark_data_analogue();

        assert!(!kgn.is_root_analogue());
        assert!(!activate_attribute(&target));
        assert!(activate_data(&target));
    }

    #[test]
    fn integration_test_root_node_generation() {
        initialize_kb();
        Crate::current().set_implementation_name("moo");
        let mut my_root = Tao::archetype().individuate_as_archetype();
        KnowledgeGraphNode::from(my_root.id()).mark_root_analogue();
        my_root.set_internal_name("my-root");
        let mut i = Implement::new();
        i.set_target(&my_root.as_form());
        let code = code_archetype(i, &CodegenConfig::default());
        assert!(!code.contains("impl FormTrait"));
    }
}
