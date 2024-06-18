---
uuid: b7f1ca8d-66a8-487e-a486-740877fd127c
title: transclusions
---

type:: [[code]]

impl [[igneous.Hyperstring]] {
    pub fn transclusions(&self) -> Vec<String> {
        self.0
            .iter()
            .filter_map(|segment| {
                if let [[igneous.HyperstringSegment]]::Transclusion(transclusion) = segment {
                    Some(transclusion.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}