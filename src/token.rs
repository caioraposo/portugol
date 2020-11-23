use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),         // add, foobar, x, y, ...
    IntLiteral(String),    // 123456
    FloatLiteral(String),  // 123.456
    StringLiteral(String), // "hello"

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Arrow,

    Lt,
    Gt,

    Eq,
    NotEq,

    Comma,
    Colon,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,

    Int,
    Float,
    String,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    Print,
    Program,
    While,
    Read,
    Let,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::Eof => write!(f, "EOF"),

            Token::Ident(ident) => write!(f, "{}", ident),
            Token::IntLiteral(int) => write!(f, "{}", int),
            Token::FloatLiteral(float) => write!(f, "{}", float),
            // TODO: Escape `"` in a string as `\"`...
            Token::StringLiteral(s) => write!(f, "\"{}\"", s),

            Token::Assign => write!(f, "="),
            Token::Arrow => write!(f, "<-"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),

            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),

            Token::Eq => write!(f, "="),
            Token::NotEq => write!(f, "!="),

            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),

            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "{{"),
            Token::Rbrace => write!(f, "}}"),
            Token::Lbracket => write!(f, "["),
            Token::Rbracket => write!(f, "]"),

            Token::Function => write!(f, "funcao"),
            Token::Let => write!(f, "let"),
            Token::Int => write!(f, "int"),
            Token::Float => write!(f, "real"),
            Token::String => write!(f, "string"),
            Token::True => write!(f, "VERDADEIRO"),
            Token::False => write!(f, "FALSO"),
            Token::If => write!(f, "se"),
            Token::Else => write!(f, "senao"),
            Token::Return => write!(f, "retorne"),
            Token::Print => write!(f, "imprima"),
            Token::Program => write!(f, "prog"),
            Token::While => write!(f, "enquanto"),
            Token::Read => write!(f, "leia"),
        }
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    keyword_to_token(ident).unwrap_or_else(|| Token::Ident(ident.to_owned()))
}

fn keyword_to_token(keyword: &str) -> Option<Token> {
    match keyword {
        "prog" => Some(Token::Program),
        "funcao" => Some(Token::Function),
        "VERDADEIRO" => Some(Token::True),
        "FALSO" => Some(Token::False),
        "se" => Some(Token::If),
        "senao" => Some(Token::Else),
        "retorne" => Some(Token::Return),
        "imprima" => Some(Token::Print),
        "enquanto" => Some(Token::While),
        "leia" => Some(Token::Read),
        "let" => Some(Token::Let),
        "int" => Some(Token::Int),
        "real" => Some(Token::Float),
        "string" => Some(Token::String),
        _ => None,
    }
}
