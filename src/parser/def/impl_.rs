use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    span::SimpleSpan,
    Parser,
};

use crate::{
    parser::{
        lexer::Token,
        ty::{named::named_parser, type_parser},
    },
    state::TypeSystem,
    ty::impl_::Impl,
};

use super::{generics::generics_parser, where_::where_parser};

pub fn impl_parser<'a, I>(
) -> impl Parser<'a, I, (), extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    just(Token::Impl)
        .then(generics_parser())
        .ignore_then(named_parser(type_parser()))
        .then_ignore(just(Token::For))
        .then(named_parser(type_parser()))
        .then(where_parser().or_not())
        .validate(|((to, from), where_), e, _| {
            let state: &mut SimpleState<TypeSystem> = e.state();
            let args = state.generics.clone();
            state.clear_generics();
            let impl_ = Impl {
                args,
                from,
                to,
                bounds: where_.unwrap_or_default(),
            };
            state.add_impl(impl_);
        })
}
