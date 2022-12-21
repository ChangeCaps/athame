use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use uuid::Uuid;

use crate::span::Span;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SourceId {
    uuid: Uuid,
}

impl SourceId {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }

    pub const fn null() -> Self {
        Self { uuid: Uuid::nil() }
    }
}

#[derive(Clone, Debug)]
pub struct Source {
    path: PathBuf,
    source: Arc<str>,
}

impl Source {
    pub fn new(path: impl Into<PathBuf>, source: impl Into<Arc<str>>) -> Self {
        Self {
            path: path.into(),
            source: source.into(),
        }
    }

    pub fn open(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let source = std::fs::read_to_string(&path).unwrap();
        let source = Arc::from(source);

        Self { path, source }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn span(&self, span: Span) -> SourceSpan<'_> {
        SourceSpan::new(self, span)
    }
}

pub struct SourceSpan<'a> {
    line: usize,
    column: usize,
    path: &'a Path,
    source: &'a str,
    span: Span,
}

impl<'a> SourceSpan<'a> {
    pub fn new(source: &'a Source, span: Span) -> Self {
        let before = &source.source()[..span.index];

        let line = before.matches('\n').count() + 1;
        let column = before.rfind('\n').map_or(span.index, |i| span.index - i);

        Self {
            line,
            column,
            path: source.path(),
            source: &source.source()[span.index..span.end()],
            span,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn path(&self) -> &Path {
        self.path
    }

    pub fn source(&self) -> &'a str {
        self.source
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, Default)]
pub struct Sources {
    sources: HashMap<SourceId, Source>,
}

impl Sources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, source: Source) -> SourceId {
        let id = SourceId::new();
        self.sources.insert(id, source);
        id
    }

    pub fn open(&mut self, path: impl Into<PathBuf>) -> SourceId {
        let path = path.into();
        let source = Source::open(&path);

        let id = SourceId::new();
        self.sources.insert(id, source);

        id
    }

    pub fn get(&self, id: SourceId) -> Option<&Source> {
        self.sources.get(&id)
    }

    pub fn get_span(&self, span: Span) -> Option<SourceSpan<'_>> {
        let source = self.get(span.source)?;
        Some(source.span(span))
    }
}
