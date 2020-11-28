fn common_docstring(documentation: &str, line_width: usize, comment_marker: &str) -> String {
    assert!(line_width > 0);
    // subtract 1 more from CODE_WIDTH to account for the space after the comment_marker at the
    // beginning of every line
    let lines = textwrap::fill(documentation, line_width - comment_marker.len() - 1);
    let mut comment = String::new();
    for line in lines.split('\n') {
        if line.is_empty() {
            comment.push_str(format!("{}\n", comment_marker).as_str()); // no space after triple slash
        } else {
            comment.push_str(format!("{} {}\n", comment_marker, line.trim_end()).as_str());
        }
    }
    comment.trim_end().to_string()
}

/// Break up a one line documentation string into a multi-line docstring.
pub fn into_docstring(documentation: &str, line_width: usize) -> String {
    common_docstring(documentation, line_width, "///")
}

/// Break up a one line documentation string into a multi-line docstring that applies to the parent
/// element.
pub fn into_parent_docstring(documentation: &str, indent_size: usize) -> String {
    common_docstring(documentation, indent_size, "//!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_short_docstring() {
        assert_eq!(
            into_docstring("A short description.", 80),
            "/// A short description."
        );
    }

    #[test]
    fn test_short_docstring_trim() {
        assert_eq!(
            into_docstring("A short description.\t", 76),
            "/// A short description."
        );
    }

    #[test]
    fn test_long_docstring() {
        assert_eq!(
            into_docstring(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec volutpat \
                malesuada ex, maximus porttitor arcu consectetur sit amet. Aliquam erat volutpat. \
                Ut quis maximus erat. Curabitur a velit convallis, suscipit lectus non, interdum \
                ex. In hac habitasse platea dictumst.",
                80
            ),
            indoc! {r#"
                /// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec volutpat
                /// malesuada ex, maximus porttitor arcu consectetur sit amet. Aliquam erat
                /// volutpat. Ut quis maximus erat. Curabitur a velit convallis, suscipit lectus
                /// non, interdum ex. In hac habitasse platea dictumst."#}
        );
    }

    #[test]
    fn test_long_docstring_indent() {
        assert_eq!(
            into_docstring(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec volutpat \
                malesuada ex, maximus porttitor arcu consectetur sit amet. Aliquam erat volutpat. \
                Ut quis maximus erat. Curabitur a velit convallis, suscipit lectus non, interdum \
                ex. In hac habitasse platea dictumst.",
                76
            ),
            indoc! {r#"
                /// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec volutpat
                /// malesuada ex, maximus porttitor arcu consectetur sit amet. Aliquam erat
                /// volutpat. Ut quis maximus erat. Curabitur a velit convallis, suscipit
                /// lectus non, interdum ex. In hac habitasse platea dictumst."#}
        );
    }

    #[test]
    fn test_long_docstring_indent_more() {
        assert_eq!(
            into_docstring(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec volutpat \
                malesuada ex, maximus porttitor arcu consectetur sit amet. Aliquam erat volutpat. \
                Ut quis maximus erat. Curabitur a velit convallis, suscipit lectus non, interdum \
                ex. In hac habitasse platea dictumst.",
                72
            ),
            indoc! {r#"
                /// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec
                /// volutpat malesuada ex, maximus porttitor arcu consectetur sit amet.
                /// Aliquam erat volutpat. Ut quis maximus erat. Curabitur a velit
                /// convallis, suscipit lectus non, interdum ex. In hac habitasse platea
                /// dictumst."#}
        );
    }

    #[test]
    fn test_docstring_newline() {
        assert_eq!(
            into_docstring("A docstring\n\nWith newlines", 80),
            indoc! {r#"
                /// A docstring
                ///
                /// With newlines"#}
        );
    }

    #[test]
    fn test_parent_docstring() {
        assert_eq!(
            into_parent_docstring(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec volutpat \
                malesuada ex, maximus porttitor arcu consectetur sit amet. Aliquam erat volutpat. \
                Ut quis maximus erat. Curabitur a velit convallis, suscipit lectus non, interdum \
                ex. In hac habitasse platea dictumst.",
                72
            ),
            indoc! {r#"
                //! Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec
                //! volutpat malesuada ex, maximus porttitor arcu consectetur sit amet.
                //! Aliquam erat volutpat. Ut quis maximus erat. Curabitur a velit
                //! convallis, suscipit lectus non, interdum ex. In hac habitasse platea
                //! dictumst."#}
        );
    }
}
