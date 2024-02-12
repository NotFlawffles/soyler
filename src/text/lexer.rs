use crate::utils::span::Span;
use crate::text::token;
use crate::diagnostic::diagnostic;

pub struct Lexer {
    pub content: String,
    pub span: Span,
    pub current: Option<u8>
}

impl Lexer {
    pub fn new(stream: String, content: String) -> Self {
        Self {
            content,
            span: Span::new(stream.clone(), 1, 1, 0, stream.len()),
            current: content.as_bytes().get(0).copied()
        }
    }

    pub fn next(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        self.skip_whitespace();

        match self.current {
            Some(b'_') => self.lex_identifier(),
            Some(b'.') => self.lex_number(),
            Some(b'"') => self.lex_string(),
            Some(b'\'') => self.lex_char(),
            Some(b'+') => self.lex_plus(),
            Some(b'-') => self.lex_minus(),
            Some(b'*') => self.lex_asterisk(),
            Some(b'/') => self.lex_slash(),
            Some(b'%') => self.lex_modulo(),
            Some(b'&') => self.lex_ampersand(),
            Some(b'|') => self.lex_pipe(),
            Some(b'^') => self.lex_caret(),
            Some(b'~') => self.lex_tilde(),
            Some(b'!') => self.lex_exclamation(),
            Some(b'>') => self.lex_greater_than(),
            Some(b'<') => self.lex_less_than(),
            Some(b'=') => self.lex_assign(),
            Some(b'(') | Some(b')') | Some(b'[') | Some(b']') | Some(b'{') | Some(b'}') => self.lex_bracket(),
            Some(b',') => self.lex_comma(),
            Some(b';') => self.lex_semicolon(),

            Some(c) => {
                if c.is_ascii_alphabetic() {
                    self.lex_identifier()
                } else if c.is_ascii_digit() {
                    self.lex_number()
                } else {
                    self.lex_unhandled()
                }
            },

            None => Ok(token::Token::new(token::Kind::EndOfFile, self.span, String::from("end of file")))
        }
    }

    fn lex_identifier(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        let mut value = String::new();

        while self.current.is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_') {
            value.push(self.current.unwrap() as char);
            self.advance();
        }

        span.length = value.len();

