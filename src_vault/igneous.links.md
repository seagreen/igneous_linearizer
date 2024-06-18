---
uuid: ecb2e766-b60c-496f-8f3b-f01ba4438a15
title: links
---

type:: [[code]]

// Also grab the [[Igneous Linearizer snippets]]

impl [[igneous.Hyperstring]] {
    pub fn links(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|segment| {
                if let [[igneous.HyperstringSegment]]::Link(link) = segment {
                    Some(link.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}