use super::{add_autogeneration_comments, into_docstring};
use std::rc::Rc;

/// Generate code for attributes.
pub fn code_attribute(name: &str, doc: Option<Rc<String>>, id: usize) -> String {
    let doc_insert = match doc {
        Some(d) => format!("\n{}", into_docstring(d.as_str(), 0)),
        None => String::new(),
    };
    let code = format!(
        r##"use std::fmt::{{Debug, Formatter, Result}};
use std::rc::Rc;
use zamm_yin::concepts::attributes::{{Attribute, AttributeTrait}};
use zamm_yin::concepts::{{ArchetypeTrait, FormTrait, Tao, YIN_MAX_ID}};
use zamm_yin::wrappers::{{debug_wrapper, CommonNodeTrait, FinalWrapper}};
{doc}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct {name} {{
    attr: Attribute,
}}

impl Debug for {name} {{
    fn fmt(&self, f: &mut Formatter) -> Result {{
        debug_wrapper("{name}", Box::new(self), f)
    }}
}}

impl From<usize> for {name} {{
    fn from(id: usize) -> Self {{
        Self {{
            attr: Attribute::from(id),
        }}
    }}
}}

impl CommonNodeTrait for {name} {{
    fn id(&self) -> usize {{
        self.attr.id()
    }}

    fn set_internal_name(&mut self, name: String) {{
        self.attr.set_internal_name(name);
    }}

    fn internal_name(&self) -> Option<Rc<String>> {{
        self.attr.internal_name()
    }}
}}

impl ArchetypeTrait<{name}> for {name} {{
    const TYPE_ID: usize = YIN_MAX_ID + {id};
    const TYPE_NAME: &'static str = "{name}";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {{
        Self {{
            attr: Attribute::individuate_with_parent(parent_id),
        }}
    }}
}}

impl FormTrait for {name} {{
    fn essence(&self) -> &FinalWrapper {{
        self.attr.essence()
    }}

    fn essence_mut(&mut self) -> &mut FinalWrapper {{
        self.attr.essence_mut()
    }}
}}

impl AttributeTrait<{name}> for {name} {{
    fn set_owner(&mut self, owner: Box<&dyn FormTrait>) {{
        self.attr.set_owner(owner);
    }}

    fn owner(&self) -> Option<Tao> {{
        self.attr.owner()
    }}

    fn set_value(&mut self, value: Box<&dyn FormTrait>) {{
        self.attr.set_value(value);
    }}

    fn value(&self) -> Option<Tao> {{
        self.attr.value()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    use crate::concepts::initialize_kb;

    #[test]
    fn check_type_created() {{
        initialize_kb();
        assert_eq!({name}::archetype().id(), {name}::TYPE_ID);
        assert_eq!(
            {name}::archetype().internal_name(),
            Some(Rc::new({name}::TYPE_NAME.to_string()))
        );
    }}

    #[test]
    fn from_node_id() {{
        initialize_kb();
        let concept = {name}::individuate();
        let concept_copy = {name}::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }}

    #[test]
    fn create_and_retrieve_node_id() {{
        initialize_kb();
        let concept1 = {name}::individuate();
        let concept2 = {name}::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }}

    #[test]
    fn create_and_retrieve_node_name() {{
        initialize_kb();
        let mut concept = {name}::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }}

    #[test]
    fn get_owner() {{
        initialize_kb();
        let mut instance = {name}::individuate();
        let owner_of_owner = {name}::individuate();
        instance.set_owner(Box::new(&owner_of_owner));
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(instance.value(), None);
    }}

    #[test]
    fn get_value() {{
        initialize_kb();
        let mut instance = {name}::individuate();
        let value_of_owner = {name}::individuate();
        instance.set_value(Box::new(&value_of_owner));
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    }}
}}
"##,
        name = name,
        doc = doc_insert,
        id = id
    );
    add_autogeneration_comments(&code)
}
