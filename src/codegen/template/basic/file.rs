use super::{Appendable, AppendedFragment, AtomicFragment, CodeFragment, ModuleFragment};
use crate::codegen::template::imports::imports_as_str;
use crate::codegen::CODE_WIDTH;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for an entire code file. Doesn't actually implement `CodeFragment` because this is
/// supposed to be top-level and not nested or appended to anything else.
#[derive(Default)]
pub struct FileFragment {
    preamble: Option<AtomicFragment>,
    contents: Rc<RefCell<AppendedFragment>>,
    tests: Vec<Rc<RefCell<dyn CodeFragment>>>,
    self_import: Option<String>,
    current_crate: Option<Rc<str>>,
}

impl FileFragment {
    /// Get a new default FileFragment to use.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a preamble for this file. Preambles are like hashbangs or module tags.
    pub fn set_preamble(&mut self, preamble: AtomicFragment) {
        self.preamble = Some(preamble)
    }

    /// Add test code to the test module for this file.
    pub fn append_test(&mut self, test: Rc<RefCell<dyn CodeFragment>>) {
        self.tests.push(test);
    }

    /// If there's something defined within this file, set the import path here, and it will be
    /// excluded from the file's final set of imports.
    pub fn set_self_import(&mut self, import: String) {
        self.self_import = Some(import);
    }

    /// Set the current crate for file imports.
    pub fn set_current_crate(&mut self, current_crate: Rc<str>) {
        self.current_crate = Some(current_crate);
    }

    /// Get the code for this fragment.
    pub fn generate_code(&self) -> String {
        self.body(CODE_WIDTH)
    }
}

impl Appendable for FileFragment {
    fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.contents.borrow_mut().append(fragment);
    }

    fn prepend(&mut self, other: Rc<RefCell<dyn CodeFragment>>) {
        self.contents.borrow_mut().prepend(other);
    }

    fn is_empty(&self) -> bool {
        self.contents.borrow().is_empty()
    }
}

impl CodeFragment for FileFragment {
    fn imports(&self) -> Vec<String> {
        Vec::new()
    }

    fn body(&self, line_width: usize) -> String {
        let mut combined = AppendedFragment::default();
        combined.append(self.contents.clone());
        if !self.tests.is_empty() {
            let mut test_mod = ModuleFragment::new_test_module();
            if let Some(current_crate) = &self.current_crate {
                test_mod.set_current_crate(current_crate.clone());
            }
            for test in &self.tests {
                test_mod.append(test.clone());
            }
            combined.append(Rc::new(RefCell::new(test_mod)));
        }

        let mut exluded_imports = vec![];
        if let Some(excluded_import) = &self.self_import {
            exluded_imports.push(excluded_import.as_str());
        }
        let imports = imports_as_str(
            &*self
                .current_crate
                .as_ref()
                .cloned()
                .unwrap_or_else(|| Rc::from("DUMMY-TEST-CRATE")),
            combined.imports(),
            &exluded_imports,
        );

        let body = combined.body(line_width);

        let mut final_file = String::new();
        if let Some(preamble) = &self.preamble {
            final_file += &format!("{}\n\n", preamble.body(line_width));
        }
        if !imports.is_empty() {
            final_file += &format!("{}\n\n", imports);
        }
        if !body.is_empty() {
            final_file += &format!("{}\n", body);
        }

        final_file
    }
}

#[cfg(test)]
mod tests {
    use super::super::AtomicFragment;
    use super::*;
    use crate::tao::initialize_kb;
    use indoc::indoc;

    #[test]
    fn test_empty_file() {
        assert_eq!(FileFragment::default().generate_code(), "");
    }

    #[test]
    fn test_file_with_preamble() {
        initialize_kb();

        let mut file = FileFragment::default();
        file.set_current_crate(Rc::from("my_crate"));
        file.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec![
                "std::OrTheOther".to_owned(),
                "my_crate::Something".to_owned(),
                "my_crate::Unused".to_owned(),
            ],
            atom: indoc! {"
                pub struct Big {
                    a: Something,
                    b: OrTheOther,
                }
            "}
            .trim()
            .to_string(),
        })));
        file.set_preamble(AtomicFragment {
            imports: vec![],
            atom: "#![allow(dead_code)]".to_owned(),
        });

        assert_eq!(
            file.generate_code(),
            indoc! {"
                #![allow(dead_code)]

                use crate::{Something, Unused};
                use std::OrTheOther;

                pub struct Big {
                    a: Something,
                    b: OrTheOther,
                }
        "}
        );
    }

    #[test]
    fn test_file_with_tests() {
        let mut file = FileFragment::default();
        file.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec!["std::OrTheOther".to_owned(), "crate::Something".to_owned()],
            atom: indoc! {"
                pub struct Big {
                    a: Something,
                    b: OrTheOther,
                }
            "}
            .trim()
            .to_string(),
        })));
        file.append_test(Rc::new(RefCell::new(AtomicFragment {
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

        assert_eq!(
            file.generate_code(),
            indoc! {"
                use crate::Something;
                use std::OrTheOther;

                pub struct Big {
                    a: Something,
                    b: OrTheOther,
                }

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
                }
        "}
        );
    }
}
