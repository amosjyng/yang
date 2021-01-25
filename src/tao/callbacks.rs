use crate::codegen::planning::imports::import_to_file_path;
use crate::codegen::planning::{
    archetype_file_path, code_archetype, code_archetype_trait, code_module, handle_init,
    in_own_submodule, module_file_path,
};
use crate::codegen::track_autogen::save_autogen;
use crate::codegen::{output_code, CodegenConfig};
use crate::tao::action::Implement;
use crate::tao::archetype::CreateImplementation;
use crate::tao::form::rust_item::{Concept, Crate, CrateExtension, Module, Trait};
use crate::tao::perspective::{BuildInfo, KnowledgeGraphNode};
use colored::*;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;

/// Retrieve all implement actions, regardless of whether they were imported or not.
fn all_implements() -> Box<dyn Iterator<Item = Implement>> {
    Box::new(
        Implement::archetype()
            .individuals()
            .into_iter()
            .map(|i| Implement::from(i.id())),
    )
}

/// Retrieve implement actions, with an optional filter for excluding imported implementation
/// requests.
pub fn implements(exclude_imported: bool) -> Box<dyn Iterator<Item = Implement>> {
    if exclude_imported {
        Box::new(all_implements().filter(|i| !KnowledgeGraphNode::from(i.id()).is_imported()))
    } else {
        all_implements()
    }
}

/// Retrieve implementation requests that pertain to a certain embodiment type.
fn items_to_implement(exclude_imported: bool, embodiment_type: &Archetype) -> Vec<Implement> {
    implements(exclude_imported)
        .filter(|i| i.embodiment().unwrap().has_ancestor(*embodiment_type))
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

/// Handle the implementation request for a new trait.
fn handle_trait_implementation(request: &Implement, codegen_cfg: &CodegenConfig) {
    let target_trait = Trait::from(request.embodiment().unwrap().id());
    let mut target_type = Archetype::from(request.target().unwrap().id());
    let code = code_archetype_trait(&mut target_type, &target_trait, &codegen_cfg);
    output_code(
        &code,
        &import_to_file_path(&BuildInfo::from(target_trait.id()).import_path().unwrap()),
        codegen_cfg,
    );
}

/// Make sure trait implementations are successfully recorded.
fn ensure_trait_impls(target_type: &mut Archetype) {
    if in_own_submodule(&target_type)
        && (!target_type.added_flags().is_empty() || !target_type.added_attributes().is_empty())
    {
        if target_type.as_trait_implementation().is_none() {
            // then create the implementation for the first time
            target_type.impl_trait();
        }
    }
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

    let mut initial_archetype_requests = items_to_implement(true, &Concept::archetype().into());
    // handle initialization first to ensure all concepts land with the right concept IDs, and to
    // make sure all implement commands get created, even the ones that are implicitly defined
    handle_init(&mut initial_archetype_requests, codegen_cfg);
    // handle_init was only called on new implementations. We should make sure all implement
    // requests are captured, because otherwise we won't know which traits have been requested
    for implementation in items_to_implement(false, &Concept::archetype().into()) {
        ensure_trait_impls(&mut Archetype::from(implementation.target().unwrap().id()));
    }

    // handle_init and ensure_trait_impls might create new implement commands
    let final_archetype_requests = items_to_implement(true, &Concept::archetype().into());
    for implement_command in final_archetype_requests {
        handle_archetype_implementation(implement_command, codegen_cfg);
    }
    for implement_command in items_to_implement(true, &Module::archetype().into()) {
        handle_module_implementation(implement_command, codegen_cfg);
    }
    for implement_command in items_to_implement(true, &Trait::archetype().into()) {
        handle_trait_implementation(&implement_command, codegen_cfg);
    }

    save_autogen();
}
