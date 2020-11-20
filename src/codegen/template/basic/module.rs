use super::{AppendedFragment, AtomicFragment, CodeFragment, FileFragment, NestedFragment};
use crate::codegen::template::imports::{imports_as_str, re_exports_as_str};
use indoc::formatdoc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

/// Fragment for a module declaration.
#[derive(Default)]
pub struct ModuleFragment {
    name: Option<String>,
    public: bool,
    test: bool,
    declare_only: bool,
    uses_entire_file: bool,
    re_exports: Vec<String>,
    submodules: Vec<Rc<RefCell<ModuleFragment>>>,
    content: Rc<RefCell<AppendedFragment>>,
}

impl ModuleFragment {
    /// Create a new module with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name: Some(name),
            ..ModuleFragment::default()
        }
    }

    /// Create a new module that takes up an entire file.
    pub fn new_file_module() -> Self {
        Self {
            uses_entire_file: true,
            ..ModuleFragment::default()
        }
    }

    /// Create a new module with the name `tests`.
    pub fn new_test_module() -> Self {
        let mut new_self = Self::new("tests".to_owned());
        new_self.mark_as_test();
        new_self
    }

    /// Mark a module as being a publicly accessible one.
    pub fn mark_as_public(&mut self) {
        self.public = true;
    }

    /// Causes the generated code to only declare the existence of a module, without actually
    /// specifying any of its contents.
    ///
    /// This should be done when a module is implemented externally inside its own `.rs.` file,
    /// because the contents will reside inside that module's `.rs` file instead.
    pub fn mark_as_declare_only(&mut self) {
        self.declare_only = true;
    }

    /// Mark the module as requiring full implementation.
    pub fn mark_for_full_implementation(&mut self) {
        self.declare_only = false;
    }

    /// Adds a submodule to the current module.
    pub fn add_submodule(&mut self, name: String) -> Rc<RefCell<ModuleFragment>> {
        let mut new_submodule = Self::new(name);
        // unlike the top-level module, submodules start out as declare-only. It is still possible
        // to add submodules manually via the `append` function instead.
        new_submodule.mark_as_declare_only();
        let module_rc = Rc::new(RefCell::new(new_submodule));
        self.submodules.push(module_rc.clone());
        module_rc
    }

    /// Mark a module as being a test module.
    pub fn mark_as_test(&mut self) {
        self.test = true;
    }

    /// Mark this for generation as an entire file. Skips generation of the mod preamble.
    pub fn mark_as_entire_file(&mut self) {
        self.uses_entire_file = true;
    }

    /// Re-export something so that it looks like it comes from this module.
    ///
    /// This usually, but not always, refers to something hidden within a submodule of this module.
    /// You can also re-export things from other modules, and they will also appear as if they came
    /// from this one.
    pub fn re_export(&mut self, export: String) {
        self.re_exports.push(export);
    }

    /// Add a fragment to the internals of this module.
    pub fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.content.borrow_mut().append(fragment);
    }

    fn submodules_by_publicity(&self, publicity: bool) -> AppendedFragment {
        let mut submodules_subset = self
            .submodules
            .iter()
            .filter(|s| s.borrow().public == publicity)
            .collect::<Vec<&Rc<RefCell<ModuleFragment>>>>();
        submodules_subset.sort();
        // single newline separation between short module declarations
        // todo: separate by declaration-only versus full-impl as well
        let mut submodules_frag = AppendedFragment::new_with_separator("\n");
        for module in submodules_subset {
            submodules_frag.append(module.clone());
        }
        submodules_frag
    }
}

