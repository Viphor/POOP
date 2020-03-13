pub use logos::Logos;
use std::fmt;
use std::ops::Range;

pub mod wrapper;

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    // Logos specific
    #[end]
    End,
    #[error]
    Error,

    // Comments
    #[regex = "//[^\n]*\n"]
    LineComment,

    // Special characters
    #[token = "("]
    LParen,
    #[token = ")"]
    RParen,
    #[token = "{"]
    LBrace,
    #[token = "}"]
    RBrace,
    #[token = "["]
    LBracket,
    #[token = "]"]
    RBracket,
    #[token = "."]
    Period,
    #[token = ","]
    Comma,
    #[token = ";"]
    Semicolon,
    #[token = ":"]
    Colon,
    #[token = "->"]
    Arrow,
    #[token = "+"]
    Plus,
    #[token = "-"]
    Minus,
    #[token = "*"]
    Star,
    #[token = "/"]
    Slash,
    #[token = "%"]
    Percent,
    #[token = "="]
    Equal,
    #[token = "=="]
    Equality,
    #[token = "!="]
    NotEq,
    #[token = "<"]
    LessThan,
    #[token = ">"]
    GreaterThan,
    #[token = "<="]
    LessEq,
    #[token = ">="]
    GreaterEq,
    #[token = "!"]
    Not,
    #[token = "&&"]
    And,
    #[token = "||"]
    Or,

    // Identifier
    #[regex = "[a-zA-Z_][a-zA-Z0-9_]*"]
    Ident,

    // Keywords
    #[token = "fn"]
    Fn,
    #[token = "let"]
    Let,
    #[token = "if"]
    If,
    #[token = "else"]
    Else,

    // Literals
    #[regex = "[0-9][0-9_]*"]
    Int,
    #[token = "true"]
    True,
    #[token = "false"]
    False,
    #[regex = "\"[^\"]*\""]
    String,

    // Type names
    #[token = "int"]
    IntType,
    #[token = "float"]
    FloatType,
    #[token = "double"]
    DoubleType,
    #[token = "bool"]
    BooleanType,
    #[token = "void"]
    VoidType,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::And => write!(f, "&&"),
            Token::Arrow => write!(f, "->"),
            Token::BooleanType => write!(f, "bool"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::DoubleType => write!(f, "double"),
            Token::Else => write!(f, "else"),
            Token::End => write!(f, "EOF"),
            Token::Equal => write!(f, "="),
            Token::Equality => write!(f, "=="),
            Token::Error => write!(f, "ERROR"),
            Token::False => write!(f, "false"),
            Token::FloatType => write!(f, "float"),
            Token::Fn => write!(f, "fn"),
            Token::GreaterEq => write!(f, ">="),
            Token::GreaterThan => write!(f, ">"),
            Token::Ident => write!(f, "<identifier>"),
            Token::If => write!(f, "if"),
            Token::Int => write!(f, "<int>"),
            Token::IntType => write!(f, "int"),
            Token::LessEq => write!(f, "<="),
            Token::LessThan => write!(f, "<"),
            Token::Let => write!(f, "let"),
            Token::LineComment => write!(f, "// <comment>"),
            Token::LBrace => write!(f, "{{"),
            Token::LBracket => write!(f, "["),
            Token::LParen => write!(f, "("),
            Token::Minus => write!(f, "-"),
            Token::Not => write!(f, "!"),
            Token::NotEq => write!(f, "!="),
            Token::Or => write!(f, "||"),
            Token::Percent => write!(f, "%"),
            Token::Period => write!(f, "."),
            Token::Plus => write!(f, "+"),
            Token::RBrace => write!(f, "}}"),
            Token::RBracket => write!(f, "]"),
            Token::RParen => write!(f, ")"),
            Token::Semicolon => write!(f, ";"),
            Token::Slash => write!(f, "/"),
            Token::Star => write!(f, "*"),
            Token::String => write!(f, "<string>"),
            Token::True => write!(f, "true"),
            Token::VoidType => write!(f, "void"),
        }
    }
}

pub struct Tokens(Vec<Token>);

impl From<Vec<Token>> for Tokens {
    fn from(value: Vec<Token>) -> Tokens {
        Tokens(value)
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.len() {
            0 => Ok(()),
            1 => write!(f, "{}", self.0[0]),
            2 => write!(f, "{} or {}", self.0[0], self.0[1]),
            n => {
                for token in self.0.iter().take(n - 1) {
                    write!(f, "{}, ", token)?
                }
                write!(f, "or {}", self.0.last().unwrap())
            }
        }
    }
}

#[cfg(test)]
mod test;

pub struct RangeConverter(Vec<Range<usize>>);

impl RangeConverter {
    pub fn new(input: &str) -> Self {
        let mut list = Vec::new();
        let mut last_char = 0;
        for line in input.lines() {
            let last = last_char + if line.is_empty() { 1 } else { line.len() };
            list.push(last_char..last);
            last_char = last + 1;
        }
        Self(list)
    }

    pub fn to_line_and_pos(&self, range: Range<usize>) -> (usize, usize) {
        for (i, line) in self.0.iter().enumerate() {
            if line.contains(&range.start) {
                return (i + 1, range.start - line.start + 1);
            }
        }
        (1, range.start + 1)
    }
}
