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
use zamm_yin::tao::{Tao, Form, FormTrait};

fn parse_attr_info(new_subtype: &mut Archetype, entry: &HashMap<String, Yaml>) {
    let mut attr_subtype = AttributeArchetype::from(new_subtype.id());
    if let Some(owner_type_name) = entry.get("owner_archetype").map(|y| y.as_str()).flatten() {
        attr_subtype.set_owner_archetype(
            Archetype::try_from(owner_type_name.to_kebab_case().as_str()).unwrap(),
        );
    }
    if let Some(value_type_name) = entry.get("value_archetype").map(|y| y.as_str()).flatten() {
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

    // define everything first
    for entry in doc.as_vec().unwrap() {
        if let Some(name) = entry["define"].as_str() {
            let mut new_concept = Tao::individuate();
            // that's all we have to do for now, just ensure that this name now exists for this node
            new_concept.set_internal_name(name.to_kebab_case());
        }
    }

    // then collect all entries for the same node together
    for entry in doc.as_vec().unwrap() {
        let current_concept = if let Some(name) = entry["define"].as_str() {
            // it's already been defined, just find it again
            let mut new_concept = Form::try_from(name.to_kebab_case().as_str()).unwrap();
            new_concept.mark_newly_defined();
            new_concept
        } else if let Some(name) = entry["name"].as_str() {
            Form::try_from(name.to_kebab_case().as_str()).unwrap()
        } else {
            // Not referring to existing node (no "name"), so it must be a new node. Not naming the // new node either (no "define"), so don't bother giving it a name.
            let mut new_concept = Tao::individuate();
            new_concept.mark_newly_defined();
            new_concept
        };
        
        let existing_entry =
        entries.entry(current_concept.id()).or_insert_with(|| HashMap::<String, Yaml>::new());

        for (k, v) in entry.as_hash().unwrap() {
            existing_entry.insert(k.as_str().unwrap().to_owned(), v.clone());
        }
    }

    // now parse all new nodes
    let mut node_ids = entries.keys().collect::<Vec<&usize>>();
    node_ids.sort(); // for determinism
    for concept_id in node_ids {
        let mut current_concept = Form::from(*concept_id);
        let entry = &entries[concept_id];

        if let Some(parent_name) = entry.get("parent").map(|y| y.as_str()).flatten() {
            let parent = Archetype::try_from(parent_name.to_kebab_case().as_str()).unwrap();
            current_concept.add_parent(parent);
        }

        if let Some(attrs) = entry.get("attributes").map(|y| y.as_vec()).flatten() {
            // attributes can only be specified for new types, not new individuals unless the
            // individual is a singleton
            let mut new_subtype = Archetype::from(*concept_id);
            for attr in attrs {
                let canonical = attr.as_str().unwrap().to_kebab_case();
                let target_attr = AttributeArchetype::try_from(canonical.as_str()).unwrap();
                new_subtype.add_attribute_type(target_attr);
            }
        }

        if current_concept.has_ancestor(Implement::archetype()) {
            let mut implement = Implement::from(*concept_id);
            let target_name = entry["target"].as_str().unwrap().to_kebab_case();
            let mut target = Archetype::try_from(target_name.as_str()).unwrap();
            implement.set_target(target);

            if entry.get("attribute_logic").map(|y| y.as_bool()).flatten().unwrap_or(false) {
                target.activate_attribute_logic();
            }
            // separate if-statement because attribute logic activation gets inherited
            if target.attribute_logic_activated() {
                let target_id = target.id();
                parse_attr_info(&mut target, &entries[&target_id]);
            }

            if entry.get("force_own_module").map(|y| y.as_bool()).flatten().unwrap_or(false) {
                target.mark_own_module();
            }

            let impl_config = ImplementConfig {
                id: entry["output_id"].as_i64().unwrap() as usize,
                doc: entry.get("documentation").map(|s| s.as_str().unwrap().to_owned()),
            };
            implement.set_config(impl_config);
        } else if current_concept.has_ancestor(Attribute::archetype().as_archetype()) {
            parse_attr_info(&mut Archetype::from(current_concept.id()), entry);
        }
        new_concepts.push(current_concept);
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
            - define: Target
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
            - define: Target
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
            - define: Foo
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
            - define: Target
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
            - define: Attribute
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
            - define: Attribute
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
    fn test_parse_split_up_definition() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - define: Attribute
              parent: Tao
            - name: Attribute
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
    fn test_parse_define_only() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - define: Attribute
            - name: Attribute
              parent: Tao
            - name: Attribute
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
    fn test_parse_out_of_order() {
        initialize_kb();

        // refer to attribute even before definition
        let concepts = parse_yaml(indoc! {"
            - name: Attribute
              parent: Tao
            - define: Attribute
            - name: Attribute
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
            - define: MyAttr
              parent: Tao
              owner_archetype: Tao
              value_archetype: Tao
            - define: Color
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
            - define: MyType
              parent: Tao
            - parent: Implement
              target: MyType
              output_id: 2
        "});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1].id());
        let target = implement.target().unwrap();
        assert!(!target.attribute_logic_activated());
        assert!(!target.force_own_module());
    }

    #[test]
    fn test_parse_implement_own_module_marked() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - define: MyType
              parent: Tao
            - parent: Implement
              target: MyType
              output_id: 2
              force_own_module: true
        "});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1].id());
        let target = implement.target().unwrap();
        assert!(!target.attribute_logic_activated());
        assert!(target.force_own_module());
    }

    #[test]
    fn test_parse_shadow_old_nodes() {
        initialize_kb();

        let concepts = parse_yaml(indoc! {"
            - define: Attribute
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
            - define: Target
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
            - define: Target
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
