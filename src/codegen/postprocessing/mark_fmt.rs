/// Marker for getting rustfmt to skip over this line.
pub const FMT_SKIP_MARKER: &str = "#![rustfmt::skip]\n#![allow(unused_attributes)]";

/// Add any relevant rustfmt skip markers for autogenerated code.
pub fn add_fmt_skips(code: &str) -> String {
    if code.trim().is_empty() {
        code.to_string()
    } else {
        format!("{}\n\n{}", FMT_SKIP_MARKER, code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_mark_fmt_empty_str() {
        assert_eq!(add_fmt_skips(""), "");
    }

    #[test]
    fn test_mark_fmt_newline_str() {
        assert_eq!(add_fmt_skips("\n"), "\n");
    }

    #[test]
    fn test_mark_fmt_short_statement() {
        assert_eq!(
            add_fmt_skips("Short line."),
            indoc! {"
            #![rustfmt::skip]
            #![allow(unused_attributes)]
            
            Short line."}
        );
    }

    #[test]
    fn test_mark_fmt_long_statement() {
        assert_eq!(
            add_fmt_skips(indoc! {"
            A really long statement, look here. Is it just me, or is it getting crazier out there?
            "}),
            indoc! {"
            #![rustfmt::skip]
            #![allow(unused_attributes)]

            A really long statement, look here. Is it just me, or is it getting crazier out there?
            "}
        );
    }

    #[test]
    fn test_mark_fmt_mixed_statements() {
        assert_eq!(
            add_fmt_skips(indoc! {"
            Shorty.
            A really long statement, look here. Is it just me, or is it getting crazier out there?

            Uhuh. Wow.
            "}),
            indoc! {"
            #![rustfmt::skip]
            #![allow(unused_attributes)]

            Shorty.
            A really long statement, look here. Is it just me, or is it getting crazier out there?

            Uhuh. Wow.
            "}
        );
    }

    #[test]
    fn test_mark_fmt_indent() {
        assert_eq!(
            add_fmt_skips(indoc! {"
            Shorty {
                A really long statement, look here. Is it just me, or is it getting crazier out there?
            }
            "}),
            indoc! {"
            #![rustfmt::skip]
            #![allow(unused_attributes)]

            Shorty {
                A really long statement, look here. Is it just me, or is it getting crazier out there?
            }
            "}
        );
    }
}
