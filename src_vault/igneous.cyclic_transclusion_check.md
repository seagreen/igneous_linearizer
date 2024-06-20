---
uuid: 27f61c60-cc72-42d0-ba4f-f16d4fd533ad
title: cyclic_transclusion_check
---

type:: [[code]]

/// If transclusions form a cycle return `Some` and the base filename of one of the pages involved.
/// Otherwise return `None`.
pub fn cyclic_transclusion_check(page_map: &HashMap<String, [[igneous.Page]]>) -> Option<String> {
    let mut node_map: HashMap<String, _> = HashMap::new();
    let mut graph: Graph<&str, ()> = DiGraph::new();

    for base_name in page_map.keys() {
        let node = graph.add_node(base_name);
        node_map.insert(base_name.clone(), node);
    }

    for (base_name, page) in page_map {
        let node = node_map.get(base_name).unwrap();
        for transclusion in &page.hyperstring.[[igneous.transclusions]]() {
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
                hyperstring: [[igneous.Hyperstring]](vec![HyperstringSegment::Transclusion(
                    "page-2".to_string(),
                )]),
            },
        );
        page_map.insert(
            "page-2".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: [[igneous.Hyperstring]](vec![HyperstringSegment::Transclusion(
                    "page-3".to_string(),
                )]),
            },
        );
        page_map.insert(
            "page-3".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: [[igneous.Hyperstring]](vec![HyperstringSegment::Transclusion(
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
                hyperstring: [[igneous.Hyperstring]](vec![HyperstringSegment::Transclusion(
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
                hyperstring: [[igneous.Hyperstring]](vec![HyperstringSegment::Transclusion(
                    "page-2".to_string(),
                )]),
            },
        );
        page_map.insert(
            "page-2".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: [[igneous.Hyperstring]](vec![HyperstringSegment::Transclusion(
                    "page-3".to_string(),
                )]),
            },
        );
        page_map.insert(
            "page-3".to_string(),
            Page {
                metadata: HashMap::new(),
                hyperstring: [[igneous.Hyperstring]](vec![]),
            },
        );
        assert_eq!(cyclic_transclusion_check(&page_map), None);
    }
}