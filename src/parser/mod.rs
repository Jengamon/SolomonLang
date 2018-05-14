mod expr;
mod stat;
mod pretty_print;

use lexer::{Token, TokenType};
use std::collections::HashMap;

pub use self::expr::{Literal as ELiteral, *};
pub use self::stat::*;
pub use self::pretty_print::LispyPrinter;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Atom(String);

// #[derive(Clone, Debug)]
// struct AtomTable {
//     register: Vec<String>,
// }
//
// impl AtomTable {
//     fn new() -> AtomTable {
//         AtomTable { register: vec![] }
//     }
//
//     fn create_atom(&mut self, n: String) -> Atom {
//         match self.register.iter().position(|x| *x == n) {
//             Some(i) => Atom(i),
//             None => {
//                 self.register.push(n);
//                 Atom(self.register.len() - 1)
//             }
//         }
//     }
//
//     fn atom_name(&mut self, a: Atom) -> String {
//         self.register[a.0].clone()
//     }
// }

#[derive(Clone, Debug)]
pub enum Literal {
    Integer(i32),
    Long(i64), // NOTE: Try to do the Pythony thing, and use this once an integer is large (or small) enough.
    Float(i64),
    String(String),
    Atom(Atom),
    VariableRef(String),
}

#[derive(Clone, Debug)]
pub enum Type {
    Int,
    Float,
    String,
    Atom(Atom), // Type <atomname> only has the value in it of $<atomname>
    Any,
    Void,
}

#[derive(Clone, Debug)]
pub enum Arguments {
    NoArg,
    SingleArg(Box<Expr>),
    Args(HashMap<String, Expr>)
}

type ParserError = String;
#[derive(Clone, Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

