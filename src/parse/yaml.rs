use crate::codegen::NameTransform;
use crate::tao::{Implement, ImplementConfig};
use std::convert::TryFrom;
use yaml_rust::YamlLoader;
use zamm_yin::tao::attribute::Attribute;
use zamm_yin::tao::{Form, FormTrait};
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, AttributeArchetype, ArchetypeTrait};
use zamm_yin::node_wrappers::CommonNodeTrait;

/// Parses a YAML string into a list of concepts as represented by the string.
pub fn parse_yaml(yaml: &str) -> Vec<Form> {
    let mut new_concepts = Vec::<Form>::new();
    let docs = YamlLoader::load_from_str(yaml).unwrap();
    let doc = &docs[0];
    for entry in doc.as_vec().unwrap() {
        let parent = Archetype::try_from(entry["parent"].as_str().unwrap()).unwrap();
        let mut new_subtype = parent.individuate_as_archetype();
        if let Some(name) = entry["name"].as_str() {
            let canonical = NameTransform::from(name).to_kebab_case();
            new_subtype.set_internal_name(canonical);
        }
        if let Some(attrs) = entry["attributes"].as_vec() {
            for attr in attrs {
                let attr_name = attr.as_str().unwrap();
                let target_attr = AttributeArchetype::try_from(attr_name).unwrap();
                new_subtype.add_attribute_type(target_attr);
            }
        }
        if parent == Implement::archetype() {
            let mut implement = Implement::from(new_subtype.id());
            let target_name =
                NameTransform::from(entry["target"].as_str().unwrap()).to_kebab_case();
            let target = Archetype::try_from(target_name.as_str()).unwrap();
            implement.set_target(target);

            let impl_config = ImplementConfig {
                id: entry["output_id"].as_i64().unwrap() as usize,
                doc: entry["documentation"].as_str().map(|s| s.to_owned()),
            };
            implement.set_config(impl_config);
        } else if new_subtype.has_ancestor(Attribute::archetype().as_archetype()) {
            // Note: it is exactly because existing functionality is tied to the existing Attribute
            // node, that Attribute cannot be auto-generated right now. When Attribute gets defined
            // in the yin file, the old Attribute gets shadowed (as it should), but then all
            // newly defined subtypes of Attribute no longer pass checks such as this one because
            // they are not subtypes of the old Attribute, and Attribute-specific generation logic
            // never gets activated.
            let mut attr_subtype = AttributeArchetype::from(new_subtype.id());
            if let Some(owner_type_name) = entry["owner_archetype"].as_str() {
                attr_subtype.set_owner_archetype(Archetype::try_from(owner_type_name).unwrap());
            }
            if let Some(value_type_name) = entry["value_archetype"].as_str() {
                attr_subtype.set_value_archetype(Archetype::try_from(value_type_name).unwrap());
            }
        }
        new_concepts.push(new_subtype.as_form());
    }
    new_concepts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tao::{initialize_kb, Implement};
    use indoc::indoc;
    use std::rc::Rc;
    use zamm_yin::tao::attribute::{Attribute, Owner, OwnerArchetype};
    use zamm_yin::tao::{Tao, FormTrait};

    #[test]
    fn test_parse_archetype() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Target
              parent: Attribute
        "});
        assert_eq!(concepts.len(), 1);
        let target = concepts[0];
        assert!(target.has_ancestor(Attribute::archetype().as_archetype()));
        assert_eq!(target.internal_name(), Some(Rc::new("target".to_owned())));
    }

    #[test]
    fn test_parse_archetype_no_name() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - parent: Attribute
        "});
        assert_eq!(concepts.len(), 1);
        let target = concepts[0];
        assert!(target.has_ancestor(Attribute::archetype().as_archetype()));
        assert_eq!(target.internal_name(), None);
    }

    #[test]
    fn test_parse_attribute_owner_value_archetypes() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Target
              parent: Attribute
              owner_archetype: Implement
              value_archetype: Tao
        "});
        assert_eq!(concepts.len(), 1);
        let target = concepts[0];
        assert!(target.has_ancestor(Attribute::archetype().as_archetype()));
        assert_eq!(target.internal_name(), Some(Rc::new("target".to_owned())));
        let target_as_attr_type = AttributeArchetype::from(*target.essence());
        assert_eq!(
            target_as_attr_type.owner_archetype(),
            Implement::archetype()
        );
        assert_eq!(target_as_attr_type.value_archetype(), Tao::archetype());
    }

    #[test]
    fn test_parse_has_attributes() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Foo
              parent: Tao
              attributes:
                - owner
                - owner-archetype
        "});
        assert_eq!(concepts.len(), 1);
        let target = concepts[0];
        assert!(target.has_ancestor(Tao::archetype().as_archetype()));
        assert_eq!(target.internal_name(), Some(Rc::new("foo".to_owned())));
        assert_eq!(
            target.attribute_archetypes(),
            vec![
                Owner::archetype(),
                OwnerArchetype::archetype()
            ]
        );
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
        let implement = Implement::from(concepts[1].id());
        assert!(implement.has_ancestor(Implement::archetype()));
        assert_eq!(
            implement.target().map(|t| t.internal_name()).flatten(),
            Some(Rc::new("target".to_owned()))
        );
        let cfg = implement.config().unwrap();
        assert_eq!(cfg.id, 2);
        assert_eq!(cfg.doc, Some("Howdy, how ya doing?".to_owned()));
    }

    #[test]
    fn test_parse_shadow_old_nodes() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Attribute
              parent: Tao
            - parent: Implement
              target: Attribute
              output_id: 2
        "});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1].id());
        let target = implement.target().unwrap();
        assert_ne!(target, Attribute::archetype().as_archetype());
        assert_eq!(target.introduced_attribute_archetypes(), vec![]);
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
        let implement = Implement::from(concepts[1].id());
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
        let implement = Implement::from(concepts[1].id());
        let cfg = implement.config().unwrap();
        assert_eq!(
            cfg.doc,
            Some("So much to do.\n\nSo little time.".to_owned())
        );
    }
}
