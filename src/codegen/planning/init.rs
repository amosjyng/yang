use super::{concept_to_struct, grab_new_implementation_id};
use crate::codegen::template::concept::auto_init_kb::{code_init, KBInitConfig, Link};
use crate::codegen::{output_code, CodegenConfig, StructConfig};
use crate::tao::archetype::CreateImplementation;
use crate::tao::form::{Crate, CrateExtension};
use crate::tao::perspective::KnowledgeGraphNode;
use crate::tao::Implement;
use heck::CamelCase;
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{
    Archetype, ArchetypeFormExtensionTrait, ArchetypeFormTrait, ArchetypeTrait,
};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::has_property::{HasAttribute, HasFlag};
use zamm_yin::tao::relation::attribute::{OwnerArchetype, ValueArchetype};
use zamm_yin::Wrapper;

fn setup_archetype_init(
    implement: &mut Implement,
    target_type: &Archetype,
    concepts_to_initialize: &mut Vec<StructConfig>,
    attributes: &mut Vec<Link>,
    add_attributes: bool,
    codegen_cfg: &CodegenConfig,
) {
    let has_attr = concept_to_struct(&HasAttribute::archetype().into(), codegen_cfg.yin);
    let has_flag = concept_to_struct(&HasFlag::archetype().into(), codegen_cfg.yin);
    let target_struct = concept_to_struct(&target_type, codegen_cfg.yin);
    concepts_to_initialize.push(target_struct);

    // only set ID for user if user hasn't already set it
    if implement.concept_id().is_none() {
        implement.set_concept_id(grab_new_implementation_id(codegen_cfg.yin));
    }

    // only initialize concept attributes if user hasn't already done it because they were
    // using an older version of Yang
    if add_attributes {
        let target_struct = concept_to_struct(&target_type, codegen_cfg.yin);

        for flag in target_type.added_flags() {
            attributes.push(Link {
                from: target_struct.clone(),
                link_type: has_flag.clone(),
                to: concept_to_struct(&flag, codegen_cfg.yin),
            });
        } // todo: set flags like we set attributes down below

        for attr in target_type.added_attributes() {
            attributes.push(Link {
                from: target_struct.clone(),
                link_type: has_attr.clone(),
                to: concept_to_struct(&attr.into(), codegen_cfg.yin),
            });
        }
        let mut attributes_to_check = target_type.attributes();
        attributes_to_check.push(OwnerArchetype::archetype());
        attributes_to_check.push(ValueArchetype::archetype());
        for attr in attributes_to_check {
            let attr_struct = concept_to_struct(&Archetype::from(attr.id()), codegen_cfg.yin);
            // use base wrapper because we want to see if the flag is set with this node, not
            // with an ancestor
            for outgoing in target_type
                .essence()
                .base_wrapper()
                .outgoing_nodes(attr.id())
            {
                attributes.push(Link {
                    from: target_struct.clone(),
                    link_type: attr_struct.clone(),
                    to: concept_to_struct(&Archetype::from(outgoing.id()), codegen_cfg.yin),
                });
            }
        }
    }
}

fn init_config(archetype_requests: &mut [Implement], codegen_cfg: &CodegenConfig) -> KBInitConfig {
    let mut concepts_to_initialize = Vec::<StructConfig>::new();
    let mut attributes = Vec::<Link>::new();
    let add_attributes = Crate::yang().version_at_least(0, 1, 7);

    for implement in archetype_requests {
        let mut target_type = Archetype::from(implement.target().unwrap().id());
        setup_archetype_init(
            implement,
            &target_type,
            &mut concepts_to_initialize,
            &mut attributes,
            add_attributes,
            codegen_cfg,
        );

        // no need to gate this behind a version since specific metas should only be defined in
        // more recent versions of Yang
        if target_type.has_specific_meta() {
            let mut target_meta = target_type.specific_meta();
            if target_meta.implementations().is_empty() {
                // if the meta is already marked for implementation because it was defined separately,
                // then don't bother creating it a second time
                let mut meta_impl = target_meta.implement();
                KnowledgeGraphNode::from(target_meta.id()).mark_newly_defined();
                let target_name = target_type.internal_name_str().unwrap();
                let meta_name = format!("{}-archetype", target_name);
                target_meta.set_internal_name_str(&meta_name);
                meta_impl.set_documentation(&format!(
                    "Meta-object for {} meta-attributes.",
                    target_name.to_camel_case()
                ));
                setup_archetype_init(
                    &mut meta_impl,
                    &target_meta,
                    &mut concepts_to_initialize,
                    &mut attributes,
                    add_attributes,
                    codegen_cfg,
                );

                // later on in the generation process, modules will be created
                if !target_meta.child_archetypes().is_empty() {
                    target_meta.impl_mod(&format!(
                        "Meta-objects for {} and its subtypes.",
                        target_name.to_camel_case()
                    ));
                }
            }
        }
    }

    KBInitConfig {
        concepts_to_initialize,
        attributes,
        yin: codegen_cfg.yin,
    }
}

