use super::{AtomicFragment, CodeFragment, NestedFragment};
use crate::codegen::StructConfig;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for a function call statement
#[derive(Default)]
pub struct FunctionCallFragment {
    /// Function to actually call
    pub call: StructConfig,
    /// Arguments for function call. Arguments can be statements themselves.
    pub arguments: Vec<Rc<RefCell<dyn CodeFragment>>>,
}

impl FunctionCallFragment {
    /// Get a default new function call.
    pub fn new(name: StructConfig) -> Self {
        Self {
            call: name,
            ..Self::default()
        }
    }

    /// Add a new argument to the function call.
    pub fn add_argument(&mut self, argument: Rc<RefCell<dyn CodeFragment>>) {
        self.arguments.push(argument);
    }
}

impl CodeFragment for FunctionCallFragment {
    fn body(&self, line_width: usize) -> String {
        let mut nested =
            NestedFragment::new(AtomicFragment::new(format!("{}(", self.call.name)), ");");
        nested.set_separator(", ");
        nested.set_nesting_postfix(",");
        for arg in &self.arguments {
            nested.append(arg.clone());
        }
        nested.body(line_width)
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = vec![self.call.import.clone()];
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
        let f = FunctionCallFragment::new(StructConfig::new("foo".to_owned()));
        assert_eq!(f.body(80), "foo();");
    }

    #[test]
    fn test_call_arguments() {
        let mut f = FunctionCallFragment::new(StructConfig::new("foo".to_owned()));
        f.add_argument(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        f.add_argument(Rc::new(RefCell::new(AtomicFragment::new("baz".to_owned()))));
        assert_eq!(f.body(80), "foo(bar, baz);");
    }

    #[test]
    fn test_call_arguments_multiline() {
        let mut f = FunctionCallFragment::new(StructConfig::new("foo".to_owned()));
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
}
