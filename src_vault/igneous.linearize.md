---
uuid: c7aef55f-a015-4822-9145-3bc3db9cf50f
title: linearize
---

type:: [[code]]

pub fn linearize(page_map: &HashMap<String, [[igneous.Page]]>, root_base_name: &str) -> Vec<[[igneous.Hyperstring]]> {
    let mut result = Vec::new();
    let mut seen_set: HashSet<String> = HashSet::new();
    let mut stack = vec![root_base_name.to_string()];

    while let Some(base_name) = stack.pop() {
        let page = page_map.get(&base_name).unwrap();
        result.push(page.hyperstring.clone());
        for link in page.hyperstring.[[igneous.links]]() {
            if !seen_set.contains(&link) {
                stack.push(link.clone());
                seen_set.insert(link.clone());
            }
        }
    }

    result
}