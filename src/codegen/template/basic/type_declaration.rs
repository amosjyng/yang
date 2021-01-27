use super::{AtomicFragment, CodeFragment, ItemDeclaration, ItemDeclarationAPI, TypeFragment};
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for generating a Rust type declaration -- whether as an associated type, or a type
/// alias.
#[derive(Default)]
pub struct TypeDeclaration {
    ///  Type being declared. Includes type bounds.
    declared_type: TypeFragment,
    /// Declaration fragment for this `Trait`.
    declaration: ItemDeclaration,
}

impl TypeDeclaration {
    /// Create a new type declaration with the given name.
    pub fn new(name: String) -> Self {
        Self {
            declared_type: TypeFragment::new(name),
            ..Self::default()
        }
    }

    /// Add a trait requirement
    pub fn add_required_trait(&mut self, required_trait: Box<dyn CodeFragment>) {
        self.declared_type.add_required_trait(required_trait);
    }
}

impl ItemDeclarationAPI for TypeDeclaration {
    fn mark_as_public(&mut self) {
        self.declaration.mark_as_public();
    }

    fn is_public(&self) -> bool {
        self.declaration.is_public()
    }

    fn add_attribute(&mut self, attribute: String) {
        self.declaration.add_attribute(attribute);
    }

    fn document(&mut self, documentation: String) {
        self.declaration.document(documentation);
    }

    fn set_body(&mut self, body: Rc<RefCell<dyn CodeFragment>>) {
        self.declaration.set_body(body);
    }

    fn mark_as_declare_only(&mut self) {
        self.declaration.mark_as_declare_only();
    }

    fn mark_for_full_implementation(&mut self) {
        self.declaration.mark_for_full_implementation();
    }
}

impl CodeFragment for TypeDeclaration {
    fn body(&self, line_width: usize) -> String {
        let mut declaration = self.declaration.clone();
        declaration.set_definition(Rc::new(RefCell::new(AtomicFragment::new(format!(
            "type {}",
            self.declared_type.body(line_width) // todo: subtract line width
        )))));
        declaration.body(line_width) // declaration will take care of indent size
    }

    fn imports(&self) -> Vec<String> {
        self.declared_type.imports()
    }
}

#[cfg(test)]
mod tests {
    use super::super::AtomicFragment;
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_type() {
        let f = TypeDeclaration::new("Foo".to_owned());

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(f.body(80), "type Foo;");
    }

    #[test]
    fn test_documented_type() {
        let mut f = TypeDeclaration::new("Foo".to_owned());
        f.document("Is gooey.".to_owned());

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(80),
            indoc! {"
            /// Is gooey.
            type Foo;"}
        );
    }

    #[test]
    fn test_type_with_requirements() {
        let mut f = TypeDeclaration::new("Foo".to_owned());
        f.document("Is gooey.".to_owned());
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
        assert_eq!(
            f.body(80),
            indoc! {"
            /// Is gooey.
            type Foo: Bar + Baz;"}
        );
    }
}
