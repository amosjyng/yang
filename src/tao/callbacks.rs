use super::{Implement, ImplementExtension};
use crate::codegen::planning::{
    archetype_file_path, code_archetype, code_module, handle_init, module_file_path,
};
use crate::codegen::track_autogen::save_autogen;
use crate::codegen::{output_code, CodegenConfig};
use crate::tao::form::{Crate, CrateExtension, Module};
use colored::*;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;

/// Retrieve implementation requests that pertain to archetypes.
fn archetypes_to_implement() -> Vec<Implement> {
    Implement::archetype()
        .individuals()
        .into_iter()
        .map(|i| Implement::from(i.id()))
        .filter(|i| !i.target().unwrap().has_ancestor(Module::archetype()))
        .collect()
}

/// Retrieve implementation requests that pertain to modules.
fn modules_to_implement() -> Vec<Implement> {
    Implement::archetype()
        .individuals()
        .into_iter()
        .map(|i| Implement::from(i.id()))
        .filter(|i| i.target().unwrap().has_ancestor(Module::archetype()))
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
    let target_module = Module::from(request.target().unwrap().id());
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

    let mut archetype_requests = archetypes_to_implement();
    // handle initialization first to ensure all concepts land with the right concept IDs
    handle_init(&mut archetype_requests, codegen_cfg);
    for implement_command in archetype_requests {
        handle_archetype_implementation(implement_command, codegen_cfg);
    }
    for implement_command in modules_to_implement() {
        handle_module_implementation(implement_command, codegen_cfg);
    }

    save_autogen();
}
