use super::{AppendedFragment, AtomicFragment, CodeFragment, ItemDeclaration, ItemDeclarationAPI};
use crate::codegen::StructConfig;
use std::cell::RefCell;
use std::rc::Rc;

/// Fragment for a trait implementation.
pub struct ImplementationFragment {
    /// Config for the trait being implemented.
    trait_cfg: StructConfig,
    /// Config for the struct the trait is being implemented for.
    struct_cfg: StructConfig,
    /// Whether this implementation is in the same file the trait is defined in.
    same_file_as_trait: bool,
    /// Whether this implementation is in the same file the struct is defined in.
    same_file_as_struct: bool,
    /// Declaration fragment for this trait.
    declaration: ItemDeclaration,
    /// Actual internal code fragments for the trait.
    content: Rc<RefCell<AppendedFragment>>,
}

impl ImplementationFragment {
    /// Create a new trait implementation for the given trait and struct.
    pub fn new(trait_cfg: StructConfig, struct_cfg: StructConfig) -> Self {
        Self {
            trait_cfg,
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

    /// Add a fragment to the internals of this implementation.
    pub fn append(&mut self, fragment: Rc<RefCell<dyn CodeFragment>>) {
        self.content.borrow_mut().append(fragment);
    }
}

impl Default for ImplementationFragment {
    fn default() -> Self {
        let mut declaration = ItemDeclaration::default();
        let content = Rc::new(RefCell::new(AppendedFragment::new_with_separator("\n")));
        declaration.set_body(content.clone());
        Self {
            trait_cfg: StructConfig::default(),
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
    fn body(&self) -> String {
        let mut declaration = self.declaration.clone();
        declaration.mark_for_full_implementation();
        declaration.set_definition(Rc::new(RefCell::new(AtomicFragment::new(format!(
            "impl {trait_name} for {struct_name}",
            trait_name = self.trait_cfg.name,
            struct_name = self.struct_cfg.name
        )))));
        declaration.body()
    }

    fn imports(&self) -> Vec<String> {
        let mut imports = self.content.borrow().imports();
        if !self.same_file_as_trait {
            imports.push(self.trait_cfg.import.clone());
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
        let f = ImplementationFragment::new(
            StructConfig {
                name: "Foo".to_owned(),
                import: "crate::Foo".to_owned(),
            },
            StructConfig {
                name: "Bar".to_owned(),
                import: "crate::Bar".to_owned(),
            },
        );

        assert_eq!(
            f.body(),
            indoc! {"
                impl Foo for Bar {
                }"}
        );
    }

    #[test]
    fn test_impl_content() {
        let mut f = ImplementationFragment::new(
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

        assert_eq!(
            f.body(),
            indoc! {"
                impl Foo for Bar {
                    fn foo_capability(&mut self) {
                        self.bar_value = 123;
                    }
                }"}
        );
    }

    #[test]
    fn test_trait_imports_separate_file() {
        let mut f = ImplementationFragment::new(
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
        let mut f = ImplementationFragment::new(
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
