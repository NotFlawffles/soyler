use crate::utils::span::Span;
use std::fmt::{Display, Formatter};

pub enum Kind {
    FileNotFound,
    FileNotRegular,
    FileNotReadable,
    SyntaxError
}

pub struct Diagnostic {
    pub kind: Kind,
    pub span: Span,
    pub message: String
}

impl Diagnostic {
    pub fn new(kind: Kind, span: Span, message: String) -> Self {
        Self {
            kind,
            span,
            message
        }
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}[{}:{}]: {}", self.span.stream, self.span.row, self.span.column, self.message)
    }
}
