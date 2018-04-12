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

use parser::expr::Expr;
use lexer::Token;


#[derive(Clone, Debug)]
pub enum Stat {
  Assignment(Assignment),
Modification(Modification),
Expression(Expression),
}

impl Stat {
  pub fn is_assignment(&self) -> bool {
    match self {
      &Stat::Assignment(..) => true,
      _ => false,
    }
  }

  pub fn as_assignment(&self) -> Option<Assignment> {
    match self {
      &Stat::Assignment(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_modification(&self) -> bool {
    match self {
      &Stat::Modification(..) => true,
      _ => false,
    }
  }

  pub fn as_modification(&self) -> Option<Modification> {
    match self {
      &Stat::Modification(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  pub fn is_expression(&self) -> bool {
    match self {
      &Stat::Expression(..) => true,
      _ => false,
    }
  }

  pub fn as_expression(&self) -> Option<Expression> {
    match self {
      &Stat::Expression(ref t) => Some(t.clone()),
      _ => None,
    }
  }
  
}


#[derive(Clone, Debug)]
pub struct Assignment {
  pub lhs: Expr,
  pub rhs: Expr,
  pub op: Token,
  
}

impl Assignment {
  pub fn new_naked(lhs: Expr,rhs: Expr,op: Token,) -> Assignment {
    Assignment {
        lhs,
      
        rhs,
      
        op,
      
    }
  }

  pub fn boxed_new_naked(lhs: Expr,rhs: Expr,op: Token,) -> Box<Assignment> {
    Box::new(Assignment {
        lhs,
      
        rhs,
      
        op,
      
    })
  }

  pub fn new(lhs: Expr,rhs: Expr,op: Token,) -> Stat {
    Stat::Assignment(Assignment{
        lhs,
      
        rhs,
      
        op,
      
    })
  }

  pub fn boxed_new(lhs: Expr,rhs: Expr,op: Token,) -> Box<Stat> {
    Box::new(Stat::Assignment(Assignment {
        lhs,
      
        rhs,
      
        op,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct Modification {
  pub lhs: Expr,
  pub rhs: Expr,
  pub op: Token,
  
}

impl Modification {
  pub fn new_naked(lhs: Expr,rhs: Expr,op: Token,) -> Modification {
    Modification {
        lhs,
      
        rhs,
      
        op,
      
    }
  }

  pub fn boxed_new_naked(lhs: Expr,rhs: Expr,op: Token,) -> Box<Modification> {
    Box::new(Modification {
        lhs,
      
        rhs,
      
        op,
      
    })
  }

  pub fn new(lhs: Expr,rhs: Expr,op: Token,) -> Stat {
    Stat::Modification(Modification{
        lhs,
      
        rhs,
      
        op,
      
    })
  }

  pub fn boxed_new(lhs: Expr,rhs: Expr,op: Token,) -> Box<Stat> {
    Box::new(Stat::Modification(Modification {
        lhs,
      
        rhs,
      
        op,
      
    }))
  }
}

#[derive(Clone, Debug)]
pub struct Expression {
  pub expr: Expr,
  
}

impl Expression {
  pub fn new_naked(expr: Expr,) -> Expression {
    Expression {
        expr,
      
    }
  }

  pub fn boxed_new_naked(expr: Expr,) -> Box<Expression> {
    Box::new(Expression {
        expr,
      
    })
  }

  pub fn new(expr: Expr,) -> Stat {
    Stat::Expression(Expression{
        expr,
      
    })
  }

  pub fn boxed_new(expr: Expr,) -> Box<Stat> {
    Box::new(Stat::Expression(Expression {
        expr,
      
    }))
  }
}


pub trait StatVisitor<N> {
  fn visit_assignment(&self, assignment: Assignment) -> N;
fn visit_modification(&self, modification: Modification) -> N;
fn visit_expression(&self, expression: Expression) -> N;
fn visit_stat(&self, d: Stat) -> N {
    match d {
      Stat::Assignment(ref d) => self.visit_assignment(d.clone()),
      Stat::Modification(ref d) => self.visit_modification(d.clone()),
      Stat::Expression(ref d) => self.visit_expression(d.clone()),
      
    }
  }

  fn visit_stat_box(&self, d: Box<Stat>) -> N {
    match &*d {
      &Stat::Assignment(ref d) => self.visit_assignment(d.clone()),
      &Stat::Modification(ref d) => self.visit_modification(d.clone()),
      &Stat::Expression(ref d) => self.visit_expression(d.clone()),
      
    }
  }
}

pub trait StatMutVisitor<N> {
  fn visit_assignment(&mut self, assignment: Assignment) -> N;
  fn visit_modification(&mut self, modification: Modification) -> N;
  fn visit_expression(&mut self, expression: Expression) -> N;
  

  fn visit_stat(&mut self, d: Stat) -> N {
    match d {
      Stat::Assignment(ref d) => self.visit_assignment(d.clone()),
      Stat::Modification(ref d) => self.visit_modification(d.clone()),
      Stat::Expression(ref d) => self.visit_expression(d.clone()),
      
    }
  }

  fn visit_stat_box(&mut self, d: Box<Stat>) -> N {
    match &*d {
      &Stat::Assignment(ref d) => self.visit_assignment(d.clone()),
      &Stat::Modification(ref d) => self.visit_modification(d.clone()),
      &Stat::Expression(ref d) => self.visit_expression(d.clone()),
      
    }
  }
}
