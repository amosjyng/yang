mod concepts {
    pub mod attributes {
        pub mod target;
    }
}

use concepts::attributes::target::Target;
use zamm_yin::concepts::attributes::Inherits;
use zamm_yin::concepts::ArchetypeTrait;
use zamm_yin::graph::{bind_in_memory_graph, Graph, InjectionGraph};
use zamm_yin::node_wrappers::CommonNodeTrait;

fn main() {
    // Initialize Yin KB with new type
    bind_in_memory_graph();
    let mut ig = InjectionGraph::new();
    zamm_yin::initialize_type!(ig, (Target));

    let mut target = Target::individuate();
    target.set_internal_name("Hello, world.".to_string());
    println!("{}", target.internal_name().unwrap());
}
