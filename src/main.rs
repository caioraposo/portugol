use portugol::lexer::Lexer;
use portugol::token::Token;

fn main() {
    let input = include_str!("../para29");

    let mut lexer = Lexer::new(input.into());

    loop {
        let token = lexer.next_token();
        println!("{}", token);
        // Exit if End of File (EOF) is reached
        if token.token == Token::Eof { break; }
    }
}
