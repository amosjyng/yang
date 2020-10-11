use zamm_yang::codegen::track_autogen::save_autogen;
use zamm_yang::codegen::{handle_implementation, CodegenConfig};
use zamm_yang::concepts::{initialize_kb, Implement, ImplementConfig};
use zamm_yin::concepts::{ArchetypeTrait, Tao};
use zamm_yin::node_wrappers::CommonNodeTrait;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    initialize_kb();

    let mut target = Tao::archetype().individuate_as_archetype();
    target.set_internal_name("Target".to_string());

    let mut implement_command = Implement::individuate();
    implement_command.set_target(target);
    implement_command.set_config(ImplementConfig {
        name: "Target".to_owned(),
        doc: Some("The target of an implement command.".to_owned()),
        id: 1,
    });

    handle_implementation(
        implement_command,
        &CodegenConfig {
            comment_autogen: true,
            yin: false,
        },
    );
    save_autogen(); // keep track of generated files for later cleaning
}
