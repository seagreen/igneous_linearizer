---
uuid: b23b7746-0f68-43fb-8a82-e018bc947017
title: link_title_or_name
---

type:: [[code]]

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