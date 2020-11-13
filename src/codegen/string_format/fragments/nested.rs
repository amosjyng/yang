use super::{CodeFragment, RUST_INDENTATION};
use crate::codegen::add_indent;
use std::cell::RefCell;
use std::rc::Rc;

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
            let body = n.borrow().body();
            let trimmed_body = body.trim();
            if !trimmed_body.is_empty() {
                for line in trimmed_body.split('\n') {
                    result += &(add_indent(RUST_INDENTATION, line) + "\n");
                }
            }
        }
        result + self.postamble.trim()
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.imports.clone();
        if let Some(n) = self.nesting.as_ref() {
            imports.append(&mut n.borrow().imports());
        }
        imports
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::string_format::fragments::{AppendedFragment, AtomicFragment};
    use indoc::indoc;

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

    #[test]
    fn test_nest_empty_contents() {
        let nested = NestedFragment {
            imports: vec!["std::official::RustStruct".to_owned()],
            preamble: "RustStruct {".to_owned(),
            nesting: Some(Rc::new(RefCell::new(AppendedFragment::default()))),
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
