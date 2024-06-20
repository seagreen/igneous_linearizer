---
uuid: 5b80d963-938f-418b-a0f8-e6d5b05fb165
title: parse_page
---

type:: [[code]]

/// Parse a single page consisting of YAML frontmatter, Dataview attributes, and a hyperstring.
///
/// The frontmatter and Dataview attributes are optional.
///
/// Dataview attributes looks like `key:: value`.
/// This section will eventually go away as Obsidian's frontmatter support matures.
pub fn parse_page(input: &str) -> (HashMap<String, gray_matter::Pod>, [[igneous.Hyperstring]]) {
    let (frontmatter, content_without_frontmatter) = [[igneous.parse_frontmatter]](input);
    let final_content = [[igneous.strip_dataview_block]](&content_without_frontmatter);
    let hyperstring = [[igneous.parse_hyperstring]](&final_content);
    (frontmatter, hyperstring)
}
#[cfg(test)]
mod parse_page_tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = r#"---
foo: bar
---

baz::quux

lorem ipsum

"#;
        let (frontmatter, content) = parse_page(input);
        let expected_frontmatter = HashMap::from([(
            "foo".to_string(),
            gray_matter::Pod::String("bar".to_string()),
        )]);
        let expected = [[igneous.Hyperstring]](vec![HyperstringSegment::Text("lorem ipsum\n".to_string())]);
        assert_eq!(frontmatter, expected_frontmatter);
        assert_eq!(content, expected);
    }
}