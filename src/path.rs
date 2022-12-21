use crate::{ident::Ident, span::Span, specialization::Spec};

#[derive(Clone, Debug, PartialEq)]
pub struct IdentSegment {
    pub name: Ident,
    pub spec: Spec,
    pub span: Span,
}

impl std::fmt::Display for IdentSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;

        if !self.spec.is_empty() {
            write!(f, "{}", self.spec)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SelfSegment {
    pub span: Span,
}

impl std::fmt::Display for SelfSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "self")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathSegment {
    IdentSegment(IdentSegment),
    SelfSegment(SelfSegment),
}

impl std::fmt::Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdentSegment(segment) => write!(f, "{}", segment),
            Self::SelfSegment(segment) => write!(f, "{}", segment),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub is_absolute: bool,
    pub segments: Vec<PathSegment>,
    pub span: Span,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_absolute {
            write!(f, "::")?;
        }

        let segments: Vec<_> = self.segments.iter().map(PathSegment::to_string).collect();
        write!(f, "{}", segments.join("::"))?;

        Ok(())
    }
}
