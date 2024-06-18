---
uuid: 71638d9e-3f2e-45d5-a8e1-3ba63a334efa
title: to_plaintext
---

type:: [[code]]

impl [[igneous.Hyperstring]] {
    pub fn to_plaintext(&self, page_map: &HashMap<String, Page>) -> String {
        self.0
            .iter()
            .map(|segment| match segment {
                [[igneous.HyperstringSegment]]::Text(text) => text.clone(),
                [[igneous.HyperstringSegment]]::Link(link) => [[igneous.link_title_or_name]](page_map, link),
                [[igneous.HyperstringSegment]]::Transclusion(transclusion) => {
                    let transclusion_page = page_map.get(transclusion).unwrap();
                    transclusion_page.hyperstring.to_plaintext(page_map)
                }
            })
            .collect::<Vec<_>>()
            .concat()
    }
}