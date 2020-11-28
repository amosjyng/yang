use crate::codegen::docstring::into_docstring;
use crate::codegen::template::basic::{
    AtomicFragment, FileFragment, FunctionFragment, ItemDeclarationAPI,
};
use crate::codegen::{StructConfig, CODE_WIDTH};
use crate::tao::form::{Crate, CrateExtension};
use indoc::formatdoc;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

/// Configuration values for KB initialization template.
#[derive(Default)]
pub struct KBInitConfig {
    /// The list of concepts to be initialized.
    pub concepts_to_initialize: Vec<StructConfig>,
}

/// Get the function that initializes concept types.
///
/// The caller will still have to choose which graph binding they want to use, and initialize it
/// themselves. The caller will also have to initialize archetype relations themselves.
fn init_types_fragment(cfg: &KBInitConfig) -> FunctionFragment {
    let mut init_fn = FunctionFragment::new("initialize_types".to_owned());
    init_fn.mark_as_public();
    init_fn.document("Adds all concepts to knowledge graph.".to_owned());

    init_fn.add_import("zamm_yin::graph::InjectionGraph".to_owned());
    init_fn.add_import("zamm_yin::graph::Graph".to_owned());
    init_fn.add_import("zamm_yin::initialize_type".to_owned());
    init_fn.add_import("zamm_yin::tao::archetype::ArchetypeTrait".to_owned());
    init_fn.add_import("zamm_yin::tao::relation::attribute::Inherits".to_owned());
    for concept in &cfg.concepts_to_initialize {
        init_fn.add_import(concept.import.clone());
    }

    let concepts_list = cfg
        .concepts_to_initialize
        .iter()
        .map(|c| &c.name)
        .format(",\n        ")
        .to_string();

    init_fn.append(Rc::new(RefCell::new(AtomicFragment::new(formatdoc! {"
        let mut ig = InjectionGraph::new();
        #[rustfmt::skip]
        initialize_type!(
            ig,
            (
                {concepts}
            )
        );
    ", concepts = concepts_list}))));

    init_fn
}

/// Defines the number of concepts generated.
fn max_id_fragment(cfg: &KBInitConfig) -> AtomicFragment {
    let max_id_doc = into_docstring("The maximum concept ID inside the types distributed by Yin itself. App-specific type concepts should continue their numbering on top of this.", CODE_WIDTH);
    AtomicFragment::new(formatdoc! {"
        {doc}
        pub const YIN_MAX_ID: usize = {concepts_size};
    ", doc = max_id_doc,
        concepts_size = cfg.concepts_to_initialize.len() - 1 // -1 because IDs are zero-indexed
    })
}

/// Generate code for the init file.
pub fn code_init(cfg: &KBInitConfig) -> String {
    let mut file = FileFragment::default();
    file.append(Rc::new(RefCell::new(max_id_fragment(cfg)))); // always define, even if unused
    file.append(Rc::new(RefCell::new(init_types_fragment(cfg))));
    file.set_current_crate(Crate::current().implementation_name().unwrap());
    file.generate_code()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;
    use crate::tao::initialize_kb;
    use indoc::indoc;

    #[test]
    fn test_init_one_concept() {
        assert_eq!(
            init_types_fragment(&KBInitConfig {
                concepts_to_initialize: vec![StructConfig {
                    name: "Me".to_owned(),
                    import: "crate::people::Me".to_owned(),
                }]
            })
            .body(),
            indoc! {"
            /// Adds all concepts to knowledge graph.
            pub fn initialize_types() {
                let mut ig = InjectionGraph::new();
                #[rustfmt::skip]
                initialize_type!(
                    ig,
                    (
                        Me
                    )
                );
            }"}
        );
    }

    #[test]
    fn test_init_multiple_concepts() {
        assert_eq!(
            init_types_fragment(&KBInitConfig {
                concepts_to_initialize: vec![
                    StructConfig {
                        name: "Me".to_owned(),
                        import: "zamm_yin::people::Me".to_owned(),
                    },
                    StructConfig {
                        name: "You".to_owned(),
                        import: "crate::people::You".to_owned(),
                    },
                    StructConfig {
                        name: "Us".to_owned(),
                        import: "crate::groups::Us".to_owned(),
                    }
                ]
            })
            .body(),
            indoc! {"
            /// Adds all concepts to knowledge graph.
            pub fn initialize_types() {
                let mut ig = InjectionGraph::new();
                #[rustfmt::skip]
                initialize_type!(
                    ig,
                    (
                        Me,
                        You,
                        Us
                    )
                );
            }"}
        );
    }

    #[test]
    fn test_init_file() {
        initialize_kb();
        Crate::current().set_implementation_name("zamm_yin");
        let code = code_init(&KBInitConfig {
            concepts_to_initialize: vec![
                StructConfig {
                    name: "Me".to_owned(),
                    import: "zamm_yin::people::Me".to_owned(),
                },
                StructConfig {
                    name: "You".to_owned(),
                    import: "crate::people::You".to_owned(),
                },
                StructConfig {
                    name: "Us".to_owned(),
                    import: "crate::groups::Us".to_owned(),
                },
            ],
        });
        assert!(code.contains("YIN_MAX_ID: usize = 2"));
        assert!(!code.contains("zamm_yin"));
    }
}
