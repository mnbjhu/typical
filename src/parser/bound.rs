use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    span::SimpleSpan,
    Parser,
};

use crate::{state::TypeSystem, ty::bound::Bound};

use super::{lexer::Token, ty::type_parser};

pub fn bound_parser<'a, I>(
) -> impl Parser<'a, I, Bound, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    type_parser()
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|(ty, super_)| Bound { ty, super_ })
}

