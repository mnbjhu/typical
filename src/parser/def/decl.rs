use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    select,
    span::SimpleSpan,
    Parser,
};

use crate::{parser::lexer::Token, state::TypeSystem, ty::decl::Decl};

use super::{generics::generics_parser, where_::where_parser};

pub fn decl_parser<'a, I>(
) -> impl Parser<'a, I, (), extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let ident = select! {
        Token::Ident(ident) => ident,
    };
    ident
        .then_ignore(generics_parser().or_not())
        .then(where_parser().or_not())
        .validate(|(name, where_), e, _| {
            let state: &mut SimpleState<TypeSystem> = e.state();
            let args = state.generics.clone();
            state.clear_generics();
            let decl = Decl {
                name,
                args,
                bounds: where_.unwrap_or_default(),
            };
            state.add_decl(decl)
        })
}
