pub mod def;
pub mod lexer;
pub mod logic;
pub mod repl;
pub mod ty;

use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    span::SimpleSpan,
    IterParser, Parser,
};
use def::{decl::decl_parser, impl_::impl_parser};
use lexer::Token;
use logic::{bound::bound_parser, logic_parser};

use crate::{logic::Logic, state::TypeSystem};

#[derive(Debug)]
pub struct Test {
    pub ts: TypeSystem,
    pub goals: Vec<Logic>,
    pub expected: Logic,
}

pub fn test_parser<'a, I>(
) -> impl Parser<'a, I, Test, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let impl_ = impl_parser();
    let decl = decl_parser();
    let env = impl_
        .or(decl)
        .separated_by(just(Token::Semi))
        .allow_trailing()
        .labelled("env");
    let goals = bound_parser()
        .map(Logic::from)
        .separated_by(just(Token::Semi))
        .allow_trailing()
        .collect()
        .labelled("goals");
    let expected = just(Token::Sep)
        .ignore_then(logic_parser())
        .labelled("expected");
    env.then(just(Token::Sep))
        .ignore_then(goals)
        .then(expected)
        .map_with(|(goals, expected), e| {
            let state: &mut SimpleState<TypeSystem> = e.state();
            Test {
                ts: state.clone(),
                goals,
                expected,
            }
        })
}
