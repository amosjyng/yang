use super::{Appendable, AppendedFragment, AtomicFragment, CodeFragment, RUST_INDENTATION};
use crate::codegen::{add_indent, INDENT_SIZE};
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
#[derive(Default)]
pub struct NestedFragment {
    /// Declaration for this fragment (e.g. function signature, class signature, etc).
    pub preamble: AtomicFragment,
    /// Content that actually defines this fragment.
    pub nesting: Vec<Rc<RefCell<dyn CodeFragment>>>,
    /// In case the nesting cannot be inlined, this allows for adding extra characters to the end
    /// of the last line of the indented nesting.
    ///
    /// For example, this allows for commas to be added to the last item in a multiline-struct,
    /// which allows for cleaner single-line diffs when a new line gets added to the struct.
    pub nesting_postfix: Option<String>,
    /// Separator to use between body fragments.
    pub separator: String,
    /// Closing for this fragment during generation. Usually just a closing bracket.
    pub postamble: String,
}

impl NestedFragment {
    /// Create a new nested fragment with the given preamble and postamble.
    pub fn new(preamble: AtomicFragment, postamble: &str) -> Self {
        Self {
            preamble,
            postamble: postamble.to_owned(),
            ..Self::default()
        }
    }

    /// Set the body fragment separator.
    pub fn set_separator(&mut self, separator: &str) {
        self.separator = separator.to_owned();
    }

    /// Set the postfix for the indented nesting.
    pub fn set_nesting_postfix(&mut self, postfix: &str) {
        self.nesting_postfix = Some(postfix.to_owned());
    }
}

impl Appendable for NestedFragment {
    fn append(&mut self, nesting: Rc<RefCell<dyn CodeFragment>>) {
        self.nesting.push(nesting);
    }
}

impl CodeFragment for NestedFragment {
    fn body(&self, line_width: usize) -> String {
        let trimmed_preamble = self.preamble.body(line_width).trim().to_owned();
        let trimmed_postamble = self.postamble.trim();
        let mut body = AppendedFragment::new_with_separator(&self.separator);
        for nesting in &self.nesting {
            body.append(nesting.clone());
        }
        let inlined_body = body.body(line_width);
        if !inlined_body.contains('\n')
            && (!trimmed_preamble.contains('{') || inlined_body.is_empty())
            && trimmed_preamble.len() + inlined_body.len() + trimmed_postamble.len() <= line_width
        {
            trimmed_preamble + &inlined_body + trimmed_postamble
        } else {
            body.set_separator(&format!("{}\n", self.separator.trim_end()));
            let multilined_body = body.body(line_width - INDENT_SIZE);
            let mut result = trimmed_preamble + "\n";
            for line in multilined_body.split('\n') {
                result += &(add_indent(RUST_INDENTATION, line) + "\n");
            }
            if let Some(postfix) = &self.nesting_postfix {
                result.pop();
                result += postfix;
                result.push('\n');
            }
            result + trimmed_postamble
        }
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.preamble.imports();
        for n in &self.nesting {
            imports.append(&mut n.borrow().imports());
        }
        imports
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::{AppendedFragment, AtomicFragment};
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
        let mut nested = NestedFragment::new(
            AtomicFragment {
                imports: vec!["std::official::RustStruct".to_owned()],
                atom: "fn new_rust_struct() -> RustStruct {".to_owned(),
            },
            "}",
        );
        nested.append(Rc::new(RefCell::new(appended)));
        assert_eq!(
            nested.imports(),
            vec![
                "std::official::RustStruct".to_owned(),
                "foreign_crate::sub::ForeignStruct".to_owned(),
                "foreign_crate::FooBarTrait".to_owned()
            ]
        );
        assert_eq!(
            nested.body(80),
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
        let nested = NestedFragment::new(
            AtomicFragment {
                imports: vec!["std::official::RustStruct".to_owned()],
                atom: "RustStruct {".to_owned(),
            },
            "}",
        );
        assert_eq!(
            nested.imports(),
            vec!["std::official::RustStruct".to_owned(),]
        );
        assert_eq!(nested.body(80), "RustStruct {}");
    }

    #[test]
    fn test_nest_empty_contents() {
        let mut nested = NestedFragment::new(
            AtomicFragment {
                imports: vec!["std::official::RustStruct".to_owned()],
                atom: "RustStruct {".to_owned(),
            },
            "}",
        );
        nested.append(Rc::new(RefCell::new(AppendedFragment::default())));
        assert_eq!(
            nested.imports(),
            vec!["std::official::RustStruct".to_owned(),]
        );
        assert_eq!(nested.body(80), "RustStruct {}");
    }

    #[test]
    fn test_nest_short_contents() {
        let mut nested = NestedFragment::new(AtomicFragment::new("foo(".to_owned()), ");");
        nested.set_separator(", ");
        nested.set_nesting_postfix(",");
        nested.append(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        assert_eq!(nested.body(80), "foo(bar);");
    }

    #[test]
    fn test_nest_short_separated_contents() {
        let mut nested = NestedFragment::new(AtomicFragment::new("foo(".to_owned()), ");");
        nested.set_separator(", ");
        nested.set_nesting_postfix(",");
        nested.append(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        nested.append(Rc::new(RefCell::new(AtomicFragment::new("baz".to_owned()))));
        assert_eq!(nested.body(80), "foo(bar, baz);");
    }

    #[test]
    fn test_nest_newlined_contents() {
        let mut nested = NestedFragment::new(AtomicFragment::new("foo(".to_owned()), ");");
        nested.set_separator(", ");
        nested.set_nesting_postfix(",");
        nested.append(Rc::new(RefCell::new(AtomicFragment::new(
            indoc! {"
                bar(
                    1,
                    2,
                )"}
            .to_owned(),
        ))));
        assert_eq!(
            nested.body(80),
            indoc! {"
            foo(
                bar(
                    1,
                    2,
                ),
            );"}
        );
    }

    #[test]
    fn test_nest_multilined_separated_contents() {
        let mut nested = NestedFragment::new(AtomicFragment::new("foo(".to_owned()), ");");
        nested.set_separator(", ");
        nested.set_nesting_postfix(",");
        nested.append(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        nested.append(Rc::new(RefCell::new(AtomicFragment::new("baz".to_owned()))));
        assert_eq!(
            nested.body(8),
            indoc! {"
            foo(
                bar,
                baz,
            );"}
        );
    }
}
