---
uuid: 3b23a239-cc49-465f-9edb-e466724ab102
title: main
---

type:: [[code]]

fn main() {
    let args = [[igneous.Args]]::parse();

    let vault_dir = args.root_file.parent().unwrap();
    let root_file_name = args.root_file.file_name().unwrap().to_str().unwrap();
    let root_base_name = root_file_name
        .strip_suffix(".md")
        .unwrap_or("Initial file must start with .md");

    let page_map = [[igneous.crawl]](vault_dir, root_base_name);

    let hyperstrings = [[igneous.transform]](&page_map, root_base_name);

    // It's tempting to try to handle `--single` earlier and exit,
    // but even `--single` still needs to:
    //
    // + crawl to depth 1 for link titles
    // + possibly crawl more if it has transclusions
    // + check for transclusion cycles
    //
    if args.single {
        let root_page = page_map.get(root_base_name).unwrap();
        println!("{}", root_page.hyperstring.[[igneous.to_plaintext]](&page_map));
    } else {
        println!("{}", [[igneous.hyperstrings_to_string]](page_map, hyperstrings));
    }
}