// Generated using gen_files.rb ver 0.0.1
// DO NOT MODIFY, AS IT WILL BE DELETED WHEN REGENERATED.

// Copyright 2018 Uche Okwo
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy of
//  this software and associated documentation files (the "Software"), to deal in
//  the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software,
//  and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all copies
// or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED,
// INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
// OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use parser::{Literal as PLiteral, Stat, Type, Arguments};
use std::collections::HashMap;
use lexer::Token;


#[derive(Clone, Debug)]
pub enum Expr {
  NewObject(NewObject),
Literal(Literal),
Unary(Unary),
Binary(Binary),
Grouping(Grouping),
Function(Function),
Block(Block),
FunctionCall(FunctionCall),
}

impl Expr {
  pub fn is_newobject(&self) -> bool {
    match self {
      &Expr::NewObject(..) => true,
      _ => false,
    }
  }

  pub fn as_newobject(&self) -> Option<NewObject> {
    match self {
      &Expr::NewObject(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_literal(&self) -> bool {
    match self {
      &Expr::Literal(..) => true,
      _ => false,
    }
  }

  pub fn as_literal(&self) -> Option<Literal> {
    match self {
      &Expr::Literal(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_unary(&self) -> bool {
    match self {
      &Expr::Unary(..) => true,
      _ => false,
    }
  }

  pub fn as_unary(&self) -> Option<Unary> {
    match self {
      &Expr::Unary(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_binary(&self) -> bool {
    match self {
      &Expr::Binary(..) => true,
      _ => false,
    }
  }

  pub fn as_binary(&self) -> Option<Binary> {
    match self {
      &Expr::Binary(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_grouping(&self) -> bool {
    match self {
      &Expr::Grouping(..) => true,
      _ => false,
    }
  }

  pub fn as_grouping(&self) -> Option<Grouping> {
    match self {
      &Expr::Grouping(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_function(&self) -> bool {
    match self {
      &Expr::Function(..) => true,
      _ => false,
    }
  }

  pub fn as_function(&self) -> Option<Function> {
    match self {
      &Expr::Function(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_block(&self) -> bool {
    match self {
      &Expr::Block(..) => true,
      _ => false,
    }
  }

  pub fn as_block(&self) -> Option<Block> {
    match self {
      &Expr::Block(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_functioncall(&self) -> bool {
    match self {
      &Expr::FunctionCall(..) => true,
      _ => false,
    }
  }

  pub fn as_functioncall(&self) -> Option<FunctionCall> {
    match self {
      &Expr::FunctionCall(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  
}


#[derive(Clone, Debug)]
pub struct NewObject {
  pub map: HashMap<String, Expr>,
  
}

impl NewObject {
  pub fn new_naked(map: HashMap<String, Expr>,) -> NewObject {
    NewObject {
        map,
      
    }
  }

  pub fn boxed_new_naked(map: HashMap<String, Expr>,) -> Box<NewObject> {
    Box::new(NewObject {
        map,
      
    })
  }

  pub fn new(map: HashMap<String, Expr>,) -> Expr {
    Expr::NewObject(NewObject{
        map,
      
    })
  }

  pub fn boxed_new(map: HashMap<String, Expr>,) -> Box<Expr> {
    Box::new(Expr::NewObject(NewObject {
        map,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct Literal {
  pub literal: PLiteral,
  pub token: Token,
  
}

impl Literal {
  pub fn new_naked(literal: PLiteral,token: Token,) -> Literal {
    Literal {
        literal,
      
        token,
      
    }
  }

  pub fn boxed_new_naked(literal: PLiteral,token: Token,) -> Box<Literal> {
    Box::new(Literal {
        literal,
      
        token,
      
    })
  }

  pub fn new(literal: PLiteral,token: Token,) -> Expr {
    Expr::Literal(Literal{
        literal,
      
        token,
      
    })
  }

  pub fn boxed_new(literal: PLiteral,token: Token,) -> Box<Expr> {
    Box::new(Expr::Literal(Literal {
        literal,
      
        token,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct Unary {
  pub op: Token,
  pub value: Box<Expr>,
  
}

impl Unary {
  pub fn new_naked(op: Token,value: Box<Expr>,) -> Unary {
    Unary {
        op,
      
        value,
      
    }
  }

  pub fn boxed_new_naked(op: Token,value: Box<Expr>,) -> Box<Unary> {
    Box::new(Unary {
        op,
      
        value,
      
    })
  }

  pub fn new(op: Token,value: Box<Expr>,) -> Expr {
    Expr::Unary(Unary{
        op,
      
        value,
      
    })
  }

  pub fn boxed_new(op: Token,value: Box<Expr>,) -> Box<Expr> {
    Box::new(Expr::Unary(Unary {
        op,
      
        value,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct Binary {
  pub op: Token,
  pub lhs: Box<Expr>,
  pub rhs: Box<Expr>,
  
}

impl Binary {
  pub fn new_naked(op: Token,lhs: Box<Expr>,rhs: Box<Expr>,) -> Binary {
    Binary {
        op,
      
        lhs,
      
        rhs,
      
    }
  }

  pub fn boxed_new_naked(op: Token,lhs: Box<Expr>,rhs: Box<Expr>,) -> Box<Binary> {
    Box::new(Binary {
        op,
      
        lhs,
      
        rhs,
      
    })
  }

  pub fn new(op: Token,lhs: Box<Expr>,rhs: Box<Expr>,) -> Expr {
    Expr::Binary(Binary{
        op,
      
        lhs,
      
        rhs,
      
    })
  }

  pub fn boxed_new(op: Token,lhs: Box<Expr>,rhs: Box<Expr>,) -> Box<Expr> {
    Box::new(Expr::Binary(Binary {
        op,
      
        lhs,
      
        rhs,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct Grouping {
  pub expr: Box<Expr>,
  
}

impl Grouping {
  pub fn new_naked(expr: Box<Expr>,) -> Grouping {
    Grouping {
        expr,
      
    }
  }

  pub fn boxed_new_naked(expr: Box<Expr>,) -> Box<Grouping> {
    Box::new(Grouping {
        expr,
      
    })
  }

  pub fn new(expr: Box<Expr>,) -> Expr {
    Expr::Grouping(Grouping{
        expr,
      
    })
  }

  pub fn boxed_new(expr: Box<Expr>,) -> Box<Expr> {
    Box::new(Expr::Grouping(Grouping {
        expr,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct Function {
  pub ret_type: Type,
  pub args: HashMap<String, Type>,
  pub body: Box<Expr>,
  pub is_method: bool,
  
}

impl Function {
  pub fn new_naked(ret_type: Type,args: HashMap<String, Type>,body: Box<Expr>,is_method: bool,) -> Function {
    Function {
        ret_type,
      
        args,
      
        body,
      
        is_method,
      
    }
  }

  pub fn boxed_new_naked(ret_type: Type,args: HashMap<String, Type>,body: Box<Expr>,is_method: bool,) -> Box<Function> {
    Box::new(Function {
        ret_type,
      
        args,
      
        body,
      
        is_method,
      
    })
  }

  pub fn new(ret_type: Type,args: HashMap<String, Type>,body: Box<Expr>,is_method: bool,) -> Expr {
    Expr::Function(Function{
        ret_type,
      
        args,
      
        body,
      
        is_method,
      
    })
  }

  pub fn boxed_new(ret_type: Type,args: HashMap<String, Type>,body: Box<Expr>,is_method: bool,) -> Box<Expr> {
    Box::new(Expr::Function(Function {
        ret_type,
      
        args,
      
        body,
      
        is_method,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct Block {
  pub body: Vec<Stat>,
  
}

impl Block {
  pub fn new_naked(body: Vec<Stat>,) -> Block {
    Block {
        body,
      
    }
  }

  pub fn boxed_new_naked(body: Vec<Stat>,) -> Box<Block> {
    Box::new(Block {
        body,
      
    })
  }

  pub fn new(body: Vec<Stat>,) -> Expr {
    Expr::Block(Block{
        body,
      
    })
  }

  pub fn boxed_new(body: Vec<Stat>,) -> Box<Expr> {
    Box::new(Expr::Block(Block {
        body,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct FunctionCall {
  pub target: Box<Expr>,
  pub args: Arguments,
  pub as_type: Option<Type>,
  
}

impl FunctionCall {
  pub fn new_naked(target: Box<Expr>,args: Arguments,as_type: Option<Type>,) -> FunctionCall {
    FunctionCall {
        target,
      
        args,
      
        as_type,
      
    }
  }

  pub fn boxed_new_naked(target: Box<Expr>,args: Arguments,as_type: Option<Type>,) -> Box<FunctionCall> {
    Box::new(FunctionCall {
        target,
      
        args,
      
        as_type,
      
    })
  }

  pub fn new(target: Box<Expr>,args: Arguments,as_type: Option<Type>,) -> Expr {
    Expr::FunctionCall(FunctionCall{
        target,
      
        args,
      
        as_type,
      
    })
  }

  pub fn boxed_new(target: Box<Expr>,args: Arguments,as_type: Option<Type>,) -> Box<Expr> {
    Box::new(Expr::FunctionCall(FunctionCall {
        target,
      
        args,
      
        as_type,
      
    }))
  }
}


pub trait ExprVisitor<N> {
  fn visit_newobject(&self, newobject: NewObject) -> N;
  fn visit_literal(&self, literal: Literal) -> N;
  fn visit_unary(&self, unary: Unary) -> N;
  fn visit_binary(&self, binary: Binary) -> N;
  fn visit_grouping(&self, grouping: Grouping) -> N;
  fn visit_function(&self, function: Function) -> N;
  fn visit_block(&self, block: Block) -> N;
  fn visit_functioncall(&self, functioncall: FunctionCall) -> N;
  

  fn visit_expr(&self, d: Expr) -> N {
    match d {
      Expr::NewObject(ref d) => self.visit_newobject(d.clone()),
      Expr::Literal(ref d) => self.visit_literal(d.clone()),
      Expr::Unary(ref d) => self.visit_unary(d.clone()),
      Expr::Binary(ref d) => self.visit_binary(d.clone()),
      Expr::Grouping(ref d) => self.visit_grouping(d.clone()),
      Expr::Function(ref d) => self.visit_function(d.clone()),
      Expr::Block(ref d) => self.visit_block(d.clone()),
      Expr::FunctionCall(ref d) => self.visit_functioncall(d.clone()),
      
    }
  }

  fn visit_expr_box(&self, d: Box<Expr>) -> N {
    match &*d {
      &Expr::NewObject(ref d) => self.visit_newobject(d.clone()),
      &Expr::Literal(ref d) => self.visit_literal(d.clone()),
      &Expr::Unary(ref d) => self.visit_unary(d.clone()),
      &Expr::Binary(ref d) => self.visit_binary(d.clone()),
      &Expr::Grouping(ref d) => self.visit_grouping(d.clone()),
      &Expr::Function(ref d) => self.visit_function(d.clone()),
      &Expr::Block(ref d) => self.visit_block(d.clone()),
      &Expr::FunctionCall(ref d) => self.visit_functioncall(d.clone()),
      
    }
  }
}

pub trait ExprMutVisitor<N> {
  fn visit_newobject(&mut self, newobject: NewObject) -> N;
  fn visit_literal(&mut self, literal: Literal) -> N;
  fn visit_unary(&mut self, unary: Unary) -> N;
  fn visit_binary(&mut self, binary: Binary) -> N;
  fn visit_grouping(&mut self, grouping: Grouping) -> N;
  fn visit_function(&mut self, function: Function) -> N;
  fn visit_block(&mut self, block: Block) -> N;
  fn visit_functioncall(&mut self, functioncall: FunctionCall) -> N;
  

  fn visit_expr(&mut self, d: Expr) -> N {
    match d {
      Expr::NewObject(ref d) => self.visit_newobject(d.clone()),
      Expr::Literal(ref d) => self.visit_literal(d.clone()),
      Expr::Unary(ref d) => self.visit_unary(d.clone()),
      Expr::Binary(ref d) => self.visit_binary(d.clone()),
      Expr::Grouping(ref d) => self.visit_grouping(d.clone()),
      Expr::Function(ref d) => self.visit_function(d.clone()),
      Expr::Block(ref d) => self.visit_block(d.clone()),
      Expr::FunctionCall(ref d) => self.visit_functioncall(d.clone()),
      
    }
  }

  fn visit_expr_box(&mut self, d: Box<Expr>) -> N {
    match &*d {
      &Expr::NewObject(ref d) => self.visit_newobject(d.clone()),
      &Expr::Literal(ref d) => self.visit_literal(d.clone()),
      &Expr::Unary(ref d) => self.visit_unary(d.clone()),
      &Expr::Binary(ref d) => self.visit_binary(d.clone()),
      &Expr::Grouping(ref d) => self.visit_grouping(d.clone()),
      &Expr::Function(ref d) => self.visit_function(d.clone()),
      &Expr::Block(ref d) => self.visit_block(d.clone()),
      &Expr::FunctionCall(ref d) => self.visit_functioncall(d.clone()),
      
    }
  }
}
