use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    select,
    span::SimpleSpan,
    Parser,
};

use crate::state::TypeSystem;

use super::{generics::generics_parser, lexer::Token};

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
        .validate(|name, e, _| {
            let state: &mut SimpleState<TypeSystem> = e.state();
            let generics = state.generics.clone();
            state.clear_generics();
            state.add_decl(name, generics)
        })
}

#[cfg(test)]
mod tests {
    use chumsky::{extra::SimpleState, Parser as _};

    use crate::{
        parser::{decl::decl_parser, lexer::lex, ty::type_parser},
        state::TypeSystem,
    };

    #[test]
    fn test_basic_decl() {
        let mut state = SimpleState::from(TypeSystem::new());
        let input = lex("Int");
        decl_parser().parse_with_state(input, &mut state).unwrap();
        assert_eq!(state.decls.len(), 1);
        let decl = state.decls.get("Int").unwrap();
        assert_eq!(decl.args.len(), 0);
    }
}
