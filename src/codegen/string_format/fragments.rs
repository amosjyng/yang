use crate::codegen::add_indent;
use crate::codegen::string_format::sort_imports;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for an entire code file.
mod file;
/// Fragment for a module declaration.
mod module;

pub use file::FileFragment;
pub use module::ModuleFragment;

/// Number of spaces Rust is usually indented by.
const RUST_INDENTATION: usize = 4;

/// Serialize imports into a string.
pub fn imports_as_str(imports: &[String]) -> String {
    // this doesn't need to take into account self.tests because tests don't contribute to file
    // imports
    let mut result = String::new();
    for import in imports {
        result += &format!("use {};\n", import);
    }
    sort_imports(&result).trim().to_owned()
}

/// Represents a fragment of code that can be appended to or nested with other code fragements.
pub trait CodeFragment {
    /// Retrieve all imports used by this fragment.
    fn imports(&self) -> Vec<String>;
    /// Retrieve main body of code in this fragment.
    fn body(&self) -> String;
}

/// Code fragment that cannot be broken down any further.
pub struct AtomicFragment {
    /// Imports for the fragment.
    pub imports: Vec<String>,
    /// Body of the fragment.
    pub atom: String,
}

impl CodeFragment for AtomicFragment {
    fn body(&self) -> String {
        self.atom.trim().to_string()
    }

    fn imports(&self) -> Vec<String> {
        self.imports.clone()
    }
}

/// Code fragment that can be combined with other code fragments.
///
/// Think: function bodies (you can always append extra lines), class bodies (you can always append
/// extra functions), etc.
pub struct AppendedFragment {
    /// Component fragments that make up this appended fragment.
    pub appendages: Vec<Rc<RefCell<dyn CodeFragment>>>,
    /// Separator between fragments when generating the body for this fragment.
    pub block_separator: String,
}

impl AppendedFragment {
    /// Create a new fragment with a custom separator between components.
    pub fn new_with_separator(block_separator: &str) -> Self {
        Self {
            appendages: Vec::new(),
            block_separator: block_separator.to_owned(),
        }
    }

    /// Append other code fragment into this one.
    pub fn append(&mut self, other: Rc<RefCell<dyn CodeFragment>>) {
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
            .iter()
            .map(|cf| cf.borrow().body())
            .format(&self.block_separator)
            .to_string()
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = Vec::new();
        for appendage in &self.appendages {
            imports.append(&mut appendage.borrow().imports());
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
pub struct NestedFragment {
    /// Imports for this fragment.
    pub imports: Vec<String>,
    /// Declaration for this fragment (e.g. function signature, class signature, etc).
    pub preamble: String,
    /// Content that actually defines this fragment.
    pub nesting: Option<Rc<RefCell<dyn CodeFragment>>>,
    /// Closing for this fragment during generation. Usually just a closing bracket.
    pub postamble: String,
}

impl NestedFragment {
    /// Nest another fragment inside of this one.
    pub fn set_nesting(&mut self, nesting: Rc<RefCell<dyn CodeFragment>>) {
        self.nesting = Some(nesting);
    }
}

impl CodeFragment for NestedFragment {
    fn body(&self) -> String {
        let mut result = self.preamble.trim().to_owned() + "\n";
        if let Some(n) = self.nesting.as_ref() {
            for line in n.borrow().body().trim().split('\n') {
                result += &(add_indent(RUST_INDENTATION, line) + "\n");
            }
        }
        result + self.postamble.trim()
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.imports.clone();
        if let Some(n) = self.nesting .as_ref() {
            imports.append(&mut n.borrow().imports());
        }
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
        appended.append(Rc::new(RefCell::new(line1)));
        appended.append(Rc::new(RefCell::new(line2)));
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
        appended.append(Rc::new(RefCell::new(line1)));
        appended.append(Rc::new(RefCell::new(line2)));
        let mut nested = NestedFragment {
            imports: vec!["std::official::RustStruct".to_owned()],
            preamble: "fn new_rust_struct() -> RustStruct {".to_owned(),
            nesting: None,
            postamble: "}".to_owned(),
        };
        nested.set_nesting(Rc::new(RefCell::new(appended)));
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
