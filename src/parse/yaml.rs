use crate::concepts::Implement;
use crate::concepts::ImplementConfig;
use std::convert::TryFrom;
use yaml_rust::YamlLoader;
use zamm_yin::concepts::{Archetype, ArchetypeTrait, FormTrait, Tao};
use zamm_yin::node_wrappers::CommonNodeTrait;

/// Parses a YAML string into a list of concepts as represented by the string.
pub fn parse_yaml(yaml: &str) -> Vec<Tao> {
    let mut new_concepts = Vec::<Tao>::new();
    let docs = YamlLoader::load_from_str(yaml).unwrap();
    let doc = &docs[0];
    for entry in doc.as_vec().unwrap() {
        let parent = Archetype::try_from(entry["parent"].as_str().unwrap()).unwrap();
        let mut new_concept = Tao::individuate_with_parent(parent.id());
        if let Some(name) = entry["name"].as_str() {
            new_concept.set_internal_name(name.to_owned());
        }
        if parent == Implement::archetype() {
            let mut implement = Implement::from(new_concept);
            let target_name = entry["target"].as_str().unwrap();
            let target = Archetype::try_from(target_name).unwrap();
            implement.set_target(target);
            let parent = target.parents().first().unwrap().internal_name().unwrap();
            let impl_config = ImplementConfig {
                name: target_name.to_owned(),
                parent_name: parent.to_owned().to_string(),
                id: entry["output_id"].as_i64().unwrap() as usize,
                doc: entry["documentation"].as_str().map(|s| s.to_owned()),
            };
            implement.set_config(impl_config);
        }
        new_concepts.push(new_concept);
    }
    new_concepts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::{initialize_kb, Implement};
    use indoc::indoc;
    use std::rc::Rc;
    use zamm_yin::concepts::attributes::Attribute;
    use zamm_yin::concepts::FormTrait;

    #[test]
    fn test_parse_archetype() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Target
              parent: Attribute
        "});
        assert_eq!(concepts.len(), 1);
        let target = concepts[0];
        assert!(target.has_ancestor(Attribute::archetype()));
        assert_eq!(target.internal_name(), Some(Rc::new("Target".to_owned())));
    }

    #[test]
    fn test_parse_archetype_no_name() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - parent: Attribute
        "});
        assert_eq!(concepts.len(), 1);
        let target = concepts[0];
        assert!(target.has_ancestor(Attribute::archetype()));
        assert_eq!(target.internal_name(), None);
    }

    #[test]
    fn test_parse_implement() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Target
              parent: Attribute
            - parent: Implement
              target: Target
              output_id: 2
              documentation: Howdy, how ya doing?
        "});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1]);
        assert!(implement.has_ancestor(Implement::archetype()));
        assert_eq!(
            implement.target().map(|t| t.internal_name()).flatten(),
            Some(Rc::new("Target".to_owned()))
        );
        let cfg = implement.config().unwrap();
        assert_eq!(cfg.id, 2);
        assert_eq!(cfg.doc, Some("Howdy, how ya doing?".to_owned()));
    }

    #[test]
    fn test_parse_multiline_string() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Target
              parent: Attribute
            - parent: Implement
              target: Target
              output_id: 2
              documentation: |-
                So much to do.
                So little time.
        "});
        let implement = Implement::from(concepts[1]);
        let cfg = implement.config().unwrap();
        assert_eq!(cfg.doc, Some("So much to do.\nSo little time.".to_owned()));
    }

    #[test]
    fn test_parse_multiline_string_multiple_breaks() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Target
              parent: Attribute
            - parent: Implement
              target: Target
              output_id: 2
              documentation: |-
                So much to do.

                So little time.
        "});
        let implement = Implement::from(concepts[1]);
        let cfg = implement.config().unwrap();
        assert_eq!(
            cfg.doc,
            Some("So much to do.\n\nSo little time.".to_owned())
        );
    }
}
