use super::CODE_WIDTH;

/// Break up a one line documentation string into a multi-line docstring.
pub fn into_docstring(documentation: &str, indent_size: usize) -> String {
    let indent = " ".repeat(indent_size);
    // subtract 4 more from CODE_WIDTH to account for "/// " at the beginning of each line
    let lines = textwrap::fill(documentation, CODE_WIDTH - indent_size - 4);
    let mut comment = String::new();
    for line in lines.split('\n') {
        if line.is_empty() {
            comment.push_str("///\n"); // no space after triple slash
        } else {
            comment.push_str(format!("{}/// {}\n", indent, line.trim_end()).as_str());
        }
    }
    comment.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_docstring() {
        assert_eq!(
            into_docstring("A short description.", 0),
            "/// A short description."
        );
    }

    #[test]
    fn test_short_docstring_indent() {
        assert_eq!(
            into_docstring("A short description.", 4),
            "    /// A short description."
        );
    }

    #[test]
    fn test_short_docstring_trim() {
        assert_eq!(
            into_docstring("A short description.\t", 4),
            "    /// A short description."
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
                0
            ),
            r#"/// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec volutpat
/// malesuada ex, maximus porttitor arcu consectetur sit amet. Aliquam erat
/// volutpat. Ut quis maximus erat. Curabitur a velit convallis, suscipit lectus
/// non, interdum ex. In hac habitasse platea dictumst."#
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
                4
            ),
            r#"    /// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec volutpat
    /// malesuada ex, maximus porttitor arcu consectetur sit amet. Aliquam erat
    /// volutpat. Ut quis maximus erat. Curabitur a velit convallis, suscipit
    /// lectus non, interdum ex. In hac habitasse platea dictumst."#
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
                8
            ),
            r#"        /// Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec
        /// volutpat malesuada ex, maximus porttitor arcu consectetur sit amet.
        /// Aliquam erat volutpat. Ut quis maximus erat. Curabitur a velit
        /// convallis, suscipit lectus non, interdum ex. In hac habitasse platea
        /// dictumst."#
        );
    }

    #[test]
    fn test_docstring_newline() {
        assert_eq!(
            into_docstring("A docstring\n\nWith newlines", 0),
            r#"/// A docstring
///
/// With newlines"#
        );
    }
}
