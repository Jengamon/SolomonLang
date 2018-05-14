extern crate fnv;
//extern crate bus;

mod lexer;
mod parser;
mod interp;

// TODO Make interp containt this module
mod var_store;

fn main() {
    use std::{fs::File, io::Read};
    let text = match File::open("test_case.slm") {
        Ok(mut fl) => {
            let mut s = String::new();
            fl.read_to_string(&mut s).unwrap();
            s
        },
        Err(e) => panic!("{}", e)
    };
    let mut lexer = lexer::Lexer::new(text.clone());
    for token in lexer {
        println!("{:?}", token);
    }
    let lexer: Vec<lexer::Token> = lexer::Lexer::new(text).take_while(|x| x.is_ok()).map(|x| x.unwrap()).collect();
    let mut parser = parser::Parser::new(lexer.clone());
    println!("{:p}", &parser);

    for stmt in parser {
        use parser::StatVisitor;
        match stmt {
            Ok(stmt) => println!("{}", parser::LispyPrinter.visit_stat(stmt)),
            Err(e) => panic!("{}", e)
        }
    }
    let parser = parser::Parser::new(lexer);

    // ~~ Parser Interator -> Interpreter ~~

    // Validator / TypeChecker Step

    // Interpretation Step
    let mut interp = interp::Interpreter::new(Default::default());
    interp.submit("test_case", parser.map(|x| x.unwrap()).collect::<Vec<_>>());
}
