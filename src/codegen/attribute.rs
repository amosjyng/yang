use super::{
    add_autogeneration_comments, add_fmt_skips, into_docstring, CodegenConfig, NameTransform,
};
use crate::concepts::ImplementConfig;

/// Generate code for attributes.
pub fn code_attribute(implement: &ImplementConfig, options: &CodegenConfig) -> String {
    let name_transform = NameTransform::from_camel_case(&implement.name);
    let doc_insert = match &implement.doc {
        Some(d) => format!("\n{}", into_docstring(d.as_str(), 0)),
        None => String::new(),
    };
    let id = if options.yin {
        format!("{}", implement.id)
    } else {
        format!("YIN_MAX_ID + {}", implement.id)
    };
    let crate_name = if options.yin { "crate" } else { "zamm_yin" };
    let imports = if options.yin { "" } else { ", YIN_MAX_ID" };
    let test_imports = if options.yin {
        "use crate::graph::bind_in_memory_graph;"
    } else {
        "use crate::concepts::initialize_kb;"
    };
    let init_kb = if options.yin {
        "bind_in_memory_graph();"
    } else {
        "initialize_kb();"
    };
    let code = format!(
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
    attr: Attribute,
}}

impl Debug for {name} {{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {{
        debug_wrapper("{name}", self, f)
    }}
}}

impl From<usize> for {name} {{
    fn from(id: usize) -> Self {{
        Self {{
            attr: Attribute::from(id),
        }}
    }}
}}

impl<'a> TryFrom<&'a str> for {name} {{
    type Error = String;

    fn try_from(name: &'a str) -> Result<Self, Self::Error> {{
        Attribute::try_from(name).map(|a| Self {{ attr: a }})
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

impl<'a> ArchetypeTrait<'a, {name}> for {name} {{
    const TYPE_ID: usize = {id};
    const TYPE_NAME: &'static str = "{internal_name}";
    const PARENT_TYPE_ID: usize = Attribute::TYPE_ID;

    fn individuate_with_parent(parent_id: usize) -> Self {{
        Self {{
            attr: Attribute::individuate_with_parent(parent_id),
        }}
    }}
}}

impl FormTrait for {name} {{
    fn essence(&self) -> &FinalNode {{
        self.attr.essence()
    }}

    fn essence_mut(&mut self) -> &mut FinalNode {{
        self.attr.essence_mut()
    }}
}}

impl<'a> AttributeTrait<'a, {name}> for {name} {{
    fn set_owner(&mut self, owner: &dyn FormTrait) {{
        self.attr.set_owner(owner);
    }}

    fn owner(&self) -> Option<Tao> {{
        self.attr.owner()
    }}

    fn set_value(&mut self, value: &dyn FormTrait) {{
        self.attr.set_value(value);
    }}

    fn value(&self) -> Option<Tao> {{
        self.attr.value()
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
        crate = crate_name,
        imports = imports,
        test_imports = test_imports,
        init_kb = init_kb,
        name = name_transform.to_camel_case(),
        internal_name = name_transform.to_kebab_case(),
        doc = doc_insert,
        id = id
    );
    let formatted = add_fmt_skips(&code);
    if options.comment_autogen {
        add_autogeneration_comments(&formatted)
    } else {
        formatted
    }
}

#[cfg(test)]
mod tests {
    use super::super::mark_autogen::AUTOGENERATION_MARKER;
    use super::super::mark_fmt::FMT_SKIP_MARKER;
    use super::*;

    #[test]
    fn test_autogen_comments() {
        let code = code_attribute(
            &ImplementConfig {
                name: "dummy".to_owned(),
                doc: None,
                id: 3,
            },
            &CodegenConfig {
                comment_autogen: true,
                track_autogen: false,
                yin: false,
            },
        );
        assert!(code.contains(AUTOGENERATION_MARKER));
        assert!(code.contains("YIN_MAX_ID"));
    }

    #[test]
    fn test_autogen_no_comments() {
        assert!(!code_attribute(
            &ImplementConfig {
                name: "dummy".to_owned(),
                doc: None,
                id: 3,
            },
            &CodegenConfig {
                comment_autogen: false,
                track_autogen: false,
                yin: false,
            }
        )
        .contains(AUTOGENERATION_MARKER));
    }

    #[test]
    fn test_autogen_yin() {
        assert!(!code_attribute(
            &ImplementConfig {
                name: "dummy".to_owned(),
                doc: None,
                id: 3,
            },
            &CodegenConfig {
                comment_autogen: true,
                track_autogen: false,
                yin: true,
            }
        )
        .contains("YIN_MAX_ID"));
    }

    #[test]
    fn test_autogen_fmt_skip() {
        let code = code_attribute(
            &ImplementConfig {
                name: "short".to_owned(),
                doc: None,
                id: 3,
            },
            &CodegenConfig {
                comment_autogen: true,
                track_autogen: false,
                yin: false,
            },
        );
        assert!(!code.contains(FMT_SKIP_MARKER));
    }

    #[test]
    fn test_autogen_fmt_not_skip() {
        let code = code_attribute(
            &ImplementConfig {
                name: "ReallySuperLongClassNameOhBoy".to_owned(),
                doc: None,
                id: 3,
            },
            &CodegenConfig {
                comment_autogen: true,
                track_autogen: false,
                yin: false,
            },
        );
        assert!(code.contains(FMT_SKIP_MARKER));
    }
}
