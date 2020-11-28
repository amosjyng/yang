use super::{AtomicFragment, CodeFragment, FunctionCallFragment};
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for an assertion macro call.
pub struct AssertFragment {
    /// Left-hand side of the assert statement.
    pub lhs: Rc<RefCell<dyn CodeFragment>>,
    /// Right-hand side of the assert statement.
    pub rhs: Rc<RefCell<dyn CodeFragment>>,
    /// The function call that this represents.
    f: FunctionCallFragment,
}

impl AssertFragment {
    /// Create a new assert_eq! macro invocation.
    pub fn new_eq(lhs: Rc<RefCell<dyn CodeFragment>>, rhs: Rc<RefCell<dyn CodeFragment>>) -> Self {
        let mut f = FunctionCallFragment::new(AtomicFragment::new("assert_eq".to_owned()));
        f.mark_macro();
        f.add_argument(lhs.clone());
        f.add_argument(rhs.clone());
        AssertFragment { lhs, rhs, f }
    }
}

impl CodeFragment for AssertFragment {
    fn body(&self, line_width: usize) -> String {
        self.f.body(line_width)
    }

    fn imports(&self) -> Vec<String> {
        self.f.imports()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_eq() {
        assert_eq!(
            AssertFragment::new_eq(
                Rc::new(RefCell::new(AtomicFragment::new("x".to_owned()))),
                Rc::new(RefCell::new(AtomicFragment::new("y".to_owned())))
            )
            .body(80),
            "assert_eq!(x, y);"
        );
    }
}
