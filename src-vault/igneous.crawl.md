---
uuid: 23fa5bfd-bf48-4a84-b7ec-f2c2412ca5c1
title: crawl
---

type:: [[code]]

pub fn crawl(vault_dir: &Path, root_base_name: &str) -> HashMap<String, [[igneous.Page]]> {
    let mut visited: HashMap<String, Page> = HashMap::new();
    let mut to_visit: Vec<String> = vec![root_base_name.to_string()];

    while let Some(base_name) = to_visit.pop() {
        if visited.contains_key(&base_name) {
            continue;
        }
        let full_path = vault_dir.join(format!("{}.md", base_name));
        let file_content = fs::read_to_string(&full_path)
            .unwrap_or_else(|_| panic!("Attempted to read the file {}", full_path.display()));
        let (metadata, hyperstring) = [[igneous.parse_page]](&file_content);

        for reference in hyperstring.[[igneous.references]]() {
            to_visit.push(reference);
        }
        visited.insert(
            base_name,
            [[igneous.Page]] {
                metadata,
                hyperstring: hyperstring.clone(),
            },
        );
    }

    visited
}