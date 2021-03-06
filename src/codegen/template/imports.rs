use itertools::Itertools;
use std::collections::HashMap;

fn sort_import_lines(imports: &mut [&str]) {
    imports.sort_by_key(|s| {
        let mut inverted_s = String::new();
        for c in s.chars() {
            if c.is_lowercase() {
                inverted_s += &c.to_uppercase().collect::<String>();
            } else {
                inverted_s += &c.to_lowercase().collect::<String>();
            }
        }
        inverted_s
    });
}

/// Sort imports alphabetically.
fn sort_imports(imports: &[&str]) -> Vec<String> {
    let (mut super_lines, mut other_lines): (Vec<&str>, Vec<&str>) =
        imports.iter().partition(|n| n.starts_with("super::"));
    // do them separately because we want super imports to come first
    sort_import_lines(&mut super_lines);
    sort_import_lines(&mut other_lines);
    super_lines
        .into_iter()
        .chain(other_lines.into_iter())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect()
}

/// Group imports together.
fn group_imports(imports: &[&str]) -> Vec<String> {
    let mut groups = HashMap::new();
    for import in imports {
        let mut path: Vec<&str> = import.split("::").collect();
        let name = path.pop().unwrap();
        groups
            .entry(path.iter().format("::").to_string())
            .or_insert_with(Vec::new)
            .push(name);
    }

    let mut final_imports = Vec::new();
    for (path, names) in &groups {
        let (mut lower, mut upper): (Vec<&str>, Vec<&str>) = names
            .iter()
            .partition(|n| n.chars().next().unwrap().is_lowercase());
        // do them separately because we want lowercase imports to come first, but the default
        // string sort would sort the uppercase ones first
        lower.sort_unstable();
        lower.dedup();
        upper.sort_unstable();
        upper.dedup();
        let import_names = lower.iter().chain(upper.iter()).format(", ").to_string();
        let import = if import_names.contains(", ") {
            // means there was more than one name
            format!("{}::{{{}}}", path, import_names)
        } else {
            format!("{}::{}", path, names.first().unwrap())
        };
        final_imports.push(import);
    }
    final_imports
}

/// Common function for producing import code blocks.
fn imports_as_str_impl(imports: &[&str], public: bool) -> String {
    let grouped_imports = group_imports(imports);
    let grouped_imports_str: Vec<&str> = grouped_imports.iter().map(|i| i.as_str()).collect();
    let sorted_imports = sort_imports(&grouped_imports_str);
    let public_str = if public { "pub " } else { "" };
    // this doesn't need to take into account self.tests because tests don't contribute to file
    // imports
    let mut result = String::new();
    for import in sorted_imports {
        result += &format!("{}use {};\n", public_str, import);
    }
    result.trim().to_owned()
}

/// If the "import" comes from the current crate, replace the current crate name with "crate".
fn replace_current_crate(current_crate: &str, import: String) -> String {
    if import.starts_with(current_crate) {
        format!("crate::{}", &import[current_crate.len() + 2..import.len()])
    } else {
        import
    }
}

/// Serialize imports into a string.
///
/// `current_crate` is always required because we always want to know what it is in order to avoid.
/// `existing_imports` filters out imports that are not needed.
pub fn imports_as_str(
    current_crate: &str,
    imports: Vec<String>,
    existing_imports: &[&str],
) -> String {
    let final_imports = imports
        .into_iter()
        .map(|i| replace_current_crate(current_crate, i))
        .collect::<Vec<String>>();
    let import_strs = final_imports
        .iter()
        .map(|s| s.as_str())
        .filter(|s| !existing_imports.contains(s))
        .collect::<Vec<&str>>();
    imports_as_str_impl(&import_strs, false)
}

