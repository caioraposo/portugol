use portugol::evaluator;
use portugol::lexer::Lexer;
use portugol::object::Environment;
use portugol::parser::Parser;
use portugol::transpiler::Transpiler;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::process;
use std::rc::Rc;

fn main() {
    let filename = env::args().nth(1).expect("Arquivo de input não inserido");
    let contents = fs::read_to_string(filename).expect("Erro ao ler arquivo");

    let mut parser = Parser::new(Lexer::new(contents));

    let program = parser.parse_program();
    if !parser.errors().is_empty() {
        println!("ERROR: foi encontrado o seguinte erro de parse:");
        let error = parser.errors().first();
        println!("\t{:?}", error);
        process::exit(1);
    }
    // for debug
    //println!("{}", program);

    // For evaluator
    let env = Rc::new(RefCell::new(Environment::new()));

    match evaluator::eval(&program, Rc::clone(&env)) {
        Ok(_obj) => {}
        Err(err) => {
            println!("ERRO SEMÂNTICO: {}", err);
            process::exit(1);
        }
    };

    let transpiler = Transpiler::new(program);
    transpiler.transpile();
}
