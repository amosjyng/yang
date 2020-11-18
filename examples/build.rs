use zamm_yang::codegen::CodegenConfig;
use zamm_yang::define;
use zamm_yang::tao::archetype::CreateImplementation;
use zamm_yang::tao::callbacks::handle_all_implementations;
use zamm_yang::tao::form::BuildInfoExtension;
use zamm_yang::tao::form::DefinedMarker;
use zamm_yang::tao::initialize_kb;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{ArchetypeFormTrait, ArchetypeTrait};
use zamm_yin::tao::form::FormTrait;
use zamm_yin::tao::relation::attribute::Attribute;
use zamm_yin::tao::Tao;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    initialize_kb();

    Attribute::archetype()
        .build_info()
        .set_import_path("zamm_yin::tao::relation::attribute::Attribute");

    define!(target);
    target.add_parent(Attribute::archetype().as_archetype());
    target.set_internal_name("Target".to_string());
    target.implement_with_doc("The target of an implement command.");

    handle_all_implementations(&CodegenConfig::default());
}
