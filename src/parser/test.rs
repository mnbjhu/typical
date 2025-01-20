use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    span::SimpleSpan,
    IterParser, Parser,
};

use crate::{
    logic::{stmt::Stmt, Logic},
    state::TypeSystem,
};

use super::{bound::bound_parser, decl::decl_parser, impl_::impl_parser, lexer::Token};

#[derive(Debug)]
pub struct Test {
    pub ts: TypeSystem,
    pub goals: Vec<Logic>,
}

pub fn test_parser<'a, I>(
) -> impl Parser<'a, I, Test, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let impl_ = impl_parser();
    let decl = decl_parser();
    let env = impl_.or(decl).separated_by(just(Token::Semi));
    let goals = bound_parser()
        .map(|bound| {
            Logic::Stmt(Stmt::Extends {
                sub: bound.ty,
                super_: bound.super_,
            })
        })
        .separated_by(just(Token::Semi))
        .collect();
    env.then(just(Token::Sep))
        .ignore_then(goals)
        .map_with(|goals, e| {
            let state: &mut SimpleState<TypeSystem> = e.state();
            Test {
                ts: state.clone(),
                goals,
            }
        })
}
