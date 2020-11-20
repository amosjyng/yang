use super::{AppendedFragment, AtomicFragment, CodeFragment, NestedFragment};
use crate::codegen::template::imports::imports_as_str;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for a module declaration.
pub struct ModuleFragment {
    name: String,
    test: bool,
    content: Rc<RefCell<AppendedFragment>>,
}

impl ModuleFragment {
    /// Create a new module with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            test: false,
            content: Rc::new(RefCell::new(AppendedFragment::default())),
        }
    }

    /// Create a new module with the name `tests`.
    pub fn new_test_module() -> Self {
        Self {
            name: "tests".to_owned(),
            test: true,
            content: Rc::new(RefCell::new(AppendedFragment::default())),
        }
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

        let cfg_test = if self.test { "#[cfg(test)]" } else { "" };
        let internal_code = format!("{}\n\n{}\n", imports_str, self.content.borrow().body());
        let internals = AtomicFragment {
            imports: Vec::new(),
            atom: internal_code,
        };
        let nested = NestedFragment {
            imports: Vec::new(),
            preamble: formatdoc! {"
                {cfg_test}
                mod {name} {{",
                name = self.name,
                cfg_test = cfg_test,
            },
            nesting: Some(Rc::new(RefCell::new(internals))),
            postamble: "}".to_owned(),
        };
        nested.body()
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
