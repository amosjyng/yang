use super::{AppendedFragment, AtomicFragment, CodeFragment, NestedFragment};
use crate::codegen::template::imports::imports_as_str;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for a module declaration.
pub struct ModuleFragment {
    name: String,
    public: bool,
    test: bool,
    declare_only: bool,
    content: Rc<RefCell<AppendedFragment>>,
}

impl ModuleFragment {
    /// Create a new module with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            public: false,
            test: false,
            declare_only: false,
            content: Rc::new(RefCell::new(AppendedFragment::default())),
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
        self.append(module_rc.clone());
        module_rc
    }

    /// Mark a module as being a test module.
    pub fn mark_as_test(&mut self) {
        self.test = true;
    }

    /// Add a fragment to the internals of this module.
    pub fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.content.borrow_mut().append(fragment);
    }
}

impl CodeFragment for ModuleFragment {
    fn body(&self) -> String {
        let mut imports = self.content.borrow().imports();
        if self.test {
            imports.push("super::*".to_owned());
        }
        let imports_str =
            imports_as_str(&imports.iter().map(|s| s.as_str()).collect::<Vec<&str>>());

        let public = if self.public { "pub " } else { "" };
        let cfg_test = if self.test { "#[cfg(test)]" } else { "" };

        if self.declare_only {
            format!("{public}mod {name};", public = public, name = self.name)
        } else {
            let internal_code = format!("{}\n\n{}\n", imports_str, self.content.borrow().body());
            let internals = AtomicFragment {
                imports: Vec::new(),
                atom: internal_code,
            };
            let nested = NestedFragment {
                imports: Vec::new(),
                preamble: formatdoc! {"
                    {cfg_test}
                    {public}mod {name} {{",
                    public = public,
                    name = self.name,
                    cfg_test = cfg_test,
                },
                nesting: Some(Rc::new(RefCell::new(internals))),
                postamble: "}".to_owned(),
            };
            nested.body()
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
        let mut test_mod = ModuleFragment::new("MyMod".to_owned());
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
                mod MyMod {
                    fn a() {
                        println!("actually b");
                    }
                }"#}
        );
    }

    #[test]
    fn test_public_module() {
        let mut test_mod = ModuleFragment::new("MyMod".to_owned());
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
                pub mod MyMod {
                    fn a() {
                        println!("actually b");
                    }
                }"#}
        );
    }

    #[test]
    fn test_declared_module() {
        let mut test_mod = ModuleFragment::new("MyMod".to_owned());
        test_mod.mark_as_declare_only();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(test_mod.body(), "mod MyMod;".to_owned());
    }

    #[test]
    fn test_publicly_declared_module() {
        let mut test_mod = ModuleFragment::new("MyMod".to_owned());
        test_mod.mark_as_declare_only();
        test_mod.mark_as_public();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(test_mod.body(), "pub mod MyMod;".to_owned());
    }

    #[test]
    fn test_submodules() {
        let mut test_mod = ModuleFragment::new("MyDom".to_owned());
        test_mod
            .add_submodule("MySub".to_owned())
            .borrow_mut()
            .mark_as_public();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {"
                mod MyDom {
                    pub mod MySub;
                }"}
        );
    }

    #[test]
    fn test_implemented_submodules() {
        let mut test_mod = ModuleFragment::new("MyDom".to_owned());
        test_mod
            .add_submodule("MySub".to_owned())
            .borrow_mut()
            .mark_for_full_implementation();

        assert_eq!(test_mod.imports(), Vec::<String>::new());
        assert_eq!(
            test_mod.body(),
            indoc! {"
                mod MyDom {
                    mod MySub {
                    }
                }"}
        );
    }

    #[test]
    fn test_file_with_tests() {
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
