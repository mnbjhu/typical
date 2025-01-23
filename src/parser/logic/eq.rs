use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    span::SimpleSpan,
    Parser,
};

use crate::{
    logic::stmt::Stmt,
    parser::{lexer::Token, ty::type_parser},
    state::TypeSystem,
};

pub fn eq_parser<'a, I>(
) -> impl Parser<'a, I, Stmt, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    type_parser()
        .then_ignore(just(Token::Eq))
        .then(type_parser())
        .map(|(ty, is)| Stmt::Exactly { ty, is })
}
