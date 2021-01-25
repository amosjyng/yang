use super::{Appendable, AtomicFragment, CodeFragment, NestedFragment};
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for a function call statement
#[derive(Default)]
pub struct FunctionCallFragment {
    /// Function to actually call
    pub call: AtomicFragment,
    /// Arguments for function call. Arguments can be statements themselves.
    pub arguments: Vec<Rc<RefCell<dyn CodeFragment>>>,
    /// Technically a macro call is different from a function call, but they're syntactically
    /// similar enough that we're only differentiating them with a single bool.
    pub is_macro: bool,
}

impl FunctionCallFragment {
    /// Get a default new function call.
    pub fn new(name: AtomicFragment) -> Self {
        Self {
            call: name,
            ..Self::default()
        }
    }

    /// Add a new argument to the function call.
    pub fn add_argument(&mut self, argument: Rc<RefCell<dyn CodeFragment>>) {
        self.arguments.push(argument);
    }

    /// Add a new argument as a string without imports.
    pub fn add_argument_str(&mut self, argument: &str) {
        self.add_argument(Rc::new(RefCell::new(AtomicFragment::new(
            argument.to_owned(),
        ))));
    }

    /// Mark this as a macro call instead of a function call.
    pub fn mark_macro(&mut self) {
        self.is_macro = true;
    }
}

impl CodeFragment for FunctionCallFragment {
    fn body(&self, line_width: usize) -> String {
        let preamble = if self.is_macro {
            format!("{}!(", self.call.atom)
        } else {
            format!("{}(", self.call.atom)
        };
        let mut nested = NestedFragment::new(AtomicFragment::new(preamble), ");");
        nested.set_separator(", ");
        if !self.is_macro {
            nested.set_nesting_postfix(",");
        }
        for arg in &self.arguments {
            nested.append(arg.clone());
        }
        nested.body(line_width)
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.call.imports();
        for arg in &self.arguments {
            imports.append(&mut arg.borrow().imports());
        }
        imports
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_empty_call() {
        let f = FunctionCallFragment::new(AtomicFragment::new("foo".to_owned()));
        assert_eq!(f.body(80), "foo();");
    }

    #[test]
    fn test_call_arguments() {
        let mut f = FunctionCallFragment::new(AtomicFragment::new("foo".to_owned()));
        f.add_argument(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        f.add_argument(Rc::new(RefCell::new(AtomicFragment::new("baz".to_owned()))));
        assert_eq!(f.body(80), "foo(bar, baz);");
    }

    #[test]
    fn test_call_arguments_multiline() {
        let mut f = FunctionCallFragment::new(AtomicFragment::new("foo".to_owned()));
        f.add_argument(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        f.add_argument(Rc::new(RefCell::new(AtomicFragment::new("baz".to_owned()))));
        assert_eq!(
            f.body(8),
            indoc! {"
            foo(
                bar,
                baz,
            );"}
        );
    }

    #[test]
    fn test_call_macro_multiline() {
        let mut f = FunctionCallFragment::new(AtomicFragment::new("assert_eq".to_owned()));
        f.mark_macro();
        f.add_argument(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        f.add_argument(Rc::new(RefCell::new(AtomicFragment::new("baz".to_owned()))));
        assert_eq!(
            f.body(8),
            indoc! {"
            assert_eq!(
                bar,
                baz
            );"}
        );
    }
}
