use zamm_yang::codegen::handle_implementation;
use zamm_yang::concepts::{initialize_kb, set_documentation, Implement};
use zamm_yin::concepts::{ArchetypeTrait, Tao};
use zamm_yin::wrappers::CommonNodeTrait;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    initialize_kb();

    let mut target = Tao::archetype().individuate_as_archetype();
    target.set_internal_name("Target".to_string());
    set_documentation(Box::new(&mut target), "The target of an implement command.");

    let mut implement_command = Implement::individuate();
    implement_command.set_target(target);

    handle_implementation(implement_command, 1, true);
}
