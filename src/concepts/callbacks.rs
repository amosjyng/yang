use super::Implement;
use crate::codegen::filesystem::{output_code, OutputConfig};
use crate::codegen::CodegenConfig;
use crate::codegen::NameTransform;
use crate::codegen::{code, CodeConfig};
use zamm_yin::concepts::archetype::ArchetypeFormTrait;
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
pub fn handle_implementation(request: Implement, codegen_cfg: &CodegenConfig) {
    let target = request.target().unwrap();
    let target_name = target.internal_name().unwrap();
    let parent_name = (*target
        .ancestry()
        .iter()
        .last()
        .unwrap()
        .internal_name()
        .unwrap())
    .clone();
    let code = code(&CodeConfig {
        name: target_name.as_str(),
        parent_name: parent_name.as_str(),
        impl_cfg: request.config().unwrap(),
        codegen_cfg: *codegen_cfg,
    });

    let file_path = file_path(target);
    output_code(&OutputConfig {
        code: &code,
        file_path: &file_path,
        git_ignore: !codegen_cfg.release,
        cargo_track: codegen_cfg.track_autogen,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::initialize_kb;
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
        assert!(code(&CodeConfig {
            parent_name: "Attribute",
            ..CodeConfig::default()
        })
        .contains("Attribute"));
    }

    #[test]
    fn integration_test_non_attribute_generation() {
        assert!(!code(&CodeConfig {
            parent_name: "Tao",
            ..CodeConfig::default()
        })
        .contains("Attribute"));
    }
}
