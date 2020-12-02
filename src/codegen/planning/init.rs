use super::{concept_to_struct, grab_new_implementation_id};
use crate::codegen::template::concept::auto_init_kb::{code_init, KBInitConfig, Link};
use crate::codegen::{output_code, CodegenConfig, StructConfig};
use crate::tao::form::{Crate, CrateExtension};
use crate::tao::{Implement, ImplementExtension};
use zamm_yin::node_wrappers::BaseNodeTrait;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{
    Archetype, ArchetypeFormExtensionTrait, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype,
};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::has_property::{HasAttribute, HasFlag};
use zamm_yin::tao::relation::attribute::{Attribute, OwnerArchetype, ValueArchetype};
use zamm_yin::tao::relation::Relation;
use zamm_yin::Wrapper;

/// Create initialization file for newly defined concepts.
pub fn handle_init(archetype_requests: &[Implement], codegen_cfg: &CodegenConfig) {
    let mut concepts_to_initialize = Vec::<StructConfig>::new();
    let mut attributes = Vec::<Link>::new();
    let add_attributes = Crate::yang().version_at_least(0, 1, 7);

    let has_attr = concept_to_struct(&HasAttribute::archetype().into(), codegen_cfg.yin);
    let has_flag = concept_to_struct(&HasFlag::archetype().into(), codegen_cfg.yin);
    let owner_type = concept_to_struct(&OwnerArchetype::archetype().into(), codegen_cfg.yin);
    let value_type = concept_to_struct(&ValueArchetype::archetype().into(), codegen_cfg.yin);

    for implement_command in archetype_requests {
        let mut implement = Implement::from(implement_command.id());
        let target_type = Archetype::from(implement.target().unwrap().id());
        let target_struct = concept_to_struct(&target_type, codegen_cfg.yin);
        concepts_to_initialize.push(target_struct.clone());

        // only set ID for user if user hasn't already set it
        if implement.implementation_id().is_none() {
            implement.set_implementation_id(grab_new_implementation_id(codegen_cfg.yin));
        }

        // only initialize concept attributes if user hasn't already done it because they were
        // using an older version of Yang
        if add_attributes {
            let aa = AttributeArchetype::from(target_type.id());
            let target_struct = concept_to_struct(&target_type, codegen_cfg.yin);

            if target_type.has_ancestor(Relation::archetype()) {
                // we want to see if it has a custom owner, not an inherited one
                let owners = aa
                    .essence()
                    .base_wrapper()
                    .outgoing_nodes(OwnerArchetype::TYPE_ID);
                if let Some(owner) = owners.first() {
                    println!("Owner of {:?} is {:?}", aa, Archetype::from(owner.id()));
                    let owner_struct =
                        concept_to_struct(&Archetype::from(owner.id()), codegen_cfg.yin);
                    attributes.push(Link {
                        from: target_struct.clone(),
                        link_type: owner_type.clone(),
                        to: owner_struct,
                    });
                }
            }

            if target_type.has_ancestor(Attribute::archetype().into()) {
                // we want to see if it has a custom value, not an inherited one
                let values = aa
                    .essence()
                    .base_wrapper()
                    .outgoing_nodes(ValueArchetype::TYPE_ID);
                if let Some(value) = values.first() {
                    let value_struct =
                        concept_to_struct(&Archetype::from(value.id()), codegen_cfg.yin);
                    attributes.push(Link {
                        from: target_struct.clone(),
                        link_type: value_type.clone(),
                        to: value_struct,
                    });
                }
            }

            for flag in target_type.added_flags() {
                attributes.push(Link {
                    from: target_struct.clone(),
                    link_type: has_flag.clone(),
                    to: concept_to_struct(&flag.into(), codegen_cfg.yin),
                });
            }

            for attr in target_type.added_attributes() {
                attributes.push(Link {
                    from: target_struct.clone(),
                    link_type: has_attr.clone(),
                    to: concept_to_struct(&attr.into(), codegen_cfg.yin),
                });
            }
        }
    }

    let code = code_init(&KBInitConfig {
        concepts_to_initialize,
        attributes,
    });
    output_code(&code, "src/tao/auto_init.rs", codegen_cfg);
}
