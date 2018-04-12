#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    Identifier, Integer, Float, String, Atom,

    // Keywords
    And, Or, Not, SelfType, //self
    Fun, Group,
    If, Else, While, Do, For,

    // ** future **
    Interface,

    // Type Keywords
    AnyType, VoidType, IntType, StringType, FloatType,

    Dot, Comma, Colon, ColonEqual, ThinArrow, // ->
    ThickArrow, // =>
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Equal, EqualEqual,
    Bang, // !
    BangEqual,
    LessThan, GreaterThan, LessEqual, GreaterEqual, // < > <= >=
    Plus, Minus, Backslash, Asterisk, Pipe,

    //Separator, // Represents a group of whitespace

    EOF
}

#[derive(Clone, Debug)]
pub struct Token {
    t: TokenType,
    data: Option<String>,
    line: usize,
}

impl Token {
    pub fn with_data(t: TokenType, data: String, line: usize) -> Token {
        Token { t, data: Some(data), line }
    }

    pub fn new(t: TokenType, line: usize) -> Token {
        Token { t, line, data: None }
    }

    pub fn token_type(&self) -> TokenType { self.t }
    pub fn line(&self) -> usize { self.line }
    pub fn data(&self) -> String { self.data.clone().unwrap_or(String::new()) }

    pub fn eof(line: usize) -> Token {
        Token {
            t: TokenType::EOF,
            data: None,
            line
        }
    }

    // pub fn separator(line: usize) -> Token {
    //     Token {
    //         t: TokenType::Separator,
    //         data: None,
    //         line
    //     }
    // }
}
