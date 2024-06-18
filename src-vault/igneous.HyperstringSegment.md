---
uuid: c987b419-6bbb-4d6d-b919-36c75a6886e1
title: HyperstringSegment
---

type:: [[code]]

#[derive(PartialEq, Debug, Clone)]
pub enum HyperstringSegment {
    Text(String),
    Link(String),
    Transclusion(String),
}