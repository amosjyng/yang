use super::CodegenConfig;
use crate::codegen::mark_autogen::add_autogeneration_comments;
pub use crate::codegen::mark_autogen::{add_indent, count_indent};
pub use crate::codegen::mark_fmt::add_fmt_skips;
pub use crate::codegen::name_transform::NameTransform;
use crate::codegen::string_format::attribute::code_attribute;
use crate::codegen::string_format::code_string_concept;
use crate::codegen::string_format::tao::code_tao;
use crate::codegen::string_format::FormatConfig;
use crate::concepts::ImplementConfig;

/// Do post-processing on generated code. Includes marking lines with autogeneration comments, or
/// marking lines as requiring formatting skips.
fn post_process_generation(code: &str, options: &CodegenConfig) -> String {
    let formatted = add_fmt_skips(&code);
    if options.comment_autogen && !options.release {
        add_autogeneration_comments(&formatted)
    } else {
        formatted
    }
}

/// Generate the final version of code, to be output to a file as-is.
pub fn code(implement: &ImplementConfig, options: &CodegenConfig) -> String {
    let format_cfg = FormatConfig::from_cfgs(implement, options);
    let initial_code = if implement.parent_name() == "Attribute" {
        code_attribute(&format_cfg)
    } else if implement.parent_name() == "Data" {
        code_string_concept(&format_cfg)
    } else {
        code_tao(&format_cfg)
    };
    post_process_generation(&initial_code, options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::mark_autogen::AUTOGENERATION_MARKER;
    use crate::codegen::mark_fmt::FMT_SKIP_MARKER;

    #[test]
    fn test_post_process_comments() {
        let codegen_cfg = CodegenConfig::default();
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig::default(),
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(result.contains(AUTOGENERATION_MARKER));
        assert!(result.contains("YIN_MAX_ID"));
    }

    #[test]
    fn test_post_process_no_comments() {
        let codegen_cfg = CodegenConfig {
            comment_autogen: false,
            ..CodegenConfig::default()
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig::default(),
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(!result.contains(AUTOGENERATION_MARKER));
    }

    #[test]
    fn test_post_process_yin() {
        let codegen_cfg = CodegenConfig {
            yin: true,
            ..CodegenConfig::default()
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig::default(),
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(!result.contains("YIN_MAX_ID"));
    }

    #[test]
    fn test_post_process_fmt_not_skip() {
        let codegen_cfg = CodegenConfig::default();
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "short".to_owned(),
                ..ImplementConfig::default()
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(!result.contains(FMT_SKIP_MARKER));
    }

    #[test]
    fn test_post_process_fmt_skip() {
        let codegen_cfg = CodegenConfig {
            release: false,
            ..CodegenConfig::default()
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "ReallySuperLongClassNameOhBoy".to_owned(),
                ..ImplementConfig::default()
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(result.contains(FMT_SKIP_MARKER));
    }

    #[test]
    fn test_post_process_fmt_skip_release() {
        let codegen_cfg = CodegenConfig {
            release: true,
            ..CodegenConfig::default()
        };
        let code = code_attribute(&FormatConfig::from_cfgs(
            &ImplementConfig {
                name: "ReallySuperLongClassNameOhBoy".to_owned(),
                ..ImplementConfig::default()
            },
            &codegen_cfg,
        ));
        let result = post_process_generation(&code, &codegen_cfg);
        assert!(result.contains(FMT_SKIP_MARKER));
    }
}
