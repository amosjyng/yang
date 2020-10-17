use super::Implement;
use crate::codegen::code;
use crate::codegen::filesystem::{output_code, OutputConfig};
use crate::codegen::CodegenConfig;
use crate::codegen::NameTransform;
use zamm_yin::concepts::{Archetype, FormTrait};
use zamm_yin::node_wrappers::CommonNodeTrait;

fn file_path(target: Archetype) -> String {
    let mut folder = "src".to_owned();
    for ancestor in target.ancestry() {
        let ancestor_name = ancestor.internal_name().unwrap();
        folder += "/";
        // this means that paths will now feature singular instead of plural nouns
        folder += &NameTransform::from(ancestor_name.as_str()).to_snake_case();
    }

    let snake_name = NameTransform::from(target.internal_name().unwrap().as_str()).to_snake_case();
    // todo: use children() instead of individuals(), and filter by type, once Yin has that
    // functionality
    let own_submodule = target
        .individuals()
        .iter()
        .any(|i| *i != target.ego_death()); // todo: remove once Yin bug fixed
    if own_submodule {
        folder += "/";
        folder += &snake_name;
    }
    // append _archetype to filename to avoid
    // https://rust-lang.github.io/rust-clippy/master/index.html#module_inception
    format!("{}/{}_archetype.rs", folder, snake_name).to_ascii_lowercase()
}

/// Handle the implementation request for a new attribute archetype.
pub fn handle_implementation(request: Implement, cfg: &CodegenConfig) {
    let impl_cfg = request.config().unwrap();
    let code = code(&impl_cfg, cfg);
    let file_path = file_path(request.target().unwrap());
    output_code(&OutputConfig {
        code: &code,
        file_path: &file_path,
        git_ignore: !cfg.release,
        cargo_track: cfg.track_autogen,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::{initialize_kb, ImplementConfig};
    use zamm_yin::concepts::attributes::{Attribute, Owner};
    use zamm_yin::concepts::{ArchetypeTrait, Tao};

    #[test]
    fn folder_path_tao() {
        initialize_kb();
        assert_eq!(file_path(Tao::archetype()), "src/tao/tao_archetype.rs");
    }

    #[test]
    fn folder_path_attributes() {
        initialize_kb();
        assert_eq!(
            file_path(Attribute::archetype()),
            "src/tao/attribute/attribute_archetype.rs"
        );
    }

    #[test]
    fn folder_path_nested() {
        initialize_kb();
        assert_eq!(
            file_path(Owner::archetype()),
            "src/tao/attribute/owner_archetype.rs"
        );
    }

    #[test]
    fn integration_test_attribute_generation() {
        assert!(code(
            &ImplementConfig {
                parent_name: "Attribute".to_owned(), // relevant for test
                ..ImplementConfig::default()
            },
            &CodegenConfig::default()
        )
        .contains("Attribute"));
    }

    #[test]
    fn integration_test_non_attribute_generation() {
        assert!(!code(
            &ImplementConfig {
                parent_name: "Tao".to_owned(), // relevant for test
                ..ImplementConfig::default()
            },
            &CodegenConfig::default()
        )
        .contains("Attribute"));
    }
}
