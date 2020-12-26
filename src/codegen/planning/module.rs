use super::{ancestor_path, in_own_submodule};
use crate::codegen::template::concept::archetype_module::{
    code_archetype_module, ArchetypeModuleConfig,
};
use crate::tao::form::{Module, ModuleExtension};
use crate::tao::perspective::{BuildInfo, BuildInfoExtension, KnowledgeGraphNode};
use crate::tao::Implement;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait};

/// Generate code for a given module. Post-processing still needed.
pub fn code_module(request: Implement, module: Module, parent: Archetype) -> String {
    let mut archetype_names = vec![];
    let mut public_submodules = vec![];
    let mut private_submodules = vec![];
    let mut re_exports = vec![];

    if KnowledgeGraphNode::from(parent.id()).is_newly_defined() {
        archetype_names.push(parent.internal_name_str().unwrap());
    } else {
        // Parent is already defined as part of a dependency, we're only creating this crate so
        // that we can access the children as well. In which case, we should also re-export the
        // concepts defined in the dependency, so that the end consumer does not depend directly on
        // the dependency.
        //
        // However, we shouldn't simply re-export the Yin submodule, because otherwise we will end
        // up with duplicate submodules, and that messes up rustdoc generation. So, we should
        // re-export only the children that were defined in Yin.
        //
        // Re-exporting all concepts that aren't newly defined won't work, because this includes a
        // lot of the concepts that are loaded on Yang startup. They were newly defined when the
        // generating Yang was first built, but now that that version of Yang is built and building
        // the next version of Yang in turn, those previously built concepts are no longer newly
        // defined. Moreover, they were defined as part of previous Yang, not previous Yin, so
        // re-exporting them as part of Yin won't work.
        //
        // Storing Yin build information during Yin generation, and then loading that back up
        // on Yang startup, won't work either because that's previous Yin's build information.
        // We're building for a future Yang that might depend on a future Yin, not on the Yin that
        // the current Yang depends on, so we can't just rely on the build information stored in
        // the Yin dependency.
        //
        // Instead, this needs to be done properly as part of an import of Yin's `yin.md` file from
        // within Yang's `yin.md` file. Only then will the diff between Yin and Yang be accurate.
        re_exports.push(format!("zamm_yin::{}::*", ancestor_path(&parent, "::")));
    }

    for child in parent.child_archetypes() {
        if in_own_submodule(&child) {
            let child_submodule = match BuildInfo::from(child.id()).representative_module() {
                Some(existing_module) => existing_module,
                None => continue,
            };
            public_submodules.push(
                (*ModuleExtension::implementation_name(&child_submodule).unwrap()).to_owned(),
            );
        } else if KnowledgeGraphNode::from(child.id()).is_newly_defined() {
            archetype_names.push(child.internal_name_str().unwrap());
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
