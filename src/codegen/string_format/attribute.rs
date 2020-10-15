use super::FormatConfig;

/// Generate code for attributes.
pub fn code_attribute(cfg: &FormatConfig) -> String {
    format!(
        r##"use std::convert::TryFrom;
use std::fmt;
use std::fmt::{{Debug, Formatter}};
use std::rc::Rc;
use {crate}::concepts::attributes::{{Attribute, AttributeTrait}};
use {crate}::concepts::{{ArchetypeTrait, FormTrait, Tao{imports}}};
use {crate}::node_wrappers::{{debug_wrapper, CommonNodeTrait, FinalNode}};
{doc}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct {name} {{
    base: {parent},
}}

impl Debug for {name} {{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {{
        debug_wrapper("{name}", self, f)
    }}
}}

impl From<usize> for {name} {{
    fn from(id: usize) -> Self {{
        Self {{
            base: {parent}::from(id),
        }}
    }}
}}

impl<'a> TryFrom<&'a str> for {name} {{
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {{
        {parent}::try_from(name).map(|a| Self {{ base: a }})
    }}
}}

impl CommonNodeTrait for {name} {{
    fn id(&self) -> usize {{
        self.base.id()
    }}

    fn set_internal_name(&mut self, name: String) {{
        self.base.set_internal_name(name);
    }}

    fn internal_name(&self) -> Option<Rc<String>> {{
        self.base.internal_name()
    }}
}}

impl<'a> ArchetypeTrait<'a, {name}> for {name} {{
    const TYPE_ID: usize = {id};
    const TYPE_NAME: &'static str = "{internal_name}";
    const PARENT_TYPE_ID: usize = {parent}::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {{
        Self {{
            base: {parent}::individuate_with_parent(parent_id),
        }}
    }}
}}

impl FormTrait for {name} {{
    fn essence(&self) -> &FinalNode {{
        self.base.essence()
    }}

    fn essence_mut(&mut self) -> &mut FinalNode {{
        self.base.essence_mut()
    }}
}}

impl<'a> AttributeTrait<'a, {name}> for {name} {{
    fn set_owner(&mut self, owner: &dyn FormTrait) {{
        self.base.set_owner(owner);
    }}

    fn owner(&self) -> Option<Tao> {{
        self.base.owner()
    }}

    fn set_value(&mut self, value: &dyn FormTrait) {{
        self.base.set_value(value);
    }}

    fn value(&self) -> Option<Tao> {{
        self.base.value()
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    {test_imports}

    #[test]
    fn check_type_created() {{
        {init_kb}
        assert_eq!({name}::archetype().id(), {name}::TYPE_ID);
        assert_eq!(
            {name}::archetype().internal_name(),
            Some(Rc::new({name}::TYPE_NAME.to_string()))
        );
    }}

    #[test]
    fn from_node_id() {{
        {init_kb}
        let concept = {name}::individuate();
        let concept_copy = {name}::from(concept.id());
        assert_eq!(concept.id(), concept_copy.id());
    }}

    #[test]
    fn from_name() {{
        {init_kb}
        let mut concept = {name}::individuate();
        concept.set_internal_name("A".to_owned());
        assert_eq!({name}::try_from("A"), Ok(concept));
        assert!({name}::try_from("B").is_err());
    }}

    #[test]
    fn create_and_retrieve_node_id() {{
        {init_kb}
        let concept1 = {name}::individuate();
        let concept2 = {name}::individuate();
        assert_eq!(concept1.id() + 1, concept2.id());
    }}

    #[test]
    fn create_and_retrieve_node_name() {{
        {init_kb}
        let mut concept = {name}::individuate();
        concept.set_internal_name("A".to_string());
        assert_eq!(concept.internal_name(), Some(Rc::new("A".to_string())));
    }}

    #[test]
    fn get_owner() {{
        {init_kb}
        let mut instance = {name}::individuate();
        let owner_of_owner = {name}::individuate();
        instance.set_owner(&owner_of_owner);
        assert_eq!(instance.owner(), Some(owner_of_owner.ego_death()));
        assert_eq!(instance.value(), None);
    }}

    #[test]
    fn get_value() {{
        {init_kb}
        let mut instance = {name}::individuate();
        let value_of_owner = {name}::individuate();
        instance.set_value(&value_of_owner);
        assert_eq!(instance.owner(), None);
        assert_eq!(instance.value(), Some(value_of_owner.ego_death()));
    }}
}}
"##,
        crate = cfg.yin_crate,
        imports = cfg.imports,
        test_imports = cfg.test_imports,
        init_kb = cfg.init_kb,
        name = cfg.name,
        internal_name = cfg.internal_name,
        parent = cfg.parent_name,
        doc = cfg.doc,
        id = cfg.id,
    )
}
