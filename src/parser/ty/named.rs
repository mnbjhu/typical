use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    select,
    span::SimpleSpan,
    IterParser as _, Parser,
};

use crate::{
    parser::lexer::Token,
    state::TypeSystem,
    ty::{Named, Type},
};

pub fn named_parser<'a, I>(
    ty: impl Parser<'a, I, Type, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone,
) -> impl Parser<'a, I, Named, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let name = select! {
        Token::Ident(name) => name,
    };
    let args = ty
        .separated_by(just(Token::Comma))
        .collect()
        .delimited_by(just(Token::LBacket), just(Token::RBacket));
    name.then(args.or_not()).map(|(name, args)| Named {
        name,
        args: args.unwrap_or_default(),
    })
}
