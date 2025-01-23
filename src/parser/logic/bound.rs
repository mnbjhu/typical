use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    span::SimpleSpan,
    Parser,
};

use crate::{
    parser::{lexer::Token, ty::type_parser},
    state::TypeSystem,
    ty::bound::Bound,
};

pub fn bound_parser<'a, I>(
) -> impl Parser<'a, I, Bound, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    type_parser()
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|(sub, super_)| Bound { sub, super_ })
}
