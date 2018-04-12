use parser::{Expr, Stat, ExprVisitor, StatVisitor, Literal};
use parser;

pub struct LispyPrinter;

impl ExprVisitor<String> for LispyPrinter {
    fn visit_unary(&self, unary: parser::Unary) -> String {
        format!("({} {})", unary.op.data(), self.visit_expr_box(unary.value))
    }

    fn visit_binary(&self, binary: parser::Binary) -> String {
        format!("({} {} {})", binary.op.data(), self.visit_expr_box(binary.lhs), self.visit_expr_box(binary.rhs))
    }

    fn visit_newobject(&self, no: parser::NewObject) -> String {
        // TODO the actual one
        format!("<{:p}>", &no)
    }

    fn visit_literal(&self, literal: parser::expr::Literal) -> String {
        match literal.literal {
            Literal::Integer(i) => format!("(int {})", i),
            Literal::Long(i) => format!("(lint {})", i),
            Literal::Float(f) => format!("(float {})", f),
            Literal::String(s) => format!("(string {})", s),
            Literal::Atom(a) => format!("(atom {:?})", a),
            Literal::VariableRef(v) => format!("(ref {})", v),
        }
    }

    fn visit_grouping(&self, group: parser::Grouping) -> String {
        format!("(group {})", self.visit_expr_box(group.expr))
    }

    fn visit_function(&self, function: parser::Function) -> String {
        format!("(function => {:?} {:?}) <{}>", function.ret_type, function.args, self.visit_expr_box(function.body))
    }

    fn visit_block(&self, block: parser::Block) -> String {
        format!("[{:?}]", block.body.iter().map(|x| self.visit_stat(x.clone())).collect::<Vec<_>>())
    }

    fn visit_functioncall(&self, functioncall: parser::FunctionCall) -> String {
        format!("(fnc {})", self.visit_expr_box(functioncall.target))
    }
}

impl StatVisitor<String> for LispyPrinter {
    fn visit_assignment(&self, assignment: parser::Assignment) -> String {
        format!("[= {} {}]", self.visit_expr(assignment.lhs), self.visit_expr(assignment.rhs))
    }
    fn visit_modification(&self, modification: parser::Modification) -> String {
        format!("[:= {} {}]", self.visit_expr(modification.lhs), self.visit_expr(modification.rhs))
    }
    fn visit_expression(&self, expression: parser::Expression) -> String {
        format!("[expr {}]", self.visit_expr(expression.expr))
    }
}
