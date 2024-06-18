---
uuid: 405ad342-b428-4d8e-a8d8-42a28859d899
title: strip_dataview_block
---

type:: [[code]]

pub fn strip_dataview_block(input: &str) -> String {
    let pattern = r"^\n*([^\s:]+::.*\n)+\n*";
    let re = Regex::new(pattern).unwrap();

    re.replace(input, "").to_string()
}

#[cfg(test)]
mod strip_dataview_block_tests {
    use super::*;

    #[test]
    fn test_dataview() {
        let input = r#"

foo::bar

lorem ipsum
"#;
        let expected = "lorem ipsum\n";
        assert_eq!(strip_dataview_block(input), expected);

        let input_with_no_match = "\n\nfoo\nbar\n";
        assert_eq!(
            strip_dataview_block(input_with_no_match),
            input_with_no_match
        );
    }
}