/// Serialize re-exports into a string.
pub fn re_exports_as_str(imports: &[&str]) -> String {
    imports_as_str_impl(imports, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::collections::HashSet;

    /// Convert a vec to a set. todo: deduplicate with same thing in Yin's cypher_graph file
    fn vec_as_set(inputs: Vec<String>) -> HashSet<String> {
        let mut set = HashSet::new();
        for input in inputs {
            set.insert(input);
        }
        set
    }

    macro_rules! assert_unordered_eq {
        ($a:expr, $b:expr) => {
            assert_eq!(vec_as_set($a), vec_as_set($b));
        };
    }

    #[test]
    fn test_replace_current_crate() {
        let replaced_import =
            replace_current_crate("my_crate", "my_crate::some::import".to_owned());
        assert_eq!(replaced_import, "crate::some::import");
    }

    #[test]
    fn test_not_replacing_current_crate() {
        let replaced_import =
            replace_current_crate("my_crate", "other_crate::some::import".to_owned());
        assert_eq!(replaced_import, "other_crate::some::import");
    }

    #[test]
    fn test_sort_imports_crate() {
        assert_eq!(
            sort_imports(&[
                "std::convert::TryFrom",
                "std::fmt::{Debug, Formatter}",
                "std::fmt",
                "std::rc::Rc",
                "crate::concept::attributes::{Attribute, AttributeTrait}",
                "crate::concept::{ArchetypeTrait, FormTrait, Tao}",
                "crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode}"
            ]),
            &[
                "crate::concept::attributes::{Attribute, AttributeTrait}",
                "crate::concept::{ArchetypeTrait, FormTrait, Tao}",
                "crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode}",
                "std::convert::TryFrom",
                "std::fmt",
                "std::fmt::{Debug, Formatter}",
                "std::rc::Rc",
            ]
        );
    }

    #[test]
    fn test_sort_imports_super() {
        assert_eq!(
            sort_imports(&[
                "crate::concept::attributes::{Attribute, AttributeTrait}",
                "crate::concept::{ArchetypeTrait, FormTrait, Tao}",
                "crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode}",
                "super::ParentTrait"
            ]),
            &[
                "super::ParentTrait",
                "crate::concept::attributes::{Attribute, AttributeTrait}",
                "crate::concept::{ArchetypeTrait, FormTrait, Tao}",
                "crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode}"
            ]
        );
    }

    #[test]
    fn test_sort_imports_ignore_empty() {
        assert_eq!(
            sort_imports(&[
                "crate::concept::attributes::{Attribute, AttributeTrait}",
                "",
                "crate::concept::{ArchetypeTrait, FormTrait, Tao}",
                "",
                "",
                "crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode}",
                "super::ParentTrait"
            ]),
            &[
                "super::ParentTrait",
                "crate::concept::attributes::{Attribute, AttributeTrait}",
                "crate::concept::{ArchetypeTrait, FormTrait, Tao}",
                "crate::node_wrappers::{debug_wrapper, CommonNodeTrait, FinalNode}",
            ]
        );
    }

    #[test]
    fn test_sort_imports_struct_vs_module() {
        assert_eq!(
            sort_imports(&[
                "zamm_yin::tao::Tao",
                "zamm_yin::tao::archetype::ArchetypeFormTrait",
                "zamm_yin::tao::attribute::{Owner, Value}",
            ]),
            &[
                "zamm_yin::tao::archetype::ArchetypeFormTrait",
                "zamm_yin::tao::attribute::{Owner, Value}",
                "zamm_yin::tao::Tao",
            ]
        );
    }

    #[test]
    fn test_sort_imports_module_no_struct() {
        assert_eq!(
            sort_imports(&["std::fmt", "std::convert::TryFrom",]),
            &["std::convert::TryFrom", "std::fmt",]
        );
    }

    #[test]
    fn test_group_import_none() {
        assert_eq!(group_imports(&[]), Vec::<String>::new());
    }

    #[test]
    fn test_group_import_uniques() {
        assert_unordered_eq!(
            group_imports(&["std::cell::Cell", "std::rc::Rc"]),
            vec!["std::cell::Cell".to_owned(), "std::rc::Rc".to_owned()]
        );
    }

    #[test]
    fn test_group_import_groups() {
        assert_unordered_eq!(
            group_imports(&["std::cell::RefCell", "std::rc::Rc", "std::cell::Cell"]),
            vec![
                "std::cell::{Cell, RefCell}".to_owned(),
                "std::rc::Rc".to_owned()
            ]
        );
    }

    #[test]
    fn test_group_import_repeats() {
        assert_unordered_eq!(
            group_imports(&["std::cell::RefCell", "std::rc::Rc", "std::cell::RefCell"]),
            vec!["std::cell::RefCell".to_owned(), "std::rc::Rc".to_owned()]
        );
    }

    #[test]
    fn test_group_imports_lowercase() {
        assert_unordered_eq!(
            group_imports(&[
                "my::mod::lowercase",
                "my::mod::ABCs",
                "my::mod::btr",
                "my::mod::KComplexity"
            ]),
            vec!["my::mod::{btr, lowercase, ABCs, KComplexity}".to_owned()]
        );
    }

    #[test]
    fn test_imports_as_str() {
        assert_eq!(
            imports_as_str(
                "my_crate",
                vec![
                    "std::cell::RefCell".to_owned(),
                    "std::rc::Rc".to_owned(),
                    "std::cell::Cell".to_owned()
                ],
                &[]
            ),
            indoc! {"
                use std::cell::{Cell, RefCell};
                use std::rc::Rc;"}
        );
    }

    #[test]
    fn test_imports_as_str_filtered() {
        assert_eq!(
            imports_as_str(
                "my_crate",
                vec![
                    "std::cell::RefCell".to_owned(),
                    "std::rc::Rc".to_owned(),
                    "std::cell::Cell".to_owned()
                ],
                &["std::cell::Cell"]
            ),
            indoc! {"
                use std::cell::RefCell;
                use std::rc::Rc;"}
        );
    }

    #[test]
    fn test_re_exports_as_str() {
        assert_eq!(
            re_exports_as_str(&["std::cell::RefCell", "std::rc::Rc", "std::cell::Cell"]),
            indoc! {"
                pub use std::cell::{Cell, RefCell};
                pub use std::rc::Rc;"}
        );
    }
}
