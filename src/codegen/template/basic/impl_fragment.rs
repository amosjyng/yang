use super::{
    Appendable, AppendedFragment, AtomicFragment, CodeFragment, ItemDeclaration, ItemDeclarationAPI,
};
use crate::codegen::StructConfig;
use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for a struct implementation. Optional whether this is implementing a trait or not.
pub struct ImplementationFragment {
    /// Lifetime variables used for defining the lifetime of a Rust object.
    pub lifetime_variables: Vec<char>,
    /// Config for the trait being implemented, if any.
    pub trait_cfg: Option<StructConfig>,
    /// Config for the struct this implementation is a part of.
    pub struct_cfg: StructConfig,
    /// Whether this implementation is in the same file the trait is defined in.
    pub same_file_as_trait: bool,
    /// Whether this implementation is in the same file the struct is defined in.
    pub same_file_as_struct: bool,
    /// Declaration fragment for this trait.
    pub declaration: ItemDeclaration,
    /// Actual internal code fragments for the trait.
    pub content: Rc<RefCell<AppendedFragment>>,
}

impl ImplementationFragment {
    /// Create a new implementation for the given struct.
    pub fn new_struct_impl(struct_cfg: StructConfig) -> Self {
        Self {
            struct_cfg,
            ..Self::default()
        }
    }

    /// Create a new trait implementation for the given trait and struct.
    pub fn new_trait_impl(trait_cfg: StructConfig, struct_cfg: StructConfig) -> Self {
        Self {
            trait_cfg: Some(trait_cfg),
            struct_cfg,
            ..Self::default()
        }
    }

    /// Mark this implementation as being in the same file as the trait definition.
    ///
    /// Matters for import purposes.
    pub fn mark_same_file_as_trait(&mut self) {
        self.same_file_as_trait = true;
    }

    /// Mark this implementation as being in the same file as the struct definition.
    ///
    /// Matters for import purposes.
    pub fn mark_same_file_as_struct(&mut self) {
        self.same_file_as_struct = true;
    }

    /// Add a new lifetime variable to this implementation.
    pub fn add_lifetime(&mut self, lifetime: char) {
        self.lifetime_variables.push(lifetime);
    }
}

impl Appendable for ImplementationFragment {
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

impl Default for ImplementationFragment {
    fn default() -> Self {
        let mut declaration = ItemDeclaration::default();
        let content = Rc::new(RefCell::new(AppendedFragment::default()));
        declaration.set_body(content.clone());
        Self {
            lifetime_variables: vec![],
            trait_cfg: None,
            struct_cfg: StructConfig::default(),
            same_file_as_trait: false,
            same_file_as_struct: false,
            declaration,
            content,
        }
    }
}

impl ItemDeclarationAPI for ImplementationFragment {
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

impl CodeFragment for ImplementationFragment {
    fn body(&self, line_width: usize) -> String {
        let mut declaration = self.declaration.clone();
        declaration.mark_for_full_implementation();
        let lifetimes = if self.lifetime_variables.is_empty() {
            String::new()
        } else {
            format!(
                "<{}>",
                self.lifetime_variables
                    .iter()
                    .map(|v| format!("'{}", v))
                    .format(", ")
                    .to_string()
            )
        };
        let definition = match &self.trait_cfg {
            Some(trait_cfg) => format!(
                "impl{lifetimes} {trait_name} for {struct_name}",
                lifetimes = lifetimes,
                trait_name = trait_cfg.name,
                struct_name = self.struct_cfg.name
            ),
            None => format!("impl {struct_name}", struct_name = self.struct_cfg.name),
        };
        declaration.set_definition(Rc::new(RefCell::new(AtomicFragment::new(definition))));
        declaration.body(line_width) // declaration itself will account for indent size
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.content.borrow().imports();
        if !self.same_file_as_trait {
            if let Some(trait_cfg) = &self.trait_cfg {
                imports.push(trait_cfg.import.clone());
            }
        }
        if !self.same_file_as_struct {
            imports.push(self.struct_cfg.import.clone());
        }
        imports
    }
}

#[cfg(test)]
mod tests {
    use super::super::AtomicFragment;
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_empty_impl() {
        let f = ImplementationFragment::new_struct_impl(StructConfig {
            name: "Bar".to_owned(),
            import: "crate::Bar".to_owned(),
        });

        assert_eq!(f.body(80), "impl Bar {}");
    }

