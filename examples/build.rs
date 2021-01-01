use zamm_yang::codegen::CodegenConfig;
use zamm_yang::define;
use zamm_yang::tao::archetype::CreateImplementation;
use zamm_yang::tao::callbacks::handle_all_implementations;
use zamm_yang::tao::initialize_kb;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::Attribute;
use zamm_yin::tao::Tao;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    initialize_kb();

    define!(target);
    target.add_parent(Attribute::archetype().into());
    target.set_internal_name("Target");
    target.implement_with_doc("The target of an implement command.");

    handle_all_implementations(&CodegenConfig::default());
}
