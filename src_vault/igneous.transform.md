---
uuid: 78ebcfc2-76ff-48a1-8880-631e30194b44
title: transform
---

type:: [[code]]

// 4 possible types of links:
//
// 1. plain link: add it to the serialization file and recursively crawl it
// 2. transclusion: just recursively crawl it
// 3. [not yet implemented] plain link in the 'ignore' list: read it (to find its 'title')
// 4. [not yet implemented] transclusion in the 'ignore' list: read it (to find its 'title')
pub fn transform(page_map: &HashMap<String, [[igneous.Page]]>, root_base_name: &str) -> Vec<[[igneous.Hyperstring]]> {
    if let Some(cycle) = [[igneous.cyclic_transclusion_check]](page_map) {
        panic!("Cyclic transclusion detected: {:?}", cycle);
    }
    [[igneous.linearize]](page_map, root_base_name)
}
