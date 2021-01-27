use super::{
    Appendable, AppendedFragment, AtomicFragment, CodeFragment, ItemDeclaration,
    ItemDeclarationAPI, TypeDeclaration, TypeFragment,
};
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for generating a Rust `Trait`.
pub struct TraitFragment {
    ///  New type that this `Trait` defines.
    trait_type: TypeFragment,
    /// Declaration fragment for this `Trait`.
    declaration: ItemDeclaration,
    /// Actual internal code fragments for the `Trait`.
    content: Rc<RefCell<AppendedFragment>>,
}

impl TraitFragment {
    /// Create a new trait with the given name.
    pub fn new(name: String) -> Self {
        Self {
            trait_type: TypeFragment::new(name),
            ..Self::default()
        }
    }

    /// Add a trait requirement
    pub fn add_required_trait(&mut self, required_trait: Box<dyn CodeFragment>) {
        self.trait_type.add_required_trait(required_trait);
    }

    /// Add an associated type to this `Trait`.
    pub fn add_associated_type(&mut self, associated_type: TypeDeclaration) {
        self.content
            .borrow_mut()
            .prepend(Rc::new(RefCell::new(associated_type)));
    }
}

impl Appendable for TraitFragment {
    fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.content.borrow_mut().append(fragment);
    }

    fn prepend(&mut self, other: Rc<RefCell<dyn CodeFragment>>) {
        self.content.borrow_mut().prepend(other);
    }

    fn is_empty(&self) -> bool {
        self.content.borrow().is_empty()
    }
}

impl Default for TraitFragment {
    fn default() -> Self {
        let mut declaration = ItemDeclaration::default();
        let content = Rc::new(RefCell::new(AppendedFragment::default()));
        declaration.set_body(content.clone());
        Self {
            trait_type: TypeFragment::default(),
            declaration,
            content,
        }
    }
}

impl ItemDeclarationAPI for TraitFragment {
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

impl CodeFragment for TraitFragment {
    fn body(&self, line_width: usize) -> String {
        let mut declaration = self.declaration.clone();
        declaration.mark_for_full_implementation();
        declaration.set_definition(Rc::new(RefCell::new(AtomicFragment::new(format!(
            "trait {}",
            self.trait_type.body(line_width) // todo: subtract line width
        )))));
        declaration.body(line_width) // declaration will take care of indent size
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.trait_type.imports();
        imports.append(&mut self.content.borrow().imports());
        imports
    }
}

#[cfg(test)]
mod tests {
    use super::super::AtomicFragment;
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_empty_trait() {
        let f = TraitFragment::new("Foo".to_owned());

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(f.body(80), "trait Foo {}");
    }

    #[test]
    fn test_documented_trait() {
        let mut f = TraitFragment::new("Foo".to_owned());
        f.document("This is a trait.".to_owned());

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(
            f.body(80),
            indoc! {"
                /// This is a trait.
                trait Foo {}"}
        );
    }

    #[test]
    fn test_public_trait() {
        let mut f = TraitFragment::new("Foo".to_owned());
        f.mark_as_public();

        assert_eq!(f.imports(), Vec::<String>::new());
        assert_eq!(f.body(80), "pub trait Foo {}");
    }

    #[test]
    fn test_trait_requirements() {
        let mut f = TraitFragment::new("Foo".to_owned());
        f.add_required_trait(Box::new(AtomicFragment {
            atom: "Bar".to_owned(),
            imports: vec!["crate::Bar".to_owned()],
        }));
        f.add_required_trait(Box::new(AtomicFragment {
            atom: "Baz".to_owned(),
            imports: vec!["crate::Baz".to_owned()],
        }));
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec![],
            atom: indoc! {"
                fn do_something(&mut self) {
                    self.bar_capability();
                    self.baz_capability();
                }"}
            .to_string(),
        })));

        assert_eq!(
            f.body(80),
            indoc! {"
                trait Foo: Bar + Baz {
                    fn do_something(&mut self) {
                        self.bar_capability();
                        self.baz_capability();
                    }
                }"}
        );
    }

    #[test]
    fn test_associated_types() {
        let mut f = TraitFragment::new("Foo".to_owned());
        f.add_required_trait(Box::new(AtomicFragment {
            atom: "Bar".to_owned(),
            imports: vec!["crate::Bar".to_owned()],
        }));
        f.add_required_trait(Box::new(AtomicFragment {
            atom: "Baz".to_owned(),
            imports: vec!["crate::Baz".to_owned()],
        }));
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec![],
            atom: indoc! {"
                fn do_something(&mut self) -> Self::Tendies {
                    self.bar_capability();
                    self.baz_yolo_fd()
                }"}
            .to_string(),
        })));
        let mut tendies = TypeDeclaration::new("Tendies".to_owned());
        tendies.add_required_trait(Box::new(TypeFragment::new("GME".to_owned())));
        f.add_associated_type(tendies);

        assert_eq!(
            f.body(80),
            indoc! {"
                trait Foo: Bar + Baz {
                    type Tendies: GME;

                    fn do_something(&mut self) -> Self::Tendies {
                        self.bar_capability();
                        self.baz_yolo_fd()
                    }
                }"}
        );
    }

    #[test]
    fn test_trait_imports() {
        let mut f = TraitFragment::new("Foo".to_owned());
        f.add_required_trait(Box::new(AtomicFragment {
            atom: "Bar".to_owned(),
            imports: vec!["crate::Bar".to_owned()],
        }));
        f.add_required_trait(Box::new(AtomicFragment {
            atom: "Baz".to_owned(),
            imports: vec!["crate::Baz".to_owned()],
        }));
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec!["crate::operators::plus".to_owned()],
            atom: "".to_owned(),
        })));

        assert_eq!(
            f.imports(),
            vec!["crate::Bar", "crate::Baz", "crate::operators::plus"]
        );
    }
}
