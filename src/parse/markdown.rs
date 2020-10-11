use super::parse_yaml;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use zamm_yin::concepts::Tao;

/// Extracts and concatenates YAML code blocks from the markdown.
fn extract_yaml(markdown: &str) -> String {
    let mut code = String::new();
    let mut in_yaml_block = false;
    for event in Parser::new(markdown) {
        match event {
            Event::Start(tag) => match tag {
                Tag::CodeBlock(kind) => match kind {
                    CodeBlockKind::Fenced(cow) if cow.to_string().as_str() == "yaml" => {
                        in_yaml_block = true
                    }
                    CodeBlockKind::Fenced(cow) if cow.to_string().as_str() == "yml" => {
                        in_yaml_block = true
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::Text(content) => {
                if in_yaml_block {
                    code += &content.into_string()
                }
            }
            Event::End(tag) => match tag {
                Tag::CodeBlock(_) => in_yaml_block = false,
                _ => (),
            },
            other => {
                if in_yaml_block {
                    dbg!("{}", other);
                }
            }
        }
    }
    code
}

/// Extract YAML code blocks from Markdown and then extract concepts from those YAML code blocks.
pub fn parse_markdown(markdown: &str) -> Vec<Tao> {
    parse_yaml(&extract_yaml(markdown))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_yaml_extraction_nothing() {
        assert_eq!(
            extract_yaml(indoc! {"
            # Some document

            No code in here.
        "}),
            "".to_owned()
        );
    }

    #[test]
    fn test_yaml_extraction_one_block() {
        assert_eq!(
            extract_yaml(indoc! {"
            # Some document

            ```yaml
            - name: Yin
            ```

            Aha! We have some code.
        "}),
            indoc! {"
            - name: Yin
        "}
        );
    }

    #[test]
    fn test_yaml_extraction_multiple_blocks() {
        assert_eq!(
            extract_yaml(indoc! {r#"
            # Some document

            ```yaml
            - name: Yin
            ```

            Aha! We have some code. More?

            ## Yes more

            ```json
            {"very": "devious"}
            ```

            Will it skip that?

            ```
            And this too?
            ```

            ```yml
            - name: Yang
              parent: Tao
            ```
        "#}),
            indoc! {"
            - name: Yin
            - name: Yang
              parent: Tao
        "}
        );
    }
}
