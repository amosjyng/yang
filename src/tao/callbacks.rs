use crate::codegen::planning::{
    archetype_file_path, code_archetype, code_module, handle_init, module_file_path,
};
use crate::codegen::track_autogen::save_autogen;
use crate::codegen::{output_code, CodegenConfig};
use crate::tao::action::Implement;
use crate::tao::form::rust_item::{Concept, Crate, CrateExtension, Module};
use crate::tao::perspective::KnowledgeGraphNode;
use colored::*;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;

/// Retrieve all non-imported implement actions.
pub fn implements() -> Box<dyn Iterator<Item = Implement>> {
    Box::new(
        Implement::archetype()
            .individuals()
            .into_iter()
            .filter(|i| !KnowledgeGraphNode::from(i.id()).is_imported())
            .map(|i| Implement::from(i.id())),
    )
}

/// Retrieve implementation requests that pertain to archetypes.
fn archetypes_to_implement() -> Vec<Implement> {
    implements()
        .filter(|i| {
            i.embodiment()
                .unwrap()
                .has_ancestor(Concept::archetype().into())
        })
        .collect()
}

/// Retrieve implementation requests that pertain to modules.
fn modules_to_implement() -> Vec<Implement> {
    implements()
        .filter(|i| {
            i.embodiment()
                .unwrap()
                .has_ancestor(Module::archetype().into())
        })
        .collect()
}

/// Handle the implementation request for a new archetype.
fn handle_archetype_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let code = code_archetype(request, codegen_cfg);
    let target_type = Archetype::from(request.target().unwrap().id());
    output_code(&code, &archetype_file_path(&target_type), codegen_cfg);
}

/// Handle the implementation request for a new module.
fn handle_module_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let target_module = Module::from(request.embodiment().unwrap().id());
    let primary_archetype = Archetype::from(target_module.most_prominent_member().unwrap().id());
    let code = code_module(request, target_module, primary_archetype);
    output_code(&code, &module_file_path(&primary_archetype), codegen_cfg);
}

/// Handle all defined implementation requests.
pub fn handle_all_implementations(codegen_cfg: &CodegenConfig) {
    let mut current_build = Crate::current();
    if current_build.implementation_name().is_none() {
        println!(
            "{}It is now recommended to explicitly specify the current crate name for import cleanup.",
            "Warning: ".yellow().bold()
        );
        if codegen_cfg.yin {
            current_build.set_implementation_name("zamm_yin");
        } else {
            current_build.set_implementation_name("DUMMY-CUSTOM-CRATE");
        }
    }

    let mut initial_archetype_requests = archetypes_to_implement();
    // handle initialization first to ensure all concepts land with the right concept IDs, and to
    // make sure all implement commands get created, even the ones that are implicitly defined
    handle_init(&mut initial_archetype_requests, codegen_cfg);
    // handle_init might create new implement commands
    let final_archetype_requests = archetypes_to_implement();
    for implement_command in final_archetype_requests {
        handle_archetype_implementation(implement_command, codegen_cfg);
    }
    for implement_command in modules_to_implement() {
        handle_module_implementation(implement_command, codegen_cfg);
    }

    save_autogen();
}
