use crate::token::{self, Token};
use std::iter::Peekable;
use std::mem;
use std::str::Chars;

pub struct Lexer {
    input: String,
    // Current position in input (points to current char)
    position: usize,
    // current char under examination
    ch: char,
    // Use `Chars` to support UTF-8.
    // https://stackoverflow.com/questions/43952104/how-can-i-store-a-chars-iterator-in-the-same-struct-as-the-string-it-is-iteratin
    chars: Peekable<Chars<'static>>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars = unsafe { mem::transmute(input.chars().peekable()) };
        let mut lexer = Lexer {
            input,
            position: 0,
            ch: '\u{0}',
            chars,
        };
        lexer.read_char();
        lexer
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token;
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::Eq;
                } else {
                    tok = Token::Assign;
                }
            }
            ':' => {
                tok = Token::Colon;
            }
            ';' => {
                tok = Token::Semicolon;
            }
            '(' => {
                tok = Token::Lparen;
            }
            ')' => {
                tok = Token::Rparen;
            }
            ',' => {
                tok = Token::Comma;
            }
            '+' => {
                tok = Token::Plus;
            }
            '-' => {
                tok = Token::Minus;
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = Token::NotEq;
                } else {
                    tok = Token::Bang;
                }
            }
            '*' => {
                tok = Token::Asterisk;
            }
            '/' => {
                tok = Token::Slash;
            }
            '<' => {
                if self.peek_char() == '-' {
                    self.read_char();
                    tok = Token::Arrow;
                } else {
                    tok = Token::Lt;
                }
            }
            '>' => {
                tok = Token::Gt;
            }
            '{' => {
                tok = Token::Lbrace;
            }
            '}' => {
                tok = Token::Rbrace;
            }
            '[' => {
                tok = Token::Lbracket;
            }
            ']' => {
                tok = Token::Rbracket;
            }
            '"' => {
                tok = Token::StringLiteral(self.read_string().to_string());
                self.read_char();
                return tok;
            }
            '\u{0}' => {
                tok = Token::Eof;
            }
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    tok = token::lookup_ident(ident);
                } else if is_digit(self.ch) {
                    // Accepts 1 | 1. | 1.0 as numbers
                    let integer_part = self.read_number().to_string();
                    if self.ch == '.' {
                        self.read_char();
                        if is_digit(self.peek_char()) {
                            let fractional_part = self.read_number();
                            tok = Token::FloatLiteral(format!(
                                "{}.{}",
                                integer_part, fractional_part
                            ));
                        } else {
                            tok = Token::FloatLiteral(format!("{}.0", integer_part));
                        }
                    } else {
                        tok = Token::IntLiteral(integer_part);
                    }
                } else {
                    tok = Token::Illegal
                }
                return tok;
            }
        }
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        // The first character needs to be a letter.
        if is_letter(self.ch) {
            self.read_char();
        }
        // The second character and after can be a letter or a digit.
        while is_letter(self.ch) || is_digit(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_string(&mut self) -> &str {
        let position = self.position + 1;
        loop {
            self.read_char();
            // TODO: Return an error when it reaches EOF.
            // TODO: Support escaping like `\"`, `\n`, `\t`, etc.
            if self.ch == '"' || self.ch == '\u{0}' {
                break;
            }
        }
        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }

    // -- Low-level methods that touches the `Chars`.

    fn read_char(&mut self) {
        self.position += if self.ch == '\u{0}' {
            0
        } else {
            self.ch.len_utf8()
        };
        self.ch = self.chars.next().unwrap_or('\u{0}');
    }

    fn peek_char(&mut self) -> char {
        self.chars.peek().cloned().unwrap_or('\u{0}')
    }
}

fn is_letter(ch: char) -> bool {
    ch == '_'
        || ch == '$'
        // `is_alphabetic` includes kanji but not emoji.
        || ch.is_alphabetic()
        // A rough emoji range
        // TODO: Review https://unicode.org/Public/emoji/12.0/emoji-data.txt
        // TODO: What to do with modifiers?
        || ('\u{203C}' <= ch && ch <= '\u{3299}')
        || ('\u{1F000}' <= ch && ch <= '\u{1FA95}')
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}
