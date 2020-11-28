use super::tao::TaoConfig;
use crate::codegen::template::basic::{AtomicFragment, FileFragment};
use crate::codegen::StructConfig;
use indoc::formatdoc;
use std::cell::RefCell;
use std::rc::Rc;

/// Config values at the time of Attribute code generation.
#[derive(Default)]
pub struct AttributeFormatConfig {
    /// Regular concept config.
    pub tao_cfg: TaoConfig,
    /// Attribute's owner archetype.
    pub owner_type: StructConfig,
    /// Attribute's owner form.
    pub owner_form: StructConfig,
    /// Attribute's value archetype.
    pub value_type: StructConfig,
    /// Attribute's value form.
    pub value_form: StructConfig,
    /// Code to use for converting the owner archetype type into `Archetype`.
    pub owner_into_archetype: String,
    /// Code to use for converting the value archetype type into `Archetype`.
    pub value_into_archetype: String,
}

/// Get the attribute body fragment.
fn attribute_fragment(cfg: &AttributeFormatConfig) -> AtomicFragment {
    AtomicFragment {
        imports: vec![
            "zamm_yin::tao::relation::attribute::AttributeTrait".to_owned(),
            cfg.owner_form.import.clone(),
            cfg.value_form.import.clone(),
        ],
        atom: formatdoc! {r#"
            impl AttributeTrait for {name} {{
                type OwnerForm = {owner_form};
                type ValueForm = {value_form};
            }}"#, name = cfg.tao_cfg.this.name,
            owner_form = cfg.owner_form.name,
            value_form = cfg.value_form.name,
        },
    }
}

/// Get the attribute test fragment.
fn attribute_test_fragment(cfg: &AttributeFormatConfig) -> AtomicFragment {
    let mut imports = vec![
        "zamm_yin::tao::archetype::ArchetypeFormTrait".to_owned(),
        "zamm_yin::tao::archetype::AttributeArchetypeFormTrait".to_owned(),
    ];
    // there's a chance the form is the same as the type, in which case it will have gotten
    // imported above already
    if cfg.owner_type.name != cfg.owner_form.name {
        imports.push(cfg.owner_type.import.clone());
    }
    // there's a chance the form is the same as the type, in which case it will have gotten
    // imported above already
    if cfg.value_type.name != cfg.value_form.name {
        imports.push(cfg.value_type.import.clone());
    }

    AtomicFragment {
        imports,
        atom: formatdoc! {r#"
            #[test]
            fn check_attribute_constraints() {{
                initialize_kb();
                assert_eq!(
                    {name}::archetype().owner_archetype(),
                    {owner_type}::archetype(){owner_into_archetype}
                );
                assert_eq!(
                    {name}::archetype().value_archetype(),
                    {value_type}::archetype(){value_into_archetype}
                );
            }}

            #[test]
            fn get_owner() {{
                initialize_kb();
                let mut instance = {name}::new();
                let owner_of_instance = {owner_type}::new();
                instance.set_owner(&owner_of_instance);
                assert_eq!(instance.owner(), Some(owner_of_instance));
                assert_eq!(instance.value(), None);
            }}
        
            #[test]
            fn get_value() {{
                initialize_kb();
                let mut instance = {name}::new();
                let value_of_instance = {value_type}::new();
                instance.set_value(&value_of_instance);
                assert_eq!(instance.owner(), None);
                assert_eq!(instance.value(), Some(value_of_instance));
            }}"#, 
            name = cfg.tao_cfg.this.name,
            owner_type = cfg.owner_type.name,
            value_type = cfg.value_type.name,
            owner_into_archetype = cfg.owner_into_archetype,
            value_into_archetype = cfg.value_into_archetype,
        },
    }
}

/// Add these flags to a file.
pub fn add_attr_fragments(cfg: &AttributeFormatConfig, file: &mut FileFragment) {
    file.append(Rc::new(RefCell::new(attribute_fragment(cfg))));
    file.append_test(Rc::new(RefCell::new(attribute_test_fragment(cfg))));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_generation() {
        let mut f = FileFragment::new();
        add_attr_fragments(
            &AttributeFormatConfig {
                tao_cfg: TaoConfig {
                    this: StructConfig {
                        name: "MyNewAttr".to_owned(),
                        ..StructConfig::default()
                    },
                    parent_name: "MyAttr".to_owned(),
                    archetype_name: "AttributeArchetype".to_owned(),
                    ..TaoConfig::default()
                },
                owner_type: StructConfig {
                    name: "MyOwner".to_owned(),
                    import: "zamm_yin::tao::MyOwner".to_owned(),
                },
                owner_form: StructConfig {
                    name: "MyOwner".to_owned(),
                    import: "zamm_yin::tao::MyOwner".to_owned(),
                },
                value_type: StructConfig {
                    name: "MyValue".to_owned(),
                    import: "zamm_yin::tao::MyValue".to_owned(),
                },
                value_form: StructConfig {
                    name: "MyValue".to_owned(),
                    import: "zamm_yin::tao::MyValue".to_owned(),
                },
                owner_into_archetype: ".as_archetype()".to_owned(),
                value_into_archetype: ".as_archetype()".to_owned(),
            },
            &mut f,
        );
        let code = f.generate_code();

        assert!(code.contains("AttributeArchetype"));
        assert!(code.contains("type OwnerForm = MyOwner"));
        assert!(code.contains("type ValueForm = MyValue"));
        assert!(code.contains("check_attribute_constraints"));
        assert!(code.contains(
            "assert_eq!(
            MyNewAttr::archetype().owner_archetype(),
            MyOwner::archetype().as_archetype()
        )"
        ));
    }
}
