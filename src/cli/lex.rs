use logos::Logos as _;

use crate::parser::lexer::Token;

pub fn lex(filename: String) {
    let text = std::fs::read_to_string(filename).unwrap();
    Token::lexer(&text).for_each(|tok| {
        println!("{:?}", tok);
    });
}
