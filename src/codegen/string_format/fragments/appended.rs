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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::string_format::fragments::AtomicFragment;
    use indoc::indoc;

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
    fn test_append_empty() {
        let appended = AppendedFragment::default();
        assert_eq!(appended.imports(), Vec::<String>::default());
        assert_eq!(appended.body(), "");
    }
}
