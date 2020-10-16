use super::{AppendedFragment, CodeFragment};
use crate::codegen::string_format::sort_imports;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for an entire code file.
#[derive(Default)]
struct FileFragment {
    contents: Rc<RefCell<AppendedFragment>>,
    tests: Option<Rc<RefCell<dyn CodeFragment>>>,
}

impl FileFragment {
    /// Add another fragment to this top-level file fragment.
    pub fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.contents.borrow_mut().append(fragment);
    }

    pub fn set_tests(&mut self, tests: Rc<RefCell<dyn CodeFragment>>) {
        self.tests = Some(tests);
    }

    /// Retrieve imports as strings.
    fn imports_as_str(&self) -> String {
        // this doesn't need to take into account self.tests because tests don't contribute to file
        // imports
        let mut result = String::new();
        for import in &self.contents.borrow().imports() {
            result += &format!("use {};\n", import);
        }
        sort_imports(&result)
    }

    /// Get the code for this fragment.
    pub fn generate_code(&self) -> String {
        let mut combined = AppendedFragment::default();
        combined.append(self.contents.clone());
        self.tests.as_ref().map(|t| combined.append(t.clone()));

        format!(
            "{}\n\n{}\n",
            self.imports_as_str(),
            self.contents.borrow().body()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::AtomicFragment;
    use super::*;
    use indoc::indoc;

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
        file.append(Rc::new(RefCell::new(AtomicFragment {
            imports: Vec::new(),
            atom: indoc! {"
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
            .trim()
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
