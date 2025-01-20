use super::lexer::Token;
use crate::state::TypeSystem;
use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    select,
    span::SimpleSpan,
    IterParser, Parser,
};

pub fn generics_parser<'a, I>(
) -> impl Parser<'a, I, (), extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let ident = select! {
        Token::Ident(ident) => ident,
    }
    .validate(|name, extra, _| {
        let state: &mut SimpleState<TypeSystem> = extra.state();
        state.add_generic(name);
    });
    ident
        .separated_by(just(Token::Comma))
        .collect()
        .delimited_by(just(Token::LBacket), just(Token::RBacket))
}
