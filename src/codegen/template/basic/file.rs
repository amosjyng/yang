use super::{AppendedFragment, AtomicFragment, CodeFragment, ModuleFragment};
use crate::codegen::template::imports::imports_as_str;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for an entire code file. Doesn't actually implement `CodeFragment` because this is
/// supposed to be top-level and not nested or appended to anything else.
#[derive(Default)]
pub struct FileFragment {
    preamble: Option<AtomicFragment>,
    contents: Rc<RefCell<AppendedFragment>>,
    tests: Vec<Rc<RefCell<dyn CodeFragment>>>,
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

    /// Add another fragment to this top-level file fragment.
    pub fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.contents.borrow_mut().append(fragment);
    }

    /// Add test code to the test module for this file.
    pub fn append_test(&mut self, test: Rc<RefCell<dyn CodeFragment>>) {
        self.tests.push(test);
    }

    /// Get the code for this fragment.
    pub fn generate_code(&self) -> String {
        let mut combined = AppendedFragment::default();
        combined.append(self.contents.clone());
        if !self.tests.is_empty() {
            let mut test_mod = ModuleFragment::new_test_module();
            for test in &self.tests {
                test_mod.append(test.clone());
            }
            combined.append(Rc::new(RefCell::new(test_mod)));
        }

        let imports = imports_as_str(
            &combined
                .imports()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>(),
        );

        let body = combined.body();

        let mut final_file = String::new();
        if let Some(preamble) = &self.preamble {
            final_file += &format!("{}\n\n", preamble.body());
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
    use indoc::indoc;

    #[test]
    fn test_empty_file() {
        assert_eq!(FileFragment::default().generate_code(), "");
    }

    #[test]
    fn test_file_with_preamble() {
        let mut file = FileFragment::default();
        file.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec![
                "std::OrTheOther".to_owned(),
                "crate::Something".to_owned(),
                "crate::Unused".to_owned(),
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
