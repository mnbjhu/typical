use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    recursive::recursive,
    select,
    span::SimpleSpan,
    Parser,
};
use named::named_parser;

use crate::{state::TypeSystem, ty::Type};

use super::lexer::Token;

pub mod named;

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
        type_var_parser()
            .or(wildcard)
            .or(named_parser(ty).map_with(|named, extra| {
                let state: &mut SimpleState<TypeSystem> = extra.state();
                if state.generics.contains(&named.name) {
                    Type::Generic(named.name)
                } else {
                    Type::Named(named)
                }
            }))
    })
}
pub fn type_var_parser<'a, I>(
) -> impl Parser<'a, I, Type, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    select! {
        Token::TypeVar(var) => var,
    }
    .validate(|var, e, emitter| {
        let span = e.span();
        let state: &mut SimpleState<TypeSystem> = e.state();
        if state.type_vars.contains_key(&var) {
            Type::Var(var)
        } else {
            emitter.emit(Rich::custom(
                span,
                format!("unknown type variable ${var}, creating a new one.",),
            ));
            let var = state.new_type_var();
            Type::Var(var)
        }
    })
}
