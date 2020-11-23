use portugol::evaluator;
use portugol::lexer::Lexer;
use portugol::object::Environment;
use portugol::parser::Parser;
use portugol::transpiler::Transpiler;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;

fn main() {
    let filename = env::args().nth(1).expect("Arquivo de input não inserido");
    let contents = fs::read_to_string(filename).expect("Erro ao ler arquivo");

    let mut parser = Parser::new(Lexer::new(contents));

    let program = parser.parse_program();
    if !parser.errors().is_empty() {
        println!("Rapaaaaiz! Olha só quanto erro!");
        println!(" erros de parse:");
        for error in parser.errors() {
            println!("\t{:?}", error);
        }
        panic!("Não compilado devido a erro(s) no parser");
    }

    // For evaluator
    let env = Rc::new(RefCell::new(Environment::new()));

    match evaluator::eval(&program, Rc::clone(&env)) {
        Ok(_obj) => {}
        Err(err) => {
            panic!("ERROR: {}", err);
        }
    };

    let transpiler = Transpiler::new(program);
    transpiler.transpile();
}
