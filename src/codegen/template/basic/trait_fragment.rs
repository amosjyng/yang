use super::{AppendedFragment, AtomicFragment, CodeFragment, ItemDeclaration, ItemDeclarationAPI};
use crate::codegen::StructConfig;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for a trait.
pub struct TraitFragment {
    /// Name of the trait.
    name: String,
    /// Declaration fragment for this trait.
    declaration: ItemDeclaration,
    /// Any traits that are required to be implemented before this one.
    required_traits: Vec<StructConfig>,
    /// Actual internal code fragments for the trait.
    content: Rc<RefCell<AppendedFragment>>,
}

impl TraitFragment {
    /// Create a new trait with the given name.
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }

    /// Add a trait requirement
    pub fn add_required_trait(&mut self, required_trait: StructConfig) {
        self.required_traits.push(required_trait);
    }

    /// Add a fragment to the internals of this trait.
    pub fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.content.borrow_mut().append(fragment);
    }
}

impl Default for TraitFragment {
    fn default() -> Self {
        let mut declaration = ItemDeclaration::default();
        let content = Rc::new(RefCell::new(AppendedFragment::new_with_separator("\n")));
        declaration.set_body(content.clone());
        Self {
            name: String::default(),
            declaration,
            required_traits: vec![],
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
        let required_traits = self.required_traits.iter().map(|r| &r.name).format(" + ");
        let requirements = if self.required_traits.is_empty() {
            String::default()
        } else {
            format!(": {}", required_traits)
        };
        let mut declaration = self.declaration.clone();
        declaration.mark_for_full_implementation();
        declaration.set_definition(Rc::new(RefCell::new(AtomicFragment::new(format!(
            "trait {name}{requirements}",
            name = self.name,
            requirements = requirements
        )))));
        declaration.body(line_width) // declaration will take care of indent size
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self
            .required_traits
            .iter()
            .map(|r| r.import.clone())
            .collect::<Vec<String>>();
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
        f.add_required_trait(StructConfig {
            name: "Bar".to_owned(),
            import: "crate::Bar".to_owned(),
        });
        f.add_required_trait(StructConfig {
            name: "Baz".to_owned(),
            import: "crate::Baz".to_owned(),
        });
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
    fn test_trait_imports() {
        let mut f = TraitFragment::new("Foo".to_owned());
        f.add_required_trait(StructConfig {
            name: "Bar".to_owned(),
            import: "crate::Bar".to_owned(),
        });
        f.add_required_trait(StructConfig {
            name: "Baz".to_owned(),
            import: "crate::Baz".to_owned(),
        });
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
