use portugol::lexer::Lexer;
use portugol::token::Token;

fn main() {
    let input = include_str!("../algoritmo_202");

    let mut lexer = Lexer::new(input.into());

    while let token = lexer.next_token() {
        println!("{}", token);
        if token.token == Token::Eof { break; }
        println!();
    }
}
