use crate::codegen::add_indent;
use itertools::Itertools;

/// Fragment for an entire code file.
mod file;

/// Number of spaces Rust is usually indented by.
const RUST_INDENTATION: usize = 4;

/// Represents a fragment of code that can be appended to or nested with other code fragements.
trait CodeFragment {
    /// Retrieve all imports used by this fragment.
    fn imports(&self) -> Vec<String>;
    /// Retrieve main body of code in this fragment.
    fn body(&self) -> String;
}

/// Code fragment that cannot be broken down any further.
struct AtomicFragment {
    imports: Vec<String>,
    atom: String,
}

impl CodeFragment for AtomicFragment {
    fn body(&self) -> String {
        self.atom.clone()
    }

    fn imports(&self) -> Vec<String> {
        self.imports.clone()
    }
}

/// Code fragment that can be combined with other code fragments.
///
/// Think: function bodies (you can always append extra lines), class bodies (you can always append
/// extra functions), etc.
struct AppendedFragment {
    appendages: Vec<Box<dyn CodeFragment>>,
    block_separator: String,
}

impl AppendedFragment {
    pub fn new_with_separator(block_separator: &str) -> Self {
        Self {
            appendages: Vec::new(),
            block_separator: block_separator.to_owned(),
        }
    }

    /// Append other code fragment into this one.
    pub fn append(&mut self, other: Box<dyn CodeFragment>) {
        self.appendages.push(other);
    }
}

impl Default for AppendedFragment {
    fn default() -> Self {
        Self {
            appendages: Vec::default(),
            block_separator: "\n\n".to_owned(),
        }
    }
}

impl CodeFragment for AppendedFragment {
    fn body(&self) -> String {
        (&self.appendages)
            .into_iter()
            .map(|cf| cf.body())
            .format(&self.block_separator)
            .to_string()
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = Vec::new();
        for appendage in &self.appendages {
            imports.append(&mut appendage.imports());
        }
        imports
    }
}

/// Code fragment that nests another code fragment inside. Internal fragment will be given an extra
/// level of indentation.
///
/// Think: functions (nested function body inside), classes (nested function implementations
/// inside).
///
/// Preamble (e.g. function return value) can introduce new imports, so that's why this has its own
/// imports.
struct NestedFragment {
    imports: Vec<String>,
    preamble: String,
    nesting: Option<Box<dyn CodeFragment>>,
    postamble: String,
}

impl NestedFragment {
    pub fn set_nesting(&mut self, nesting: Box<dyn CodeFragment>) {
        self.nesting = Some(nesting);
    }
}

impl CodeFragment for NestedFragment {
    fn body(&self) -> String {
        let mut result = self.preamble.clone() + "\n";
        self.nesting.as_ref().map(|n| {
            for line in n.body().split('\n') {
                result += &(add_indent(RUST_INDENTATION, line) + "\n");
            }
        });
        result + &self.postamble
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.imports.clone();
        self.nesting
            .as_ref()
            .map(|n| imports.append(&mut n.imports()));
        imports
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_atom() {
        let line = AtomicFragment {
            imports: vec!["foreign_crate::sub::ForeignStruct".to_owned()],
            atom: "let mut f = ForeignStruct {};".to_owned(),
        };
        assert_eq!(
            line.imports(),
            vec!["foreign_crate::sub::ForeignStruct".to_owned()]
        );
        assert_eq!(line.body(), "let mut f = ForeignStruct {};".to_owned());
    }

    #[test]
    fn test_append() {
        let line1 = AtomicFragment {
            imports: vec!["foreign_crate::sub::ForeignStruct".to_owned()],
            atom: "let mut f = ForeignStruct {};".to_owned(),
        };
        let line2 = AtomicFragment {
            imports: vec!["foreign_crate::FooBarTrait".to_owned()],
            atom: "f.foo_bar()".to_owned(),
        };
        let mut appended = AppendedFragment::new_with_separator("\n");
        appended.append(Box::new(line1));
        appended.append(Box::new(line2));
        assert_eq!(
            appended.imports(),
            vec![
                "foreign_crate::sub::ForeignStruct".to_owned(),
                "foreign_crate::FooBarTrait".to_owned()
            ]
        );
        assert_eq!(
            appended.body(),
            indoc! {"
                let mut f = ForeignStruct {};
                f.foo_bar()
            "}
            .trim()
        );
    }

    #[test]
    fn test_nest() {
        let line1 = AtomicFragment {
            imports: vec!["foreign_crate::sub::ForeignStruct".to_owned()],
            atom: "let mut f = ForeignStruct {};".to_owned(),
        };
        let line2 = AtomicFragment {
            imports: vec!["foreign_crate::FooBarTrait".to_owned()],
            atom: "f.foo_bar()".to_owned(),
        };
        let mut appended = AppendedFragment::new_with_separator("\n");
        appended.append(Box::new(line1));
        appended.append(Box::new(line2));
        let mut nested = NestedFragment {
            imports: vec!["std::official::RustStruct".to_owned()],
            preamble: "fn new_rust_struct() -> RustStruct {".to_owned(),
            nesting: None,
            postamble: "}".to_owned(),
        };
        nested.set_nesting(Box::new(appended));
        assert_eq!(
            nested.imports(),
            vec![
                "std::official::RustStruct".to_owned(),
                "foreign_crate::sub::ForeignStruct".to_owned(),
                "foreign_crate::FooBarTrait".to_owned()
            ]
        );
        assert_eq!(
            nested.body(),
            indoc! {"
                fn new_rust_struct() -> RustStruct {
                    let mut f = ForeignStruct {};
                    f.foo_bar()
                }
            "}
            .trim()
        );
    }

    #[test]
    fn test_empty_nest() {
        let nested = NestedFragment {
            imports: vec!["std::official::RustStruct".to_owned()],
            preamble: "RustStruct {".to_owned(),
            nesting: None,
            postamble: "}".to_owned(),
        };
        assert_eq!(
            nested.imports(),
            vec!["std::official::RustStruct".to_owned(),]
        );
        assert_eq!(
            nested.body(),
            indoc! {"
                RustStruct {
                }
            "}
            .trim()
        );
    }
}
