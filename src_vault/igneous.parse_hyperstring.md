---
uuid: ca0f2ad5-25ce-456a-a720-11164d5091c7
title: parse_hyperstring
---

type:: [[code]]

pub fn parse_hyperstring(input: &str) -> [[igneous.Hyperstring]] {
    let opening_brackets = r"\[\[";
    let closing_brackets = r"\]\]";
    let pattern = format!("(({opening_brackets}|!{opening_brackets})(.+?){closing_brackets})");
    let re = Regex::new(&pattern).unwrap();

    let mut segments: Vec<[[igneous.HyperstringSegment]]> = Vec::new();
    let mut last_end = 0;

    // Need to do this instead of having `Link` and `Text` named matches,
    // because if we make the latter `.+` is will gobble everything up.
    // If we make it `.+?` it outputs `Text("f"), Text("o"), Text("o")`` etc.
    for cap in re.captures_iter(input) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();
        let text = &input[last_end..start];
        if !text.is_empty() {
            segments.push([[igneous.HyperstringSegment]]::Text(text.to_string()));
        }
        if cap[0].starts_with("!\u{005B}\u{005B}") {
            segments.push([[igneous.HyperstringSegment]]::Transclusion(cap[3].to_string()));
        } else {
            segments.push([[igneous.HyperstringSegment]]::Link(cap[3].to_string()));
        }
        last_end = end;
    }

    if last_end < input.len() {
        segments.push([[igneous.HyperstringSegment]]::Text(input[last_end..].to_string()));
    }

    Hyperstring(segments)
}

#[cfg(test)]
mod parse_hyperstring_tests {
    use super::*;

    const OPEN: &str = "\u{005B}\u{005B}"; // Two `[`s

    #[test]
    fn test_parse_hyperstring() {
        let input = format!("foo {OPEN}bar]] baz");
        let hyperstring = parse_hyperstring(&input);
        let expected = [[igneous.Hyperstring]](vec![
            [[igneous.HyperstringSegment]]::Text("foo ".to_string()),
            [[igneous.HyperstringSegment]]::Link("bar".to_string()),
            [[igneous.HyperstringSegment]]::Text(" baz".to_string()),
        ]);
        assert_eq!(hyperstring, expected);
    }

    #[test]
    fn test_parse_transclusion_hyperstring() {
        let input = format!("foo !{OPEN}bar]] baz");
        let hyperstring = parse_hyperstring(&input);
        let expected = [[igneous.Hyperstring]](vec![
            [[igneous.HyperstringSegment]]::Text("foo ".to_string()),
            [[igneous.HyperstringSegment]]::Transclusion("bar".to_string()),
            [[igneous.HyperstringSegment]]::Text(" baz".to_string()),
        ]);
        assert_eq!(hyperstring, expected);
    }

    #[test]
    fn test_parse_hyperstring_trailing() {
        let input = format!("foo {OPEN}bar]] baz]]");
        let hyperstring = parse_hyperstring(&input);
        let expected = [[igneous.Hyperstring]](vec![
            [[igneous.HyperstringSegment]]::Text("foo ".to_string()),
            [[igneous.HyperstringSegment]]::Link("bar".to_string()),
            [[igneous.HyperstringSegment]]::Text(" baz]]".to_string()),
        ]);
        assert_eq!(hyperstring, expected);
    }

    // Consider forbidding this.
    #[test]
    fn test_parse_hyperstring_interior() {
        let input = format!("foo {OPEN}ba]r]] baz");
        let hyperstring = parse_hyperstring(&input);
        let expected = [[igneous.Hyperstring]](vec![
            [[igneous.HyperstringSegment]]::Text("foo ".to_string()),
            [[igneous.HyperstringSegment]]::Link("ba]r".to_string()),
            [[igneous.HyperstringSegment]]::Text(" baz".to_string()),
        ]);
        assert_eq!(hyperstring, expected);
    }
}
