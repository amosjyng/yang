use crate::codegen::string_format::sort_imports;
use super::{AppendedFragment, CodeFragment};

/// Fragment for an entire code file.
#[derive(Default)]
struct FileFragment {
    contents: AppendedFragment,
}

impl FileFragment {
    /// Add another fragment to this top-level file fragment.
    pub fn append(&mut self, fragment: Box<dyn CodeFragment>) {
        self.contents.append(fragment);
    }

    /// Retrieve imports as strings.
    fn imports_as_str(&self) -> String {
        let mut result = String::new();
        for import in &self.contents.imports() {
            result += &format!("use {};\n", import);
        }
        sort_imports(&result)
    }

    /// Get the code for this fragment.
    pub fn generate_code(&self) -> String {
        format!("{}\n\n{}\n", self.imports_as_str(), self.contents.body())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::AtomicFragment;
    use indoc::indoc;

    #[test]
    fn test_file_with_tests() {
        let mut file = FileFragment::default();
        file.append(Box::new(AtomicFragment {
            imports: vec!["std::OrTheOther".to_owned(), "crate::Something".to_owned()],
            atom: indoc!{"
                pub struct Big {
                    a: Something,
                    b: OrTheOther,
                }
            "}.trim().to_string(),
        }));
        file.append(Box::new(AtomicFragment {
            imports: Vec::new(),
            atom: indoc!{"
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
            "}.trim().to_string(),
        }));

        assert_eq!(file.generate_code(), indoc!{"
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
        "});
    }
}
