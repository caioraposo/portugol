use portugol::lexer::Lexer;
use portugol::token::Token;
use std::env;
use std::fs;

fn main() {
    let filename = env::args().nth(1).expect("Arquivo de input não inserido");

    let contents = fs::read_to_string(filename).expect("Erro ao ler arquivo");

    let mut lexer = Lexer::new(contents.into());

    loop {
        let token = lexer.next_token();
        println!("{}", token);
        // Exit if End of File (EOF) is reached
        if token.token == Token::Eof {
            break;
        }
    }
}
