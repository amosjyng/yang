use crate::codegen::template::basic::{AtomicFragment, FunctionFragment, SelfReference};
use crate::codegen::StructConfig;
use std::cell::RefCell;
use std::rc::Rc;

/// Config values at the time of Flag getter/setter code generation.
pub struct FlagConfig {
    /// The public name to serve as a basis for the getter/setter function names.
    pub property_name: Rc<str>,
    /// An explanation of what this property is. Will be prepended by different strings for the
    /// setter and getter documentation.
    pub doc: Rc<str>,
    /// Concept representing the flag.
    pub flag: StructConfig,
    /// String to use for the Yin crate.
    pub yin_crate: Rc<str>,
}

impl Default for FlagConfig {
    fn default() -> Self {
        Self {
            property_name: Rc::from(""),
            doc: Rc::from(""),
            flag: StructConfig::default(),
            yin_crate: Rc::from("zamm_yin"),
        }
    }
}

/// Get the setter fragment for the flag.
fn setter_fragment(cfg: &FlagConfig) -> FunctionFragment {
    let mut f = FunctionFragment::new(format!("mark_{}", cfg.property_name));
    f.set_documentation(format!("Mark this as {}", cfg.doc));
    f.set_self_reference(SelfReference::Mutable);
    f.add_import(cfg.flag.import.clone());
    f.add_import(format!("{}::tao::archetype::ArchetypeTrait", cfg.yin_crate));
    f.add_import(format!("{}::tao::form::FormTrait", cfg.yin_crate));
    f.append(Rc::new(RefCell::new(AtomicFragment::new(format!(
        "self.essence_mut().add_flag({}::TYPE_ID);",
        cfg.flag.name
    )))));
    f
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::CodeFragment;
    use indoc::indoc;

    #[test]
    fn test_setter_fragment_body() {
        assert_eq!(
            setter_fragment(&FlagConfig {
                property_name: Rc::from("newly_defined"),
                doc: Rc::from("newly defined as part of the current build."),
                flag: StructConfig {
                    name: "NewlyDefined".to_owned(),
                    import: "crate::tao::relation::flag::NewlyDefined".to_owned(),
                },
                yin_crate: Rc::from("zamm_yin"),
            })
            .body(),
            indoc! {"
            /// Mark this as newly defined as part of the current build.
            fn mark_newly_defined(&mut self) {
                self.essence_mut().add_flag(NewlyDefined::TYPE_ID);
            }"}
        );
    }
}
