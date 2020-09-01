use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenWrapper {
    pub token: Token,
    pub category: Category,
    line: usize,
    column: usize,
}

impl TokenWrapper {
    pub fn new(token: Token, line: usize, column: usize) -> TokenWrapper {
        let category = match &token {
            Token::Assign | Token::Plus | Token::Minus |
            Token::Bang | Token::Asterisk | Token::Slash |
            Token::Lt | Token::Gt | Token::Eq | Token::NotEq => Category::Operator,

            Token::Integer | Token::Function | Token::True |
            Token::False | Token::If | Token::Else |
            Token::Return | Token::Print | Token::Program => Category::Keyword,

            Token::String(_) => Category::String,
            Token::Ident(_) => Category::Identifier,
            Token::Int(_) | Token::Float(_) => Category::Number,

            _ => Category::Others,

        };
        TokenWrapper { token, category, line, column }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Category {
    Keyword,
    Identifier,
    Others,
    Operator,
    Number,
    String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),  // add, foobar, x, y, ...
    Int(String),    // 123456
    Float(String),  // 123.456
    String(String), // "hello"

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

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

    Integer,
    Function,
    True,
    False,
    If,
    Else,
    Return,
    Print,
    Program,
}

impl fmt::Display for TokenWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}, \"{}\", {}, {}]",
            self.category, self.token, self.line, self.column)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::Eof => write!(f, "EOF"),

            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Int(int) => write!(f, "{}", int),
            Token::Float(float) => write!(f, "{}", float),
            // TODO: Escape `"` in a string as `\"`...
            Token::String(s) => write!(f, "\"{}\"", s),

            Token::Assign => write!(f, "="),
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
            Token::Integer => write!(f, "int"),
            Token::True => write!(f, "VERDADEIRO"),
            Token::False => write!(f, "FALSO"),
            Token::If => write!(f, "se"),
            Token::Else => write!(f, "senao"),
            Token::Return => write!(f, "retorne"),
            Token::Print => write!(f, "imprima"),
            Token::Program => write!(f, "prog"),
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
        "int" => Some(Token::Integer),
        "VERDADEIRO" => Some(Token::True),
        "FALSO" => Some(Token::False),
        "se" => Some(Token::If),
        "senao" => Some(Token::Else),
        "retorne" => Some(Token::Return),
        "imprima" => Some(Token::Print),
        _ => None,
    }
}
