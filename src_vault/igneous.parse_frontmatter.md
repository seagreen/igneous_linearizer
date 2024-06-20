---
uuid: 69f6b48a-6f93-466d-9699-8c46831b21a5
title: parse_frontmatter
---

type:: [[code]]

/// Parses frontmatter if present. Returns it and the non-frontmatter remainder of the input.
pub fn parse_frontmatter(file_content: &str) -> (HashMap<String, gray_matter::Pod>, String) {
    let matter = Matter::<gray_matter::engine::YAML>::new();
    let result = matter.parse(file_content);

    let frontmatter = match result.data {
        Some(data) => match data {
            gray_matter::Pod::Hash(map) => map,
            _ => panic!("Expected Pod::Hash but found other variant"),
        },
        None => HashMap::new(),
    };

    (frontmatter, result.content)
}

#[cfg(test)]
mod parse_frontmatter_tests {
    use super::*;

    #[test]
    fn test_frontmatter() {
        let content = r#"---
foo: bar
---
lorem ipsum

"#;
        let (frontmatter, content) = parse_frontmatter(content);
        let expected = HashMap::from([(
            "foo".to_string(),
            gray_matter::Pod::String("bar".to_string()),
        )]);
        assert_eq!(frontmatter, expected);
        assert_eq!(content, "lorem ipsum\n");
    }
}