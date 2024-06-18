---
uuid: 4421ab6f-0b95-4a16-a6b2-9e015b838d60
---

type:: [[code]]

use clap::Parser;
use gray_matter::Matter;
use petgraph::graph::DiGraph;
use petgraph::Graph;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
