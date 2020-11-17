use super::CodeFragment;

/// Code fragment that cannot be broken down any further.
pub struct AtomicFragment {
    /// Imports for the fragment.
    pub imports: Vec<String>,
    /// Body of the fragment.
    pub atom: String,
}

impl AtomicFragment {
    /// Create a new AtomicFragment with no imports.
    pub fn new(atom: String) -> Self {
        Self {
            imports: Vec::<String>::default(),
            atom,
        }
    }
}

impl CodeFragment for AtomicFragment {
    fn body(&self) -> String {
        self.atom.trim().to_string()
    }

    fn imports(&self) -> Vec<String> {
        self.imports.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atom() {
        let line = AtomicFragment {
            imports: vec!["foreign_crate::sub::ForeignStruct".to_owned()],
            atom: "let mut f = ForeignStruct {};".to_owned(),
        };
        assert_eq!(
            line.imports(),
            vec!["foreign_crate::sub::ForeignStruct".to_owned()]
        );
        assert_eq!(line.body(), "let mut f = ForeignStruct {};".to_owned());
    }
}
