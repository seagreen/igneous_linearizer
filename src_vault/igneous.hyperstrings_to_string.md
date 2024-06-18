---
uuid: 35c6799b-e49e-4ffa-96d9-651e220bce87
title: hyperstrings_to_string
---

type:: [[code]]

pub fn hyperstrings_to_string(
    page_map: HashMap<String, [[igneous.Page]]>,
    hyperstrings: Vec<[[igneous.Hyperstring]]>,
) -> String {
    hyperstrings
        .into_iter()
        .rev()
        .map(|hs| hs.[[igneous.to_plaintext]](&page_map))
        .collect::<Vec<_>>()
        .join("\n\n")
}