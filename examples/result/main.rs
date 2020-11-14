mod tao;

use tao::relation::attribute::Target;
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::ArchetypeTrait;

fn main() {
    tao::initialize_kb();

    let mut target = Target::individuate();
    target.set_internal_name("Hello, world.".to_string());
    println!("{}", target.internal_name().unwrap());
}
