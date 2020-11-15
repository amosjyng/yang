use super::{AppendedFragment, AtomicFragment, CodeFragment};
use crate::codegen::string_format::imports::imports_as_str;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for an entire code file. Doesn't actually implement `CodeFragment` because this is
/// supposed to be top-level and not nested or appended to anything else.
#[derive(Default)]
pub struct FileFragment {
    preamble: Option<AtomicFragment>,
    contents: Rc<RefCell<AppendedFragment>>,
    tests: Option<Rc<RefCell<dyn CodeFragment>>>,
}

impl FileFragment {
    /// Set a preamble for this file. Preambles are like hashbangs or module tags.
    pub fn set_preamble(&mut self, preamble: AtomicFragment) {
        self.preamble = Some(preamble)
    }

    /// Add another fragment to this top-level file fragment.
    pub fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.contents.borrow_mut().append(fragment);
    }

    /// Set the test module for this file.
    pub fn set_tests(&mut self, tests: Rc<RefCell<dyn CodeFragment>>) {
        self.tests = Some(tests);
    }

    /// Get the code for this fragment.
    pub fn generate_code(&self) -> String {
        let mut combined = AppendedFragment::default();
        combined.append(self.contents.clone());
        if let Some(t) = self.tests.as_ref() {
            combined.append(t.clone());
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
    use super::super::{AtomicFragment, ModuleFragment};
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
        file.set_tests(Rc::new(RefCell::new(test_mod)));

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