type Precedence = i32;
type PrefixParslet = Box<Fn(&mut Parser, Token) -> Result<Expr, ParserError>>;
type InfixParslet = (Box<Fn(&mut Parser, Precedence, Token, Expr) -> Result<Expr, ParserError>>, Precedence);

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            index: 0,
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.index < self.tokens.len() {
            self.index += 1;
            Some(self.tokens[self.index-1].clone())
        } else {
            None
        }
    }

    pub fn peek(&self, off: usize) -> Option<Token> {
        if self.index + off < self.tokens.len() {
            Some(self.tokens[self.index+off].clone())
        } else {
            None
        }
    }

    pub fn expect(&mut self, tt: TokenType) -> Result<Token, ParserError> {
        match self.next() {
            Some(tok) => if tok.token_type() == tt {
                Ok(tok)
            } else {
                Err(format!("Mismatch: expected {:?}, got {:?} on line {}", tt, tok.token_type(), tok.line()))
            },
            None => Err(format!("Mismatch: expected {:?}, got EOF", tt))
        }
    }

    pub fn jump_if(&mut self, tt: TokenType) -> Option<Token> {
        match self.peek(0) {
            Some(tok) => if tok.token_type() == tt {
                self.next()
            } else {
                None
            },
            None => None
        }
    }

    pub fn dispose<F>(&mut self, prec: F) where F: Fn(TokenType) -> bool {
        while let Some(tok) = self.peek(0) {
            if !prec(tok.token_type()) { break } else { self.next(); }
        }
    }

    // We use Recursive descent to parse statements, but pratt parsers to do expressions

    fn parse_type(&mut self) -> Result<Type, ParserError> {
        if let Some(tok) = self.next() {
            match tok.token_type() {
                TokenType::IntType => Ok(Type::Int),
                t => Err(format!("No type corresponding to {:?} on line {}", t, tok.line()))
            }
        } else {
            Err(format!("No more tokens @ aprse_type"))
        }
    }

    // Expressions
    pub fn get_prefix(t: TokenType) -> Option<PrefixParslet> {
        match t {
            TokenType::Integer => Some(Box::new(|_, t: Token| {
                let long = t.data().parse::<i64>();
                let short = t.data().parse::<i32>();
                match (long, short) {
                    (Ok(long), Ok(short)) => Ok(expr::Literal::new(Literal::Integer(short), t)),
                    (Ok(long), Err(_)) => Ok(expr::Literal::new(Literal::Long(long), t)),
                    (Err(_), Ok(_)) => unreachable!(),
                    (Err(_), Err(_)) => Err(format!("Number overflows: {} at line {}", t.data(), t.line()))
                }
            })),
            TokenType::Identifier => Some(Box::new(|_, t: Token| {
                Ok(expr::Literal::new(Literal::VariableRef(t.data()), t))
            })),
            TokenType::LBrace => Some(Box::new(|p: &mut Parser, t: Token| {
                let mut b = HashMap::new();

                while let Some(tok) = p.peek(0) {
                    if tok.token_type() == TokenType::RBrace { break }
                    let id = p.expect(TokenType::Identifier)?;
                    // DO type parsing here
                    p.expect(TokenType::Equal)?;
                    let expr = p.parse_expression(0)?;
                    match b.get(&id.data()) {
                        Some(_) => Err(format!("Has duplicate key {} at {}", id.data(), id.line()))?,
                        None => b.insert(id.data(), expr)
                    };
                    p.jump_if(TokenType::Comma);
                }
                p.expect(TokenType::RBrace)?;

                Ok(expr::NewObject::new(b))
            })),
            TokenType::LParen => Some(Box::new(|p: &mut Parser, t:Token| {
                let expr = p.parse_expression(0)?;
                p.expect(TokenType::RParen)?;
                Ok(expr::Grouping::new(Box::new(expr)))
            })),
            TokenType::Fun => Some(Box::new(|p: &mut Parser, t: Token| {
                let mut _type = Type::Void;
                let mut args = HashMap::new();
                if p.jump_if(TokenType::LParen).is_some() {
                    _type = p.parse_type()?;
                    p.expect(TokenType::RParen)?;
                }

                p.expect(TokenType::LBrace)?;
                let is_method = p.jump_if(TokenType::SelfType).is_some();
                if is_method { p.expect(TokenType::Comma)?; }
                loop {
                    let nam = p.expect(TokenType::Identifier)?;
                    p.expect(TokenType::Colon)?;
                    let expr = p.parse_type()?;
                    args.insert(nam.data(), expr); // TODO Check for duplicates
                    if p.jump_if(TokenType::Comma).is_none() { break }
                }
                p.expect(TokenType::RBrace)?;

                let body = p.parse_expression(0)?;

                Ok(expr::Function::new(_type, args, Box::new(body), is_method))
            })),
            TokenType::LBracket => Some(Box::new(|p: &mut Parser, t: Token| {
                let mut bdy = vec![];
                while let Some(tok) = p.peek(0) {
                    match tok.token_type() {
                        TokenType::RBracket => break,
                        _ => bdy.push(p.parse_statement()?)
                    }
                }
                p.next(); // Eat ]

                Ok(expr::Block::new(bdy))
            })),
            _ => None
        }
    }

    pub fn get_infix(t: TokenType) -> Option<InfixParslet> {
        let binary = |rpc: bool| {
            Box::new(move |p: &mut Parser, r: Precedence, t: Token, l: Expr| {
                let r = if rpc {
                    p.parse_expression(r - 1)
                } else {
                    p.parse_expression(r)
                }?;
                Ok(expr::Binary::new(t, Box::new(l), Box::new(r)))
            })
        };
        match t {
            TokenType::Dot => Some((binary(false), 50)),
            TokenType::Plus => Some((binary(false), 30)),
            TokenType::Minus => Some((binary(false), 30)),
            TokenType::Backslash => Some((binary(false), 35)),
            TokenType::Asterisk => Some((binary(false), 35)),
            TokenType::ThinArrow => Some((Box::new(|p: &mut Parser, r: Precedence, t: Token, l: Expr| {
                if p.jump_if(TokenType::LParen).is_none() { // Full message style
                    let mut args = HashMap::new();
                    loop {
                        let name = p.expect(TokenType::Identifier)?;
                        p.expect(TokenType::Colon)?;
                        let value = p.parse_expression(0)?;
                        args.insert(name.data(), value); // TODO Check for duplicates
                        if p.jump_if(TokenType::Comma).is_none() { break }
                    }
                    Ok(expr::FunctionCall::new(Box::new(l), Arguments::Args(args), None))
                } else {
                    let arg = if p.jump_if(TokenType::RParen).is_none() {
                        let ret = Arguments::SingleArg(Box::new(p.parse_expression(0)?));
                        p.expect(TokenType::RParen)?;
                        ret
                    } else {
                        Arguments::NoArg
                    };
                    Ok(expr::FunctionCall::new(Box::new(l), arg, None))
                }
            }), 49)),
            _ => None
        }
    }

    pub fn parse_expression(&mut self, p: Precedence) -> Result<Expr, ParserError> {
        if let Some(token) = self.next() {
            let mut left = match Parser::get_prefix(token.token_type()) {
                Some(func) => func(self, token.clone())?,
                None => Err(format!("No prefix parser exists for {:?} at line {}", token.token_type(), token.line()))?
            };
            while let Some(tok) = self.peek(0) {
                if let Some((func, prec)) = Parser::get_infix(tok.clone().token_type()) {
                    if p >= prec { break }
                    self.next();
                    left = func(self, prec, tok.clone(), left)?;
                } else { break }
            }
            Ok(left)
        } else {
            Err(format!("No more tokens to parse"))
        }
    }

    fn binary_statement<F>(&mut self, left: Expr, op: Token, c: F) -> Result<Stat, ParserError>
        where F: Fn(Expr, Expr, Token) -> Stat {
        let right = self.parse_expression(0)?;
        Ok(c(left, right, op))
    }

    pub fn parse_statement(&mut self) -> Result<Stat, ParserError> {
        match self.peek(0) {
            Some(tok) => {
                if tok.token_type() == TokenType::EOF { Err(format!("EOF {}", self.is_done()))? }
                let left = self.parse_expression(0)?;
                match self.peek(0) {
                    Some(tok) => match tok.token_type() {
                        TokenType::Equal => {self.next();self.binary_statement(left, tok, stat::Assignment::new)},
                        TokenType::ColonEqual => {self.next();self.binary_statement(left, tok, stat::Modification::new)},
                        tt => Ok(stat::Expression::new(left))
                    },
                    None => Ok(stat::Expression::new(left))
                }
            },
            None => Err("EOF".into())
        }
    }

    pub fn is_done(&self) -> bool { self.index >= self.tokens.iter().filter(|t| t.token_type() != TokenType::EOF).count() }
}

use std::iter::Iterator;
impl Iterator for Parser {
    type Item = Result<Stat, ParserError>;
    fn next(&mut self) -> Option<Result<Stat, ParserError>> {
        if self.is_done() {
            None
        } else {
            Some(self.parse_statement())
        }
    }
}
