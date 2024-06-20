---
uuid: c7aef55f-a015-4822-9145-3bc3db9cf50f
title: linearize
---

type:: [[code]]

/// Topologically sort all pages transitively referenced by the root page.
///
/// Invariant: always returns the root page as the first element of the output vector.
///
/// Transclusions will have their transitive references included.
/// So if you transclude some code which itself references other pages,
/// those pages will appear in the output vector.
///
/// The transclusion itself though will not
/// (assuming it's only used as a transclusion and never linked to).
/// In that case it's assumed to be a snippet not a standalone code definition.
/// Therefore it shouldn't appear at the top level of the eventual output file
/// and thus isn't included at the top level of the output vector here.
pub fn linearize(page_map: &HashMap<String, [[igneous.Page]]>, root_base_name: &str) -> Vec<[[igneous.Hyperstring]]> {
    let all_links: HashSet<String> = page_map
        .values()
        .flat_map(|page| page.hyperstring.[[igneous.links]]())
        .collect();

    let mut result = Vec::new();
    let mut seen_set: HashSet<String> = HashSet::new();
    let mut stack = vec![root_base_name.to_string()];

    while let Some(base_name) = stack.pop() {
        let page = page_map.get(&base_name).unwrap();
        if base_name == root_base_name || all_links.contains(&base_name) {
            result.push(page.hyperstring.clone());
        }
        for reference in page.hyperstring.[[igneous.references]]() {
            if !seen_set.contains(&reference) {
                stack.push(reference.clone());
                seen_set.insert(reference.clone());
            }
        }
    }

    result
}