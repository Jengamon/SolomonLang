use parser::{Expr, Stat, ExprMutVisitor, StatMutVisitor,
    Assignment, Modification, Expression, Literal, ELiteral, NewObject, Unary, Binary,
    Grouping, Function,  Block, FunctionCall};
use lexer::{Token, TokenType};
use std::{collections::HashMap, convert::AsRef, fmt::Debug, cmp::PartialEq, ptr, rc::Rc};

pub struct Options {
    strict_mode: bool
}

impl Default for Options {
    fn default() -> Options {
        Options {
            strict_mode: false,
        }
    }
}

pub struct Interpreter {
    opts: Options,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolomonObj {
    members: HashMap<String, SolomonObject>
}

pub type SolomonFunc = Fn(&mut Interpreter, Vec<SolomonObject>) -> SolomonObject;
pub trait SolomonCallable: Debug { // For now, structure must be debuggable
    fn call(&self, &mut Interpreter, Vec<SolomonObject>) -> SolomonObject;
    fn arity(&self) -> usize;
}
impl PartialEq for SolomonCallable {
    fn eq(&self, o: &SolomonCallable) -> bool {
        ptr::eq(self, o)
    }
}

// TODO create a dummy box that wraps a type SolomonFunc into a SolomonCallable

#[derive(Clone, Debug, PartialEq)]
pub enum SolomonObject {
    Null,
    Integer(i64),
    Float(f64),
    Atom(usize),
    String(String),
    Object(SolomonObj),
    Function(Rc<Box<SolomonCallable>>),
}

impl Interpreter {
    pub fn new(opts: Options) -> Interpreter {
        Interpreter {
            opts
        }
    }

    // Change from Option to Result when impl is ready for errors instead of Nones
    pub fn submit<S: AsRef<str>>(&mut self, mod_name: S, p: Vec<Stat>) -> Option<SolomonObject> {
        for stat in p {
            self.visit_stat(stat);
        }
        None
    }
}

impl StatMutVisitor<()> for Interpreter {
    fn visit_assignment(&mut self, assignment: Assignment) -> () {
        //println!("INTERP> {:?}", assignment);
        let rs = self.visit_expr(assignment.rhs.clone());
        println!("Value> {:?} to {:?}", "lhs", rs);
    }
    fn visit_modification(&mut self, modification: Modification) -> () {}
    fn visit_expression(&mut self, expression: Expression) -> () {}
}

impl ExprMutVisitor<()> for Interpreter {
    fn visit_newobject(&mut self, newobject: NewObject) -> () {}
    fn visit_literal(&mut self, literal: ELiteral) -> () {}
    fn visit_unary(&mut self, unary: Unary) -> () {}
    fn visit_binary(&mut self, binary: Binary) -> () {}
    fn visit_grouping(&mut self, grouping: Grouping) -> () {}
    fn visit_function(&mut self, function: Function) -> () {}
    fn visit_block(&mut self, block: Block) -> () {}
    fn visit_functioncall(&mut self, functioncall: FunctionCall) -> () {}
}
