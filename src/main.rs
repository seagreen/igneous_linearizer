use clap::Parser;
use gray_matter::Matter;
use petgraph::graph::DiGraph;
use petgraph::Graph;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name = "ROOT_FILE_PATH")]
    root_file: PathBuf,

    #[arg(
        long,
        help = concat!(
            "Only transpile ROOT_FILE_PATH, don't crawl its dependencies and include them in the output as well.",
            " Useful for compiling import blocks that must be at the start of a file,",
            " e.g. `igneous-linearizer --single import.md > main.rs; igneous-linearierizer main.md >> main.rs"
        )
    )]
    single: bool,
}

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

/// Strips a block of Dataview attributes if present. Returns the non-Dataview remainder of the input.
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

pub fn parse_hyperstring(input: &str) -> Hyperstring {
    let opening_brackets = r"\[\[";
    let closing_brackets = r"\]\]";
    let pattern = format!("(({opening_brackets}|!{opening_brackets})(.+?){closing_brackets})");
    let re = Regex::new(&pattern).unwrap();

    let mut segments: Vec<HyperstringSegment> = Vec::new();
    let mut last_end = 0;

    // Need to do this instead of having `Link` and `Text` named matches,
    // because if we make the latter `.+` is will gobble everything up.
    // If we make it `.+?` it outputs `Text("f"), Text("o"), Text("o")`` etc.
    for cap in re.captures_iter(input) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();
        let text = &input[last_end..start];
        if !text.is_empty() {
            segments.push(HyperstringSegment::Text(text.to_string()));
        }
        if cap[0].starts_with("!\u{005B}\u{005B}") {
            segments.push(HyperstringSegment::Transclusion(cap[3].to_string()));
        } else {
            segments.push(HyperstringSegment::Link(cap[3].to_string()));
        }
        last_end = end;
    }

    if last_end < input.len() {
        segments.push(HyperstringSegment::Text(input[last_end..].to_string()));
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
        let expected = Hyperstring(vec![
            HyperstringSegment::Text("foo ".to_string()),
            HyperstringSegment::Link("bar".to_string()),
            HyperstringSegment::Text(" baz".to_string()),
        ]);
        assert_eq!(hyperstring, expected);
    }

    #[test]
    fn test_parse_transclusion_hyperstring() {
        let input = format!("foo !{OPEN}bar]] baz");
        let hyperstring = parse_hyperstring(&input);
        let expected = Hyperstring(vec![
            HyperstringSegment::Text("foo ".to_string()),
            HyperstringSegment::Transclusion("bar".to_string()),
            HyperstringSegment::Text(" baz".to_string()),
        ]);
        assert_eq!(hyperstring, expected);
    }

    #[test]
    fn test_parse_hyperstring_trailing() {
        let input = format!("foo {OPEN}bar]] baz]]");
        let hyperstring = parse_hyperstring(&input);
        let expected = Hyperstring(vec![
            HyperstringSegment::Text("foo ".to_string()),
            HyperstringSegment::Link("bar".to_string()),
            HyperstringSegment::Text(" baz]]".to_string()),
        ]);
        assert_eq!(hyperstring, expected);
    }

    // Consider forbidding this.
    #[test]
    fn test_parse_hyperstring_interior() {
        let input = format!("foo {OPEN}ba]r]] baz");
        let hyperstring = parse_hyperstring(&input);
        let expected = Hyperstring(vec![
            HyperstringSegment::Text("foo ".to_string()),
            HyperstringSegment::Link("ba]r".to_string()),
            HyperstringSegment::Text(" baz".to_string()),
        ]);
        assert_eq!(hyperstring, expected);
    }
}

/// Parse a single page, consisting of YAML frontmatter, Dataview attributes, and a hyperstring.
///
/// The frontmatter and Dataview attributes are optional.
///
/// Dataview attributes looks like `key:: value`.
/// This section will eventually go away as Obsidian's frontmatter support matures.
pub fn parse_page(input: &str) -> (HashMap<String, gray_matter::Pod>, Hyperstring) {
    let (frontmatter, content_without_frontmatter) = parse_frontmatter(input);
    let final_content = strip_dataview_block(&content_without_frontmatter);
    let hyperstring = parse_hyperstring(&final_content);
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
        let expected = Hyperstring(vec![HyperstringSegment::Text("lorem ipsum\n".to_string())]);
        assert_eq!(frontmatter, expected_frontmatter);
        assert_eq!(content, expected);
    }
}

