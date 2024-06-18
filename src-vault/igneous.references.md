---
uuid: 2e3088b9-1ea3-48cb-bbcf-7fa1b0eb8881
title: references
---

type:: [[code]]

impl [[igneous.Hyperstring]] {
    pub fn references(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|segment| match segment {
                [[igneous.HyperstringSegment]]::Link(link) => Some(link.clone()),
                [[igneous.HyperstringSegment]]::Transclusion(transclusion) => Some(transclusion.clone()),
                _ => None,
            })
            .collect()
    }
}