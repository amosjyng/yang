use super::CodeFragment;

use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

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

    /// Update the separator between fragments.
    pub fn set_separator(&mut self, separator: &str) {
        self.block_separator = separator.to_owned();
    }
}

/// Marks this type of fragment as accepting additional appended fragments. If this is a nested
/// fragment, then the appended fragments may show up as nested within the body of the fragment,
pub trait Appendable: CodeFragment {
    /// Append other code fragment into this one.
    fn append(&mut self, other: Rc<RefCell<dyn CodeFragment>>);

    /// Prepend other code fragment at the beginning of this one.
    fn prepend(&mut self, other: Rc<RefCell<dyn CodeFragment>>);

    /// Returns true if no fragments have been appended into this one.
    fn is_empty(&self) -> bool;
}

impl Appendable for AppendedFragment {
    fn append(&mut self, other: Rc<RefCell<dyn CodeFragment>>) {
        self.appendages.push(other);
    }

    fn prepend(&mut self, other: Rc<RefCell<dyn CodeFragment>>) {
        self.appendages.insert(0, other);
    }

    fn is_empty(&self) -> bool {
        self.appendages.is_empty()
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
    fn body(&self, line_width: usize) -> String {
        (&self.appendages)
            .iter()
            .map(|cf| cf.borrow().body(line_width))
            .filter(|b| !b.is_empty())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::basic::AtomicFragment;
    use indoc::indoc;

    #[test]
    fn test_append_empty() {
        let appended = AppendedFragment::default();
        assert_eq!(appended.imports(), Vec::<String>::default());
        assert_eq!(appended.body(80), "");
    }

    #[test]
    fn test_append_one() {
        let mut appended = AppendedFragment::default();
        appended.append(Rc::new(RefCell::new(AtomicFragment::new("one".to_owned()))));
        assert_eq!(appended.imports(), Vec::<String>::default());
        assert_eq!(appended.body(80), "one");
    }

    #[test]
    fn test_append_one_actual() {
        let mut appended = AppendedFragment::default();
        appended.append(Rc::new(RefCell::new(AtomicFragment::new("one".to_owned()))));
        appended.append(Rc::new(RefCell::new(
            AtomicFragment::new(String::default()),
        )));
        assert_eq!(appended.imports(), Vec::<String>::default());
        assert_eq!(appended.body(80), "one");
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
            appended.body(80),
            indoc! {"
                let mut f = ForeignStruct {};
                f.foo_bar()
            "}
            .trim()
        );
    }

    #[test]
    fn test_prepend() {
        let line1 = AtomicFragment {
            imports: vec!["foreign_crate::sub::ForeignStruct".to_owned()],
            atom: "let mut f = ForeignStruct {};".to_owned(),
        };
        let line2 = AtomicFragment {
            imports: vec!["foreign_crate::FooBarTrait".to_owned()],
            atom: "f.foo_bar()".to_owned(),
        };
        let mut appended = AppendedFragment::new_with_separator("\n");
        appended.prepend(Rc::new(RefCell::new(line2)));
        appended.prepend(Rc::new(RefCell::new(line1)));
        assert_eq!(
            appended.imports(),
            vec![
                "foreign_crate::sub::ForeignStruct".to_owned(),
                "foreign_crate::FooBarTrait".to_owned()
            ]
        );
        assert_eq!(
            appended.body(80),
            indoc! {"
                let mut f = ForeignStruct {};
                f.foo_bar()
            "}
            .trim()
        );
    }
}