impl Hyperstring {
    pub fn references(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|segment| match segment {
                HyperstringSegment::Link(link) => Some(link.clone()),
                HyperstringSegment::Transclusion(transclusion) => Some(transclusion.clone()),
                _ => None,
            })
            .collect()
    }
}

/// Reads in all files transitively referenced by `root_base_name`.
///
/// This is the last input done by the program, after this is processing and output.
pub fn crawl(vault_dir: &Path, root_base_name: &str) -> HashMap<String, Page> {
    let mut visited: HashMap<String, Page> = HashMap::new();
    let mut to_visit: Vec<String> = vec![root_base_name.to_string()];

    while let Some(base_name) = to_visit.pop() {
        if visited.contains_key(&base_name) {
            continue;
        }
        let full_path = vault_dir.join(format!("{}.md", base_name));
        let file_content = fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Attempted to read the file {}", full_path.display()));
        let (metadata, hyperstring) = parse_page(&file_content);

        for reference in hyperstring.references() {
            to_visit.push(reference);
        }
        visited.insert(
            base_name,
            Page {
                metadata,
                hyperstring: hyperstring.clone(),
            },
        );
    }

    visited
}

impl Hyperstring {
    pub fn transclusions(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|segment| {
                if let HyperstringSegment::Transclusion(transclusion) = segment {
                    Some(transclusion.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn cyclic_transclusion_check(page_map: &HashMap<String, Page>) -> Option<String> {
    let mut node_map: HashMap<String, _> = HashMap::new();
    let mut graph: Graph<&str, ()> = DiGraph::new();

    for base_name in page_map.keys() {
        let node = graph.add_node(base_name);
        node_map.insert(base_name.clone(), node);
    }

    for (base_name, page) in page_map {
        let node = node_map.get(base_name).unwrap();
        for transclusion in &page.hyperstring.transclusions() {
            let target_node = node_map.get(transclusion).unwrap();
            graph.add_edge(*node, *target_node, ());
        }
    }

    match petgraph::algo::toposort(&graph, None) {
        Ok(_) => None,
        Err(cycle) => Some(graph.node_weight(cycle.node_id()).unwrap().to_string()),
    }
}

#[cfg(test)]
mod cyclic_transclusion_check_tests {
    use super::*;

    #[test]
    fn test_cycle() {
        let mut page_map = HashMap::new();
        page_map.insert(
            "page-1".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: Hyperstring(vec![HyperstringSegment::Transclusion(
                    "page-2".to_string(),
                )]),
            },
        );
        page_map.insert(
            "page-2".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: Hyperstring(vec![HyperstringSegment::Transclusion(
                    "page-3".to_string(),
                )]),
            },
        );
        page_map.insert(
            "page-3".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: Hyperstring(vec![HyperstringSegment::Transclusion(
                    "page-1".to_string(),
                )]),
            },
        );
        let result = cyclic_transclusion_check(&page_map);
        assert!(
            matches!(result, Some(page) if page == "page-1" || page == "page-2" || page == "page-3")
        );
    }

    #[test]
    fn test_loop() {
        let mut page_map = HashMap::new();
        page_map.insert(
            "page-1".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: Hyperstring(vec![HyperstringSegment::Transclusion(
                    "page-1".to_string(),
                )]),
            },
        );

        let result = cyclic_transclusion_check(&page_map);
        assert_eq!(result, Some("page-1".to_string()));
    }

    #[test]
    fn test_no_cycle() {
        let mut page_map = HashMap::new();
        page_map.insert(
            "page-1".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: Hyperstring(vec![HyperstringSegment::Transclusion(
                    "page-2".to_string(),
                )]),
            },
        );
        page_map.insert(
            "page-2".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: Hyperstring(vec![HyperstringSegment::Transclusion(
                    "page-3".to_string(),
                )]),
            },
        );
        page_map.insert(
            "page-3".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: Hyperstring(vec![]),
            },
        );
        assert_eq!(cyclic_transclusion_check(&page_map), None);
    }
}

