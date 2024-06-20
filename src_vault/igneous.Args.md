---
uuid: 2bbb74cd-be92-4cdd-817d-741c8e0a0473
title: Args
---

type:: [[code]]

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_name = "ROOT_FILE_PATH")]
    root_file: PathBuf,

    #[arg(
        long,
        help = concat!(
            "Only transpile ROOT_FILE_PATH, don't crawl its dependencies and include them in the output as well.",
            " Useful for compiling import blocks that must be at the start of a file,",
            " e.g. `igneous-linearizer --single import.md > main.rs; igneous-linearierizer main.md >> main.rs"
        )
    )]
    single: bool,
}