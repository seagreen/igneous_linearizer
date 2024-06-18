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

    #[arg(long, help = "Only transpile ROOT_FILE_PATH")]
    single: bool,
}