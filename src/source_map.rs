#[derive(Debug, Default)]
pub struct SourceMap<'a> {
    sources: Vec<Source<'a>>,
}

#[derive(Debug)]
pub struct Source<'a> {
    pub filename: &'a str,
    pub text: &'a str,
}

impl<'a> SourceMap<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_source(&mut self, source: Source<'a>) -> SourceId {
        self.sources.push(source);
        SourceId(self.sources.len() - 1)
    }

    pub fn get_source(&self, id: SourceId) -> &Source {
        &self.sources[id.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceId(usize);