/// Create initialization file for newly defined concepts.
pub fn handle_init(archetype_requests: &mut [Implement], codegen_cfg: &CodegenConfig) {
    let code = code_init(&init_config(archetype_requests, codegen_cfg));
    output_code(&code, "src/tao/auto_init.rs", codegen_cfg);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::initialize_kb;
    use zamm_yin::tao::archetype::AttributeArchetypeFormTrait;
    use zamm_yin::tao::form::{Form, FormTrait};
    use zamm_yin::tao::relation::attribute::Attribute;

    #[test]
    fn test_single_new_concept() {
        initialize_kb();
        Crate::yang().set_version("0.1.7");
        let mut impls = vec![];
        let mut implement = Implement::new();
        let mut new_concept = Form::archetype().individuate_as_archetype();
        new_concept.set_internal_name_str("Bobby");
        implement.set_target(&new_concept.as_form());
        impls.push(implement);
        let cfg = init_config(&mut impls, &CodegenConfig::default());
        assert_eq!(
            cfg.concepts_to_initialize.first().map(|s| s.name.clone()),
            Some("Bobby".to_owned())
        );
        assert_eq!(cfg.attributes, Vec::<Link>::new());
    }

    #[test]
    fn test_concept_attr() {
        initialize_kb();
        Crate::yang().set_version("0.1.7");
        let mut impls = vec![];

        let mut new_attr = Attribute::archetype().individuate_as_archetype();
        new_attr.set_internal_name_str("Name");
        let mut owner = Form::archetype().individuate_as_archetype();
        owner.set_internal_name_str("Bobby");
        let mut value = Form::archetype().individuate_as_archetype();
        value.set_internal_name_str("Word");
        owner.add_attribute(new_attr);
        new_attr.set_owner_archetype(owner);
        new_attr.set_value_archetype(value);

        let mut implement_attr = Implement::new();
        implement_attr.set_target(&new_attr.as_form());
        impls.push(implement_attr);
        let mut implement_owner = Implement::new();
        implement_owner.set_target(&owner.as_form());
        impls.push(implement_owner);
        let mut implement_value = Implement::new();
        implement_value.set_target(&value.as_form());
        impls.push(implement_value);

        let cfg = init_config(&mut impls, &CodegenConfig::default());
        assert_eq!(
            cfg.concepts_to_initialize
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<&str>>(),
            vec!["Name", "Bobby", "Word"]
        );
        assert_eq!(
            cfg.attributes
                .iter()
                .map(|l| l.as_tuple())
                .collect::<Vec<(&str, &str, &str)>>(),
            vec![
                ("Name", "OwnerArchetype", "Bobby"),
                ("Name", "ValueArchetype", "Word"),
                ("Bobby", "HasAttribute", "Name"),
            ]
        );
    }

    #[test]
    fn test_generic_attr() {
        initialize_kb();
        Crate::yang().set_version("0.1.7");
        let mut impls = vec![];

        let mut new_attr = Attribute::archetype().individuate_as_archetype();
        new_attr.set_internal_name_str("Name");

        let mut implement_attr = Implement::new();
        implement_attr.set_target(&new_attr.as_form());
        impls.push(implement_attr);

        let cfg = init_config(&mut impls, &CodegenConfig::default());
        // it should avoid outputting inherited attributes introduced by its ancestors
        assert_eq!(
            cfg.attributes
                .iter()
                .map(|l| l.as_tuple())
                .collect::<Vec<(&str, &str, &str)>>(),
            Vec::<(&str, &str, &str)>::new()
        );
    }
}
