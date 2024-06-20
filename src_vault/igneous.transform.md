---
uuid: 78ebcfc2-76ff-48a1-8880-631e30194b44
title: transform
---

type:: [[code]]

pub fn transform(page_map: &HashMap<String, [[igneous.Page]]>, root_base_name: &str) -> Vec<[[igneous.Hyperstring]]> {
    if let Some(cycle) = [[igneous.cyclic_transclusion_check]](page_map) {
        panic!("Cyclic transclusion detected: {:?}", cycle);
    }
    [[igneous.linearize]](page_map, root_base_name)
}
