use chumsky::{
    container::Seq,
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    recursive::recursive,
    select,
    span::SimpleSpan,
    IterParser, Parser,
};

use crate::{
    state::TypeSystem,
    ty::{Named, Type},
};

use super::lexer::Token;

pub fn type_parser<'a, I>(
) -> impl Parser<'a, I, Type, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let wildcard = just(Token::Underscore).map_with(|_, e| {
        let state: &mut SimpleState<TypeSystem> = e.state();
        Type::Var(state.new_type_var())
    });

    recursive(|ty| {
        wildcard.or(named_parser(ty).map_with(|named, extra| {
            let state: &mut SimpleState<TypeSystem> = extra.state();
            if state.generics.args.contains(&named.name) {
                Type::Generic(named.name)
            } else {
                Type::Named(named)
            }
        }))
    })
}

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
