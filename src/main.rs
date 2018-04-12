mod lexer;
mod parser;

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
    let lexer = lexer::Lexer::new(text).take_while(|x| x.is_ok()).map(|x| x.unwrap()).collect();
    let mut parser = parser::Parser::new(lexer);
    println!("{:p}", &parser);

    while !parser.is_done() {
        use parser::StatVisitor;
        match parser.parse_statement() {
            Ok(stmt) => println!("{}", parser::LispyPrinter.visit_stat(stmt)),
            Err(e) => panic!("{}", e)
        }
    }

    // Validator / TypeChecker Step

    // Interpretation Step
}
