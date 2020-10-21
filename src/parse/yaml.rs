use crate::tao::archetype::CodegenFlags;
use crate::tao::form::DefinedMarker;
use crate::tao::{Implement, ImplementConfig};
use heck::KebabCase;
use std::collections::HashMap;
use std::convert::TryFrom;
use yaml_rust::{Yaml, YamlLoader};
use zamm_yin::node_wrappers::CommonNodeTrait;
use zamm_yin::tao::archetype::{Archetype, ArchetypeFormTrait, ArchetypeTrait, AttributeArchetype};
use zamm_yin::tao::attribute::Attribute;
use zamm_yin::tao::{Form, FormTrait};

fn parse_attr_info(new_subtype: &mut Archetype, entry: &Yaml) {
    let mut attr_subtype = AttributeArchetype::from(new_subtype.id());
    if let Some(owner_type_name) = entry["owner_archetype"].as_str() {
        attr_subtype.set_owner_archetype(
            Archetype::try_from(owner_type_name.to_kebab_case().as_str()).unwrap(),
        );
    }
    if let Some(value_type_name) = entry["value_archetype"].as_str() {
        attr_subtype.set_value_archetype(
            Archetype::try_from(value_type_name.to_kebab_case().as_str()).unwrap(),
        );
    }
}

/// Parses a YAML string into a list of concepts as represented by the string.
pub fn parse_yaml(yaml: &str) -> Vec<Form> {
    let mut new_concepts = Vec::<Form>::new();
    let docs = YamlLoader::load_from_str(yaml).unwrap();
    let doc = &docs[0];
    let mut entries = HashMap::new();
    for entry in doc.as_vec().unwrap() {
        let parent_name = entry["parent"].as_str().unwrap().to_kebab_case();
        let parent = Archetype::try_from(parent_name.as_str()).unwrap();
        let mut new_concept = parent.individuate_as_form();
        new_concept.mark_newly_defined();
        entries.insert(new_concept.id(), entry.clone());

        if let Some(name) = entry["name"].as_str() {
            new_concept.set_internal_name(name.to_kebab_case());
        }
        if let Some(attrs) = entry["attributes"].as_vec() {
            // attributes can only be specified for new types, not new individuals unless the
            // individual is a singleton
            let mut new_subtype = Archetype::from(new_concept.id());
            for attr in attrs {
                let canonical = attr.as_str().unwrap().to_kebab_case();
                let target_attr = AttributeArchetype::try_from(canonical.as_str()).unwrap();
                new_subtype.add_attribute_type(target_attr);
            }
        }
    }
    // some entries cannot be parsed until a later entry gives it more context, so do a second pass
    // todo: get all new nodes, once Yin supports that
    let mut node_ids = entries.keys().collect::<Vec<&usize>>();
    node_ids.sort(); // for determinism
    for new_concept_id in node_ids {
        let new_concept = Form::from(*new_concept_id);
        let entry = &entries[new_concept_id];
        if new_concept.has_ancestor(Implement::archetype()) {
            let mut implement = Implement::from(new_concept.id());
            let target_name = entry["target"].as_str().unwrap().to_kebab_case();
            let mut target = Archetype::try_from(target_name.as_str()).unwrap();
            implement.set_target(target);

            if entry["attribute_logic"].as_bool().unwrap_or(false) {
                target.activate_attribute_logic();
            }
            // separate if-statement because attribute logic activation gets inherited
            if target.attribute_logic_activated() {
                let target_id = target.id();
                parse_attr_info(&mut target, &entries[&target_id]);
            }

            if entry["own_module"].as_bool().unwrap_or(false) {
                target.mark_own_module();
            }

            let impl_config = ImplementConfig {
                id: entry["output_id"].as_i64().unwrap() as usize,
                doc: entry["documentation"].as_str().map(|s| s.to_owned()),
            };
            implement.set_config(impl_config);
        } else if new_concept.has_ancestor(Attribute::archetype().as_archetype()) {
            parse_attr_info(&mut Archetype::from(new_concept.id()), entry);
        }
        new_concepts.push(new_concept);
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
    use zamm_yin::tao::{FormTrait, Tao};

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
        assert!(target.is_newly_defined());
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
                - OwnerArchetype
        "});
        assert_eq!(concepts.len(), 1);
        let target = concepts[0];
        assert!(target.has_ancestor(Tao::archetype().as_archetype()));
        assert_eq!(target.internal_name(), Some(Rc::new("foo".to_owned())));
        assert_eq!(
            target.attribute_archetypes(),
            vec![Owner::archetype(), OwnerArchetype::archetype()]
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
        let target = implement.target().unwrap();
        assert_eq!(target.internal_name(), Some(Rc::new("target".to_owned())));
        assert!(target.as_form().is_newly_defined());
        assert!(!target.attribute_logic_activated());
        let cfg = implement.config().unwrap();
        assert_eq!(cfg.id, 2);
        assert_eq!(cfg.doc, Some("Howdy, how ya doing?".to_owned()));
    }

    #[test]
    fn test_parse_implement_attr() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Attribute
              parent: Tao
            - parent: Implement
              target: Attribute
              output_id: 2
              attribute_logic: true
        "});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1].id());
        let target = implement.target().unwrap();
        assert!(target.attribute_logic_activated());
        let cfg = implement.config().unwrap();
        assert_eq!(cfg.id, 2);
        assert_eq!(cfg.doc, None);
    }

    #[test]
    fn test_parse_implement_attr_archetypes() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: Attribute
              parent: Tao
              owner_archetype: Tao
              value_archetype: Form
            - parent: Implement
              target: Attribute
              output_id: 2
              attribute_logic: true
        "});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1].id());
        let target = AttributeArchetype::from(implement.target().unwrap().id());
        assert!(target.attribute_logic_activated());
        assert_eq!(target.owner_archetype(), Tao::archetype());
        assert_eq!(target.value_archetype(), Form::archetype());
        let cfg = implement.config().unwrap();
        assert_eq!(cfg.id, 2);
        assert_eq!(cfg.doc, None);
    }

    #[test]
    fn test_parse_implement_inherited_attr_archetypes() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: MyAttr
              parent: Tao
              owner_archetype: Tao
              value_archetype: Tao
            - name: Color
              parent: MyAttr
              value_archetype: Form
            - parent: Implement
              target: MyAttr
              output_id: 2
              attribute_logic: true
            - parent: Implement
              target: Color
              output_id: 3
        "});
        assert_eq!(concepts.len(), 4);
        let target = AttributeArchetype::from(concepts[1].id());
        assert!(target.attribute_logic_activated());
        assert_eq!(
            target
                .ancestry()
                .into_iter()
                .map(|a| a.internal_name().unwrap())
                .collect::<Vec<Rc<String>>>(),
            vec![Rc::new("tao".to_owned()), Rc::new("my-attr".to_owned())]
        );
        assert_eq!(target.owner_archetype(), Tao::archetype());
        assert_eq!(target.value_archetype(), Form::archetype());
    }

    #[test]
    fn test_parse_implement_own_module_unmarked() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: MyType
              parent: Tao
            - parent: Implement
              target: MyType
              output_id: 2
        "});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1].id());
        let target = implement.target().unwrap();
        assert!(!target.attribute_logic_activated());
        assert!(!target.own_module());
    }

    #[test]
    fn test_parse_implement_own_module_marked() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - name: MyType
              parent: Tao
            - parent: Implement
              target: MyType
              output_id: 2
              own_module: true
        "});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1].id());
        let target = implement.target().unwrap();
        assert!(!target.attribute_logic_activated());
        assert!(target.own_module());
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
