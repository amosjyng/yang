use itertools::Itertools;

/// Sort imports alphabetically.
pub fn sort_imports(imports: &str) -> String {
    let mut import_lines: Vec<&str> = imports.split('\n').collect();
    import_lines.sort_by_key(|s| {
        let n = s.len();
        if n > 0 {
            &s[..n - 1]
        } else {
            s
        }
    });
    import_lines
        .into_iter()
        .format("\n")
        .to_string()
        .trim()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_sort_imports_crate() {
        assert_eq!(
            sort_imports(indoc! {"
            use std::convert::TryFrom;
            use std::fmt::{Debug, Formatter};
            use std::fmt;
            use std::rc::Rc;
            use crate::concepts::attributes::{Attribute, AttributeTrait};
            use crate::concepts::{ArchetypeTrait, FormTrait, Tao{imports}};
            use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
        "}),
            indoc! {"
            use crate::concepts::attributes::{Attribute, AttributeTrait};
            use crate::concepts::{ArchetypeTrait, FormTrait, Tao{imports}};
            use crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode};
            use std::convert::TryFrom;
            use std::fmt;
            use std::fmt::{Debug, Formatter};
            use std::rc::Rc;
        "}
            .trim()
        );
    }
}