    #[test]
    fn test_empty_trait_impl() {
        let f = ImplementationFragment::new_trait_impl(
            StructConfig {
                name: "Foo".to_owned(),
                import: "crate::Foo".to_owned(),
            },
            StructConfig {
                name: "Bar".to_owned(),
                import: "crate::Bar".to_owned(),
            },
        );

        assert_eq!(f.body(80), "impl Foo for Bar {}");
    }

    #[test]
    fn test_trait_impl_with_lifetimes() {
        let mut f = ImplementationFragment::new_trait_impl(
            StructConfig {
                name: "Foo<'a, 'b>".to_owned(),
                import: "crate::Foo".to_owned(),
            },
            StructConfig {
                name: "Bar".to_owned(),
                import: "crate::Bar".to_owned(),
            },
        );
        f.add_lifetime('a');
        f.add_lifetime('b');

        assert_eq!(f.body(80), "impl<'a, 'b> Foo<'a, 'b> for Bar {}");
    }

    #[test]
    fn test_impl_content() {
        let mut f = ImplementationFragment::new_trait_impl(
            StructConfig {
                name: "Foo".to_owned(),
                import: "crate::Foo".to_owned(),
            },
            StructConfig {
                name: "Bar".to_owned(),
                import: "crate::Bar".to_owned(),
            },
        );
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec![],
            atom: indoc! {"
                fn foo_capability(&mut self) {
                    self.bar_value = 123;
                }"}
            .to_string(),
        })));
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec![],
            atom: indoc! {"
                fn foo_capability2(&mut self) {
                    self.bar_value = 123;
                }"}
            .to_string(),
        })));

        assert_eq!(
            f.body(80),
            indoc! {"
                impl Foo for Bar {
                    fn foo_capability(&mut self) {
                        self.bar_value = 123;
                    }

                    fn foo_capability2(&mut self) {
                        self.bar_value = 123;
                    }
                }"}
        );
    }

    #[test]
    fn test_trait_imports_separate_file() {
        let mut f = ImplementationFragment::new_trait_impl(
            StructConfig {
                name: "Foo".to_owned(),
                import: "crate::Foo".to_owned(),
            },
            StructConfig {
                name: "Bar".to_owned(),
                import: "crate::Bar".to_owned(),
            },
        );
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec!["std::rc::Rc".to_owned()],
            atom: indoc! {"
                fn foo_capability(&mut self) {
                    self.bar_value = 123;
                }"}
            .to_string(),
        })));

        assert_eq!(f.imports(), vec!["std::rc::Rc", "crate::Foo", "crate::Bar"]);
    }

    #[test]
    fn test_trait_imports_same_file() {
        let mut f = ImplementationFragment::new_trait_impl(
            StructConfig {
                name: "Foo".to_owned(),
                import: "crate::Foo".to_owned(),
            },
            StructConfig {
                name: "Bar".to_owned(),
                import: "crate::Bar".to_owned(),
            },
        );
        f.mark_same_file_as_trait();
        f.mark_same_file_as_struct();
        f.append(Rc::new(RefCell::new(AtomicFragment {
            imports: vec!["std::rc::Rc".to_owned()],
            atom: indoc! {"
                fn foo_capability(&mut self) {
                    self.bar_value = 123;
                }"}
            .to_string(),
        })));

        assert_eq!(f.imports(), vec!["std::rc::Rc"]);
    }
}
