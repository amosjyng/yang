use super::{concept_to_struct, grab_new_implementation_id};
use crate::codegen::template::concept::auto_init_kb::{code_init, KBInitConfig};
use crate::codegen::{output_code, CodegenConfig, StructConfig};
use crate::tao::{Implement, ImplementExtension};
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::Archetype;

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
