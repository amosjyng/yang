use super::parse_yaml;
use crate::codegen::{add_indent, count_indent};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};
use zamm_yin::concepts::Tao;

/// Extracts and concatenates YAML code blocks from the markdown.
fn extract_yaml(markdown: &str) -> String {
    let mut code = String::new();
    let mut in_yaml_block = false;
    let mut in_documentation_block = false;
    let mut doc_indent = 0;
    for event in Parser::new(markdown) {
        match event {
            Event::Start(tag) => match tag {
                Tag::CodeBlock(kind) => {
                    if let CodeBlockKind::Fenced(cow) = kind {
                        match cow.to_string().as_str() {
                            "yaml" => in_yaml_block = true,
                            "yml" => in_yaml_block = true,
                            _ => (),
                        }
                    }
                }
                start => {
                    dbg!(start);
                }
            },
            Event::Text(content) => {
                dbg!(&content);
                if in_yaml_block {
                    let trimmed = content.trim_end();
                    if trimmed.ends_with("|-") {
                        in_documentation_block = true;
                        let (existing_indent, _) =
                            count_indent(trimmed.split('\n').last().unwrap());
                        dbg!(existing_indent);
                        doc_indent = existing_indent + 2; // +2 for YAML quote indent
                    }
                    code += &content.into_string();
                } else if in_documentation_block {
                    code += &add_indent(doc_indent, &content);
                    code += "\n";
                }
            }
            Event::End(tag) => match tag {
                Tag::CodeBlock(_) => in_yaml_block = false,
                Tag::BlockQuote => in_documentation_block = false,
                Tag::Paragraph => {
                    if in_documentation_block {
                        code += "\n";
                    }
                }
                end => {
                    dbg!(end);
                }
            },
            event => {
                dbg!(event);
            }
        }
    }
    if code.ends_with("\n\n") {
        // happens if input code already contains trailing newline
        code.pop();
    }
    code
}

/// Extract YAML code blocks from Markdown and then extract concepts from those YAML code blocks.
pub fn parse_md(markdown: &str) -> Vec<Tao> {
    parse_yaml(&extract_yaml(markdown))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::{initialize_kb, Implement};
    use indoc::indoc;
    use std::rc::Rc;
    use zamm_yin::concepts::{ArchetypeTrait, FormTrait};
    use zamm_yin::node_wrappers::CommonNodeTrait;

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

    #[test]
    fn test_yaml_extraction_multiline_string() {
        assert_eq!(
            extract_yaml(indoc! {r#"
            # O rly?

            ```yml
            - name: Yang
              documentation: |-
                Like, here's one line.

                And now here's another.
            ```
        "#}),
            indoc! {"
            - name: Yang
              documentation: |-
                Like, here's one line.

                And now here's another.
        "}
        );
    }

    #[test]
    fn test_yaml_extraction_multiline_string_as_quote() {
        assert_eq!(
            extract_yaml(indoc! {r#"
            # O rly?

            ```yml
            - name: Yang
              documentation: |-
            ```

            > Like, here's one line.
            > Part of the same line.
            >
            > And now here's another.
        "#}),
            indoc! {"
            - name: Yang
              documentation: |-
                Like, here's one line.
                Part of the same line.

                And now here's another.
        "}
        );
    }

    #[test]
    fn test_yaml_extraction_regular_text_after_quote() {
        assert_eq!(
            extract_yaml(indoc! {r#"
            # O rly?

            ```yml
            - name: Yang
              documentation: |-
            ```

            > Like, here's one line.
            > Part of the same line.
            >
            > And now here's another.

            Regular text, please ignore.
        "#}),
            indoc! {"
            - name: Yang
              documentation: |-
                Like, here's one line.
                Part of the same line.

                And now here's another.
        "}
        );
    }

    #[test]
    fn test_yaml_extraction_full() {
        initialize_kb();

        let concepts = parse_md(indoc! {r#"
            # Let's try this

            ```yaml
            - name: Target
              parent: Attribute
            ```

            Wait a second... this is just what we have in the yaml.rs test!

            ```yml
            - parent: Implement
              target: Target
              output_id: 2
              documentation: |-
                Howdy, how ya doing?

                I'm doing fine, you?
            ```
        "#});
        assert_eq!(concepts.len(), 2);
        let implement = Implement::from(concepts[1]);
        assert!(implement.has_ancestor(Implement::archetype()));
        assert_eq!(
            implement.target().map(|t| t.internal_name()).flatten(),
            Some(Rc::new("Target".to_owned()))
        );
        let cfg = implement.config().unwrap();
        assert_eq!(cfg.id, 2);
        assert_eq!(
            cfg.doc,
            Some(
                indoc! {"
            Howdy, how ya doing?
            
            I'm doing fine, you?
            "}
                .trim_end()
                .to_owned()
            )
        );
    }
}