        Ok(token::Token::new(
            token::Kind::Identifier,
            span,
            value
        ))
    }

    fn lex_number(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        let mut value = String::new();
        let mut kind = token::Kind::Integer;

        while self.current.is_some_and(|c| c.is_ascii_digit() || c == b'.') {
            if self.current == Some(b'.') {
                if kind == token::Kind::Float {
                    return Err(diagnostic::Diagnostic::new(
                        diagnostic::Kind::SyntaxError,
                        span,
                        String::from("unexpected '.'")
                    ));
                }

                kind = token::Kind::Float;
            }

            value.push(self.current.unwrap() as char);
            self.advance();
        }

        span.length = value.len();


        if value.as_bytes().get(0) == Some(&b'.') && span.length == 1 {
            Err(diagnostic::Diagnostic::new(
                diagnostic::Kind::SyntaxError,
                span,
                String::from("unexpected '.'")
            ))
        } else {
            Ok(token::Token::new(
                kind,
                span,
                value
            ))
        }
    }

    fn lex_string(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {

    }

    fn lex_char(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        
    }

    fn lex_plus(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::PlusAssign,
                    span,
                    String::from("+=")
                )))
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Plus,
                span,
                String::from("+")
            )))
        }
    }

    fn lex_minus(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::MinusAssign,
                    span,
                    String::from("-=")
                )))
            },

            Some(b'>') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::Arrow,
                    span,
                    String::from("->")
                )))
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Plus,
                span,
                String::from("+")
            )))
        }
    }

    fn lex_asterisk(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::AsteriskAssign,
                    span,
                    String::from("*=")
                )))
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Asterisk,
                span,
                String::from("*")
            )))
        }
    }

    fn lex_slash(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::SlashAssign,
                    span,
                    String::from("/=")
                )))
            },

            Some(b'/') => {
                while self.current.is_some_and(|c| c != b'\n') {
                    self.advance();
                    span.length += 1;
                }

                self.next()
            },

            Some(b'*') => {
                loop {
                    if self.current == Some(b'*') {
                        self.advance();

                        if self.current.is_none() {
                            return Err(diagnostic::Diagnostic::new(
                                diagnostic::Kind::SyntaxError,
                                span,
                                String::from("unterminated sequence")
                            ));
                        }

                        if self.current == Some(b'/') {
                            self.advance();
                            break;
                        }
                    }

                    self.advance();
                    span.length += 1;
                }

                self.next()
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Slash,
                span,
                String::from("/")
            )))
        }
    }

    fn lex_modulo(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::ModuloAssign,
                    span,
                    String::from("%=")
                )))
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Modulo,
                span,
                String::from("%")
            )))
        }
    }

    fn lex_ampersand(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::AmpersandAssign,
                    span,
                    String::from("&=")
                )))
            },

            Some(b'&') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::AmpersandAmpersand,
                    span,
                    String::from("&&")
                )))
            }

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Ampersand,
                span,
                String::from("&")
            )))
        }
    }

    fn lex_pipe(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::PipeAssign,
                    span,
                    String::from("|=")
                )))
            },

            Some(b'|') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::PipePipe,
                    span,
                    String::from("||")
                )))
            }

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Pipe,
                span,
                String::from("|")
            )))
        }
    }

    fn lex_caret(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::CaretAssign,
                    span,
                    String::from("^=")
                )))
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Caret,
                span,
                String::from("^")
            )))
        }
    }

    fn lex_tilde(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        Ok(self.advance_with_token(token::Token::new(
            token::Kind::Tilde,
            span,
            String::from("~")
        )))
    }

    fn lex_exclamation(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::ExclamationAssign,
                    span,
                    String::from("!=")
                )))
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Exclamation,
                span,
                String::from("!")
            )))
        }
    }

    fn lex_greater_than(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::GreaterThanOrEqual,
                    span,
                    String::from(">=")
                )))
            },

            Some(b'>') => {
                span.length += 1;
                self.advance();

                match self.current {
                    Some(b'=') => {
                        span.length += 1;

                        Ok(self.advance_with_token(token::Token::new(
                            token::Kind::BitwiseRightShiftAssign,
                            span,
                            String::from(">>=")
                        )))
                    },

                    _ => {
                        Ok(self.advance_with_token(token::Token::new(
                            token::Kind::BitwiseRightShift,
                            span,
                            String::from(">>")
                        )))
                    }
                }
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::GreaterThan,
                span,
                String::from("!")
            )))
        }
    }

    fn lex_less_than(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span.length += 1;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::LessThanOrEqual,
                    span,
                    String::from("<=")
                )))
            },

            Some(b'<') => {
                span.length += 1;
                self.advance();

                match self.current {
                    Some(b'=') => {
                        span.length += 1;

                        Ok(self.advance_with_token(token::Token::new(
                            token::Kind::BitwiseLeftShiftAssign,
                            span,
                            String::from("<<=")
                        )))
                    },

                    _ => {
                        Ok(self.advance_with_token(token::Token::new(
                            token::Kind::BitwiseLeftShift,
                            span,
                            String::from("<<")
                        )))
                    }
                }
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::LessThan,
                span,
                String::from("<")
            )))
        }
    }

    fn lex_assign(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 0;
        self.advance();

        match self.current {
            Some(b'=') => {
                span = self.span.clone();
                span.length = 2;

                Ok(self.advance_with_token(token::Token::new(
                    token::Kind::AssignAssign,
                    span,
                    String::from("==")
                )))
            },

            None => Ok(self.advance_with_token(token::Token::new(
                token::Kind::Assign,
                span,
                String::from("=")
            )))
        }
    }

    fn lex_bracket(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 1;

        match self.current.unwrap() {
            b'(' => Ok(self.advance_with_token(token::Token::new(
                token::Kind::LeftParenthesis,
                span,
                String::from("(")
            ))),

            b')' => Ok(self.advance_with_token(token::Token::new(
                token::Kind::RightParenthesis,
                span,
                String::from(")")
            ))),

            b'[' => Ok(self.advance_with_token(token::Token::new(
                token::Kind::LeftSquareBrace,
                span,
                String::from("[")
            ))),

            b']' => Ok(self.advance_with_token(token::Token::new(
                token::Kind::RightSquareBrace,
                span,
                String::from("]")
            ))),

            b'{' => Ok(self.advance_with_token(token::Token::new(
                token::Kind::LeftCurlyBrace,
                span,
                String::from("{")
            ))),

            b'}' => Ok(self.advance_with_token(token::Token::new(
                token::Kind::RightCurlyBrace,
                span,
                String::from("}")
            ))),
        }
    }

    fn lex_comma(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 1;

        Ok(self.advance_with_token(token::Token::new(
            token::Kind::Comma,
            span,
            String::from_utf8(vec![self.current.unwrap()]).unwrap()
        )))
    }

    fn lex_semicolon(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 1;

        Ok(self.advance_with_token(token::Token::new(
            token::Kind::SemiColon,
            span,
            String::from_utf8(vec![self.current.unwrap()]).unwrap()
        )))
    }

    fn lex_unhandled(&mut self) -> Result<token::Token, diagnostic::Diagnostic> {
        let mut span = self.span.clone();
        span.length = 1;

        Ok(self.advance_with_token(token::Token::new(
            token::Kind::Unhandled,
            span,
            String::from_utf8(vec![self.current.unwrap()]).unwrap()
        )))
    }

    fn advance_with_token(&mut self, token: token::Token) -> token::Token {
        self.advance();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.current.is_some_and(|c| c.is_ascii_whitespace()) {
            let current = self.current.clone();
            if current == Some(b'\n') {
                self.span.row += 1;
                self.span.column = 0;
            }

            self.advance();
        }
    }

    fn advance(&mut self) {
        self.span.index += 1;
        self.span.column += 1;
        self.current = self.content.as_bytes().get(self.span.index).copied();
    }
}
