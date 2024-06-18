---
uuid: 48d7f111-dc4b-4937-94fe-6916d1373b06
---

type:: [[code]]

impl fmt::Display for [[igneous.Hyperstring]] {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for segment in &self.0 {
            write!(f, "{}", segment)?;
        }
        Ok(())
    }
}

impl fmt::Display for [[igneous.HyperstringSegment]] {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            [[igneous.HyperstringSegment]]::Text(text) => write!(f, "{}", text),
            [[igneous.HyperstringSegment]]::Link(link) => write!(f, "\u{005B}\u{005B}{}]]", link),
            [[igneous.HyperstringSegment]]::Transclusion(transclusion) => {
                write!(f, "!\u{005B}\u{005B}{}]]", transclusion)
            }
        }
    }
}