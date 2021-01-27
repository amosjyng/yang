use super::CodeFragment;
use itertools::Itertools;

/// Fragment for a type specification.
#[derive(Default)]
pub struct TypeFragment {
    /// Name of the type.
    name: String,
    /// Any traits that are required to be implemented for this type.
    required_traits: Vec<Box<dyn CodeFragment>>,
}

impl TypeFragment {
    /// Create a new type with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }

    /// Add a trait requirement
    pub fn add_required_trait(&mut self, required_trait: Box<dyn CodeFragment>) {
        self.required_traits.push(required_trait);
    }
}

impl CodeFragment for TypeFragment {
    fn body(&self, line_width: usize) -> String {
        // todo: subtract appropriate line width
        let required_traits = self
            .required_traits
            .iter()
            .map(|r| r.body(line_width))
            .format(" + ");
        if self.required_traits.is_empty() {
            self.name.clone()
        } else {
            format!("{}: {}", self.name, required_traits)
        }
    }

    fn imports(&self) -> Vec<String> {
        self.required_traits
            .iter()
            .flat_map(|r| r.imports())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::super::AtomicFragment;
    use super::*;

    #[test]
    fn test_type() {
        let f = TypeFragment::new("Foo".to_owned());

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(f.body(80), "Foo");
    }

    #[test]
    fn test_type_with_requirements() {
        let mut f = TypeFragment::new("Foo".to_owned());
        f.add_required_trait(Box::new(AtomicFragment {
            atom: "Bar".to_owned(),
            imports: vec!["crate::Bar".to_owned()],
        }));
        f.add_required_trait(Box::new(AtomicFragment {
            atom: "Baz".to_owned(),
            imports: vec!["crate::Baz".to_owned()],
        }));

        assert_eq!(
            f.imports(),
            vec!["crate::Bar".to_owned(), "crate::Baz".to_owned()]
        );
        assert_eq!(f.body(80), "Foo: Bar + Baz");
    }
}