impl fmt::Display for Hyperstring {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for segment in &self.0 {
            write!(f, "{}", segment)?;
        }
        Ok(())
    }
}

impl fmt::Display for HyperstringSegment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HyperstringSegment::Text(text) => write!(f, "{}", text),
            HyperstringSegment::Link(link) => write!(f, "\u{005B}\u{005B}{}]]", link),
            HyperstringSegment::Transclusion(transclusion) => {
                write!(f, "!\u{005B}\u{005B}{}]]", transclusion)
            }
        }
    }
}

// Also grab the Igneous Linearizer snippets

impl Hyperstring {
    pub fn links(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|segment| {
                if let HyperstringSegment::Link(link) = segment {
                    Some(link.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn linearize(page_map: &HashMap<String, Page>, root_base_name: &str) -> Vec<Hyperstring> {
    let mut result = Vec::new();
    let mut seen_set: HashSet<String> = HashSet::new();
    let mut stack = vec![root_base_name.to_string()];

    while let Some(base_name) = stack.pop() {
        let page = page_map.get(&base_name).unwrap();
        result.push(page.hyperstring.clone());
        for link in page.hyperstring.links() {
            if !seen_set.contains(&link) {
                stack.push(link.clone());
                seen_set.insert(link.clone());
            }
        }
    }

    result
}

pub fn transform(page_map: &HashMap<String, Page>, root_base_name: &str) -> Vec<Hyperstring> {
    if let Some(cycle) = cyclic_transclusion_check(page_map) {
        panic!("Cyclic transclusion detected: {:?}", cycle);
    }
    linearize(page_map, root_base_name)
}

pub fn link_title_or_name(page_map: &HashMap<String, Page>, base_name: &str) -> String {
    match page_map.get(base_name) {
        Some(page) => match page.metadata.get("title") {
            Some(gray_matter::Pod::String(title)) => title.clone(),
            Some(other) => panic!(
                "Expected the title value of {} to be a string, got: {:?}",
                base_name, other
            ),
            None => base_name.to_string(),
        },
        None => base_name.to_string(),
    }
}

impl Hyperstring {
    pub fn to_plaintext(&self, page_map: &HashMap<String, Page>) -> String {
        self.0
            .iter()
            .map(|segment| match segment {
                HyperstringSegment::Text(text) => text.clone(),
                HyperstringSegment::Link(link) => link_title_or_name(page_map, link),
                HyperstringSegment::Transclusion(transclusion) => {
                    let transclusion_page = page_map.get(transclusion).unwrap();
                    transclusion_page.hyperstring.to_plaintext(page_map)
                }
            })
            .collect::<Vec<_>>()
            .concat()
    }
}

pub struct Page {
    pub metadata: HashMap<String, gray_matter::Pod>,
    pub hyperstring: Hyperstring,
}

#[derive(PartialEq, Debug, Clone)]
pub enum HyperstringSegment {
    Text(String),
    Link(String),
    Transclusion(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Hyperstring(pub Vec<HyperstringSegment>);

pub fn hyperstrings_to_string(
    page_map: HashMap<String, Page>,
    hyperstrings: Vec<Hyperstring>,
) -> String {
    hyperstrings
        .into_iter()
        .rev()
        .map(|hs| hs.to_plaintext(&page_map))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn main() {
    let args = Args::parse();

    let vault_dir = args.root_file.parent().unwrap();
    let root_file_name = args.root_file.file_name().unwrap().to_str().unwrap();
    let root_base_name = root_file_name
        .strip_suffix(".md")
        .unwrap_or("Initial file must start with .md");

    let page_map = crawl(vault_dir, root_base_name);

    let hyperstrings = transform(&page_map, root_base_name);

    // It's tempting to try to handle `--single` earlier and exit,
    // but even `--single` still needs to:
    //
    // + crawl to depth 1 for link titles
    // + possibly crawl more if it has transclusions
    // + check for transclusion cycles
    //
    if args.single {
        let root_page = page_map.get(root_base_name).unwrap();
        println!("{}", root_page.hyperstring.to_plaintext(&page_map));
    } else {
        println!("{}", hyperstrings_to_string(page_map, hyperstrings));
    }
}