impl Ord for ModuleFragment {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for ModuleFragment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ModuleFragment {
    fn eq(&self, other: &Self) -> bool {
        // in context, we'll only be comparing modules at the same level with each other
        self.name == other.name
    }
}

impl Eq for ModuleFragment {}

impl CodeFragment for ModuleFragment {
    fn body(&self) -> String {
        let public = if self.public { "pub " } else { "" };
        let cfg_test = if self.test { "#[cfg(test)]" } else { "" };

        if self.declare_only {
            format!(
                "{public}mod {name};",
                public = public,
                name = self.name.as_ref().unwrap()
            )
        } else {
            let mut imports = self.content.borrow().imports();
            if self.test {
                imports.push("super::*".to_owned());
            }
            let mut imports_str =
                imports_as_str(&imports.iter().map(|s| s.as_str()).collect::<Vec<&str>>());
            if !imports_str.is_empty() {
                imports_str += "\n\n";
            }

            let mut re_exports_str = re_exports_as_str(
                &self
                    .re_exports
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<&str>>(),
            );
            if !re_exports_str.is_empty() {
                re_exports_str += "\n\n";
            }

            let mut submodules_frag = AppendedFragment::default();
            submodules_frag.append(Rc::new(RefCell::new(self.submodules_by_publicity(true))));
            submodules_frag.append(Rc::new(RefCell::new(self.submodules_by_publicity(false))));

            let mut internals = AppendedFragment::default();
            internals.append(Rc::new(RefCell::new(submodules_frag)));
            if !imports_str.is_empty() {
                internals.append(Rc::new(RefCell::new(AtomicFragment::new(imports_str))));
            }
            if !re_exports_str.is_empty() {
                internals.append(Rc::new(RefCell::new(AtomicFragment::new(re_exports_str))));
            }
            internals.append(self.content.clone());
            let internals_rc = Rc::new(RefCell::new(internals));

            if self.uses_entire_file {
                let mut f = FileFragment::new();
                f.append(internals_rc);
                f.generate_code()
            } else {
                // todo: try using FileFragment to generate the internals here too
                let nested = NestedFragment {
                    imports: Vec::new(),
                    preamble: formatdoc! {"
                        {cfg_test}
                        {public}mod {name} {{",
                        public = public,
                        name = self.name.as_ref().unwrap(),
                        cfg_test = cfg_test,
                    },
                    nesting: Some(internals_rc),
                    postamble: "}".to_owned(),
                };
                nested.body()
            }
        }
    }

    fn imports(&self) -> Vec<String> {
        Vec::new() // module-level imports should never make it out of the module
    }
}

#[cfg(test)]
mod tests {
    use super::super::AtomicFragment;
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_regular_module() {
        let mut test_mod = ModuleFragment::new("my_mod".to_owned());
        test_mod.append(Rc::new(RefCell::new(AtomicFragment {
            imports: Vec::new(),
            atom: indoc! {r#"
                fn a() {
                    println!("actually b");
                }"#}
            .to_string(),
        })));

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {r#"
                mod my_mod {
                    fn a() {
                        println!("actually b");
                    }
                }"#}
        );
    }

    #[test]
    fn test_public_module() {
        let mut test_mod = ModuleFragment::new("my_mod".to_owned());
        test_mod.mark_as_public();
        test_mod.append(Rc::new(RefCell::new(AtomicFragment {
            imports: Vec::new(),
            atom: indoc! {r#"
                fn a() {
                    println!("actually b");
                }"#}
            .to_string(),
        })));

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {r#"
                pub mod my_mod {
                    fn a() {
                        println!("actually b");
                    }
                }"#}
        );
    }

    #[test]
    fn test_declared_module() {
        let mut test_mod = ModuleFragment::new("my_mod".to_owned());
        test_mod.mark_as_declare_only();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(test_mod.body(), "mod my_mod;".to_owned());
    }

    #[test]
    fn test_publicly_declared_module() {
        let mut test_mod = ModuleFragment::new("my_mod".to_owned());
        test_mod.mark_as_declare_only();
        test_mod.mark_as_public();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(test_mod.body(), "pub mod my_mod;".to_owned());
    }

    #[test]
    fn test_submodules() {
        let mut test_mod = ModuleFragment::new("my_dom".to_owned());
        test_mod
            .add_submodule("my_sub".to_owned())
            .borrow_mut()
            .mark_as_public();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {"
                mod my_dom {
                    pub mod my_sub;
                }"}
        );
    }

    #[test]
    fn test_implemented_submodules() {
        let mut test_mod = ModuleFragment::new("my_dom".to_owned());
        test_mod
            .add_submodule("my_sub".to_owned())
            .borrow_mut()
            .mark_for_full_implementation();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {"
                mod my_dom {
                    mod my_sub {
                    }
                }"}
        );
    }

    #[test]
    fn test_sorted_submodules() {
        let mut test_mod = ModuleFragment::new("my_dom".to_owned());
        test_mod
            .add_submodule("sub_d".to_owned())
            .borrow_mut()
            .mark_as_public();
        test_mod.add_submodule("sub_b".to_owned());
        test_mod.add_submodule("sub_c".to_owned());
        test_mod
            .add_submodule("sub_a".to_owned())
            .borrow_mut()
            .mark_as_public();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {"
                mod my_dom {
                    pub mod sub_a;
                    pub mod sub_d;

                    mod sub_b;
                    mod sub_c;
                }"}
        );
    }

    #[test]
    fn test_re_exports() {
        let mut test_mod = ModuleFragment::new("my_dom".to_owned());
        test_mod.add_submodule("my_sub".to_owned());

        test_mod.re_export("my_sub::StructA".to_owned());
        test_mod.re_export("my_sub::StructB".to_owned());

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {"
                mod my_dom {
                    mod my_sub;

                    pub use my_sub::{StructA, StructB};
                }"}
        );
    }

    #[test]
    fn test_as_entire_file() {
        let mut test_mod = ModuleFragment::new_file_module();
        test_mod.add_submodule("my_sub".to_owned());

        test_mod.re_export("my_sub::StructA".to_owned());
        test_mod.re_export("my_sub::StructB".to_owned());

        test_mod.append(Rc::new(RefCell::new(AtomicFragment {
            imports: Vec::new(),
            atom: indoc! {r#"
                fn a() {
                    println!("actually b");
                }"#}
            .to_string(),
        })));

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {r#"
                mod my_sub;

                pub use my_sub::{StructA, StructB};

                fn a() {
                    println!("actually b");
                }
                "#}
        );
    }

    #[test]
    fn test_test_module() {
        let mut test_mod = ModuleFragment::new_test_module();
        test_mod.append(Rc::new(RefCell::new(AtomicFragment {
            imports: Vec::new(),
            atom: indoc! {"
                #[test]
                fn test_big() {
                    Big {
                        a: Something {},
                        b: OrTheOther {},
                    }
                }"}
            .to_string(),
        })));

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {"
                #[cfg(test)]
                mod tests {
                    use super::*;

                    #[test]
                    fn test_big() {
                        Big {
                            a: Something {},
                            b: OrTheOther {},
                        }
                    }
                }"}
        );
    }
}
