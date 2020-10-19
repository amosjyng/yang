use zamm_yang::codegen::track_autogen::save_autogen;
use zamm_yang::codegen::CodegenConfig;
use zamm_yang::tao::callbacks::handle_implementation;
use zamm_yang::tao::{initialize_kb, Implement, ImplementConfig};
use zamm_yin::tao::archetype::{ArchetypeTrait, ArchetypeFormTrait};
use zamm_yin::tao::attribute::Attribute;
use zamm_yin::node_wrappers::CommonNodeTrait;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    initialize_kb();

    let mut target = Attribute::archetype().individuate_as_archetype();
    target.set_internal_name("Target".to_string());

    let mut implement_command = Implement::individuate();
    implement_command.set_target(target.as_archetype());
    implement_command.set_config(ImplementConfig {
        id: 1,
        doc: Some("The target of an implement command.".to_owned()),
    });

    handle_implementation(implement_command, &CodegenConfig::default());
    save_autogen(); // keep track of generated files for later cleaning
}
