/// Mark files as autogenerated.
pub mod mark_autogen;
/// Mark lines as not needing formatting.
///
/// Sometimes, it's just too hard to autogenerate perfectly formatted code.
pub mod mark_fmt;

use super::CodegenConfig;
use mark_autogen::add_autogeneration_comments;
use mark_fmt::add_fmt_skips;

/// Do post-processing on generated code. Includes marking lines with autogeneration comments, or
/// marking lines as requiring formatting skips.
pub fn post_process_generation(code: &str, options: &CodegenConfig) -> String {
    if options.release {
        return code.to_owned(); // no post-processing for releases
    }

    let formatted = if options.comment_autogen && options.add_rustfmt_attributes {
        add_fmt_skips(&code)
    } else {
        code.to_owned()
    };
    if options.comment_autogen {
        add_autogeneration_comments(&formatted)
    } else {
        formatted
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::template::concept::tao::{tao_file_fragment, TaoConfig};
    use crate::codegen::StructConfig;
    use mark_autogen::AUTOGENERATION_MARKER;
    use mark_fmt::FMT_SKIP_MARKER;

    fn code_form(cfg: &TaoConfig) -> String {
        let f = tao_file_fragment(cfg);
        f.generate_code()
    }

    #[test]
    fn test_post_process_comments() {
        let code = code_form(&TaoConfig::default());
        let result = post_process_generation(&code, &CodegenConfig::default());
        assert!(result.contains(AUTOGENERATION_MARKER));
        assert!(result.contains("YIN_MAX_ID"));
    }

    #[test]
    fn test_post_process_no_comments() {
        let code = code_form(&TaoConfig::default());
        let result = post_process_generation(
            &code,
            &CodegenConfig {
                comment_autogen: false,
                ..CodegenConfig::default()
            },
        );
        assert!(!result.contains(AUTOGENERATION_MARKER));
    }

    #[test]
    fn test_post_process_fmt_always_skip() {
        let code = code_form(&TaoConfig {
            this: StructConfig {
                name: "S".to_owned(), // short
                ..StructConfig::default()
            },
            ..TaoConfig::default()
        });
        let result = post_process_generation(&code, &CodegenConfig::default());
        assert!(result.contains(FMT_SKIP_MARKER));
    }

    #[test]
    fn test_post_process_fmt_skip() {
        let code = code_form(&TaoConfig {
            this: StructConfig {
                name: "ReallySuperLongClassNameOhBoy".to_owned(),
                ..StructConfig::default()
            },
            ..TaoConfig::default()
        });
        let result = post_process_generation(
            &code,
            &CodegenConfig {
                release: false,
                ..CodegenConfig::default()
            },
        );
        assert!(result.contains(FMT_SKIP_MARKER));
    }

    #[test]
    fn test_post_process_fmt_skip_release() {
        let code = code_form(&TaoConfig {
            this: StructConfig {
                name: "ReallySuperLongClassNameOhBoy".to_owned(),
                ..StructConfig::default()
            },
            ..TaoConfig::default()
        });
        let result = post_process_generation(
            &code,
            &CodegenConfig {
                release: true,
                ..CodegenConfig::default()
            },
        );
        assert!(!result.contains(FMT_SKIP_MARKER));
    }
}
