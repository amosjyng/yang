use super::{ancestor_path, in_own_submodule};
use crate::codegen::template::concept::archetype_module::{
    code_archetype_module, ArchetypeModuleConfig,
};
use crate::tao::archetype::CodegenFlags;
use crate::tao::form::{BuildInfo, BuildInfoExtension};
use crate::tao::form::{Module, ModuleExtension};
use crate::tao::{Implement, ImplementExtension};
use std::rc::Rc;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;

/// Generate code for a given module. Post-processing still needed.
pub fn code_module(request: Implement, module: Module, parent: Archetype) -> String {
    let mut archetype_names = vec![];
    let mut public_submodules = vec![];
    let mut private_submodules = vec![];
    let mut re_exports = vec![];

    if parent.is_newly_defined() {
        archetype_names.push(Rc::from(parent.internal_name().unwrap().as_str()));
    } else {
        // Parent is already defined as part of a dependency, we're only creating this crate so
        // that we can access the children as well. In which case, we should also re-export the
        // concepts defined in the dependency, so that the end consumer does not depend directly on
        // the dependency.
        re_exports.push(format!("zamm_yin::{}::*", ancestor_path(&parent, "::")));
    }

    for child in parent.child_archetypes() {
        if in_own_submodule(&child) {
            let child_submodule = match BuildInfo::from(child.id()).representative_module() {
                Some(existing_module) => existing_module,
                None => {
                    let mut new_submodule = Module::new();
                    new_submodule.set_most_prominent_member(&child.as_form());
                    new_submodule
                }
            };
            public_submodules.push(
                (*ModuleExtension::implementation_name(&child_submodule).unwrap()).to_owned(),
            );
        } else if child.is_newly_defined() {
            archetype_names.push(Rc::from(child.internal_name().unwrap().as_str()));
        } // else, if this child doesn't have their own module, and has also been already defined,
          // then we will re-export them later in this function
    }

    for submodule in module.submodules() {
        private_submodules.push((*submodule.implementation_name().unwrap()).to_owned());
    }

    for re_export in module.re_exports() {
        re_exports.push((*re_export).to_owned());
    }

    code_archetype_module(&ArchetypeModuleConfig {
        doc: request.documentation(),
        archetype_names,
        private_submodules,
        public_submodules,
        re_exports,
    })
}