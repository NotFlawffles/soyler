use crate::utils::span::Span;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Kind {
    Identifier,
    Integer,
    Float,
    Boolean,
    String,
    Char,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Modulo,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Exclamation,
    GreaterThan,
    LessThan,
    BitwiseLeftShift,
    BitwiseRightShift,
    Assign,
    AmpersandAmpersand,
    PipePipe,
    AssignAssign,
    PlusAssign,
    MinusAssign,
    AsteriskAssign,
    SlashAssign,
    ModuloAssign,
    AmpersandAssign,
    PipeAssign,
    CaretAssign,
    ExclamationAssign,
    GreaterThanOrEqual,
    LessThanOrEqual,
    BitwiseLeftShiftAssign,
    BitwiseRightShiftAssign,

    LeftParenthesis,
    RightParenthesis,
    LeftSquareBrace,
    RightSquareBrace,
    LeftCurlyBrace,
    RightCurlyBrace,

    Comma,
    SemiColon,
    Arrow,

    Unhandled,
    EndOfFile
}

#[derive(Debug)]
pub struct Token {
    pub kind: Kind,
    pub span: Span,
    pub raw: String
}

impl Token {
    pub fn new(kind: Kind, span: Span, raw: String) -> Self {
        Self {
            kind,
            span,
            raw
        }
    }
}
