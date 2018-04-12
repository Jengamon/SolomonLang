mod token;

pub use self::token::{Token, TokenType};

use std::convert::AsRef;

#[derive(Clone, Debug)]
pub struct Lexer {
    text: String,
    index: usize,
    lindex: usize,
}
pub type LexerError = String; // Stopgap
impl Lexer {
    pub fn new(text: String) -> Lexer {
        Lexer {
            text, index: 0, lindex: 1,
        }
    }

    fn single<S: AsRef<str>>(&self, t: TokenType, s: S) -> Option<Token> {
        Some(Token::with_data(t, s.as_ref().into(), self.lindex))
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        let mut tok = None;
        while let None = tok {
            tok = match self.next_char() {
                Some(tok) => match tok {
                    '.' => self.single(TokenType::Dot, "."),
                    ',' => self.single(TokenType::Comma, ","),
                    ':' => match self.peek() {
                        Some('=') => {self.next_char(); Some(Token::new(TokenType::ColonEqual, self.lindex))},
                        _ => Some(Token::new(TokenType::Colon, self.lindex))
                    },
                    '-' => match self.peek() {
                        Some('>') => {self.next_char(); Some(Token::new(TokenType::ThinArrow, self.lindex))},
                        _ => Some(Token::new(TokenType::Minus, self.lindex))
                    },
                    '=' => match self.peek() {
                        Some('>') => {self.next_char(); Some(Token::new(TokenType::ThickArrow, self.lindex))},
                        Some('=') => {self.next_char(); Some(Token::new(TokenType::EqualEqual, self.lindex))},
                        _ => Some(Token::new(TokenType::Equal, self.lindex))
                    },
                    '+' => self.single(TokenType::Plus, "+"),
                    '{' => Some(Token::new(TokenType::LBrace, self.lindex)),
                    '}' => Some(Token::new(TokenType::RBrace, self.lindex)),
                    ')' => Some(Token::new(TokenType::RParen, self.lindex)),
                    '(' => Some(Token::new(TokenType::LParen, self.lindex)),
                    '[' => Some(Token::new(TokenType::LBracket, self.lindex)),
                    ']' => Some(Token::new(TokenType::RBracket, self.lindex)),
                    '*' => self.single(TokenType::Asterisk, "*"),
                    '$' => Some(self.identifier_like(None, false, TokenType::Atom)?),
                    '|' => Some(Token::new(TokenType::Pipe, self.lindex)),
                    '\\' => Some(Token::new(TokenType::Backslash, self.lindex)),
                    '/' => {
                        match self.peek() {
                            Some('/') => { // one-line comment
                                while self.peek() != Some('\n') { self.next_char(); }
                                self.next_char(); // Eat the \n
                                self.lindex += 1;
                            },
                            Some('*') => {
                                let mut level = 1;
                                let startline = self.lindex;
                                while level > 0 { // ml comment
                                    match self.next_char() {
                                        Some('*') => match self.peek() {
                                            Some('/') => {self.next_char(); level -= 1},
                                            _ => {}
                                        },
                                        Some('/') => match self.peek() {
                                            Some('*') => {self.next_char(); level += 1},
                                            _ => {}
                                        },
                                        Some('\n') => self.lindex += 1,
                                        None => return Err(format!("Unterminated multiline comment from line {}", startline)),
                                        _ => {}
                                    };
                                }
                            },
                            _ => return Err(format!("Stray / on line {}", self.lindex))
                        };
                        None
                    },
                    '\n' => {self.lindex += 1; None},
                    '\r' => None,
                    ' ' | '\t' => {
                        while let Some(c) = self.peek() {
                            match c {
                                ' ' | '\t' => self.next_char(),
                                _ => break
                            };
                        }
                        //Some(Token::separator(self.lindex))
                        None
                    },
                    r @ '_' => Some(self.identifier_like(Some(r), true, TokenType::Identifier)?),
                    r if r.is_alphabetic() => Some(self.identifier_like(Some(r), true, TokenType::Identifier)?),
                    r if r.is_ascii_digit() => Some(self.number(r)?),
                    _ => unimplemented!("{:?}:{}", self.text.chars().nth(self.index-1),self.lindex)
                },
                None => Some(Token::eof(self.lindex))
            }
        }
        Ok(tok.unwrap())
    }

    fn keyword(s: String) -> Option<TokenType> {
        match &*s.to_lowercase() {
            "and" => Some(TokenType::And),
            "or" => Some(TokenType::Or),
            "not" => Some(TokenType::Not),
            "group" => Some(TokenType::Group),
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "while" => Some(TokenType::While),
            "do" => Some(TokenType::Do),
            "for" => Some(TokenType::For),
            "interface" => Some(TokenType::Interface),
            "any" => Some(TokenType::AnyType),
            "void" => Some(TokenType::VoidType),
            "int" => Some(TokenType::IntType),
            "string" => Some(TokenType::StringType),
            "fun" => Some(TokenType::Fun),
            _ => None
        }
    }

    // LEXER RULES
    fn identifier_like(&mut self, c: Option<char>, find_keywords: bool, t: TokenType) -> Result<Token, LexerError> {
        let mut s = String::new();
        if c.is_some() {
            s.push(c.unwrap());
        }

        loop {
            match self.peek() {
                Some(r) if r.is_alphanumeric() => { s.push(self.next_char().unwrap()) },
                Some(r@'_') | Some(r@'@') | Some(r@'!') | Some(r@'?') => { s.push(self.next_char().unwrap()) },
                _ => break
            }
        }

        let t = if find_keywords {
            Lexer::keyword(s.clone()).unwrap_or(t)
        } else {
            t
        };

        Ok(Token::with_data(t, s, self.lindex))
    }

    fn number(&mut self, c: char) -> Result<Token, LexerError> {
        let mut s = String::new();
        s.push(c);

        let mut dotwatch = false;
        loop {
            match self.peek() {
                Some(r) if r.is_ascii_digit() => { s.push(self.next_char().unwrap()) },
                Some('.') if !dotwatch => {dotwatch = true; s.push(self.next_char().unwrap())},
                _ => break
            }
        }

        let ttype = if dotwatch {
            TokenType::Float
        } else {
            TokenType::Integer
        };
        //println!("IVAL => {:?}, FVAL => {:?}", s.parse::<i64>(), s.parse::<f64>());
        Ok(Token::with_data(ttype, s, self.lindex))
    }

    fn next_char(&mut self) -> Option<char> {
        if self.index < self.text.len() {
            self.index += 1;
            self.text.chars().nth(self.index - 1)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.index < self.text.len() {
            self.text.chars().nth(self.index)
        } else {
            None
        }
    }

    pub fn is_done(&self) -> bool { self.index >= self.text.len() }
}

use std::iter::Iterator;

impl Iterator for Lexer {
    type Item = Result<Token, LexerError>;
    fn next(&mut self) -> Option<Result<Token, LexerError>> {
        if !self.is_done() {
            Some(self.next_token())
        } else {
            None
        }
    }
}
