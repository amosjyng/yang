use super::{AtomicFragment, CodeFragment, NestedFragment};
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for a vector
#[derive(Default)]
pub struct VecFragment {
    /// Elements of the vector.
    pub elements: Vec<Rc<RefCell<dyn CodeFragment>>>,
}

impl VecFragment {
    /// Get a default new vector.
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    /// Add a new argument to the function call.
    pub fn add_element(&mut self, element: Rc<RefCell<dyn CodeFragment>>) {
        self.elements.push(element);
    }

    /// Add a new argument as a string without imports.
    pub fn add_element_str(&mut self, element: &str) {
        self.add_element(Rc::new(RefCell::new(AtomicFragment::new(
            element.to_owned(),
        ))));
    }
}

impl CodeFragment for VecFragment {
    fn body(&self, line_width: usize) -> String {
        let mut nested = NestedFragment::new(AtomicFragment::new("vec![".to_owned()), "]");
        nested.set_separator(", ");
        for arg in &self.elements {
            nested.append(arg.clone());
        }
        nested.body(line_width)
    }

    fn imports(&self) -> Vec<String> {
        let mut elements = Vec::<String>::new();
        for element in &self.elements {
            elements.append(&mut element.borrow().imports());
        }
        elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_empty_vec() {
        let f = VecFragment::new();
        assert_eq!(f.body(80), "vec![]");
    }

    #[test]
    fn test_vec_elements() {
        let mut f = VecFragment::new();
        f.add_element(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        f.add_element(Rc::new(RefCell::new(AtomicFragment::new("baz".to_owned()))));
        assert_eq!(f.body(80), "vec![bar, baz]");
    }

    #[test]
    fn test_vec_elements_multiline() {
        let mut f = VecFragment::new();
        f.add_element(Rc::new(RefCell::new(AtomicFragment::new("bar".to_owned()))));
        f.add_element(Rc::new(RefCell::new(AtomicFragment::new("baz".to_owned()))));
        assert_eq!(
            f.body(8),
            indoc! {"
            vec![
                bar,
                baz
            ]"}
        );
    }
}
