use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    span::SimpleSpan,
    IterParser as _, Parser,
};

use crate::{
    parser::{lexer::Token, logic::bound::bound_parser},
    state::TypeSystem,
    ty::bound::Bound,
};

pub fn where_parser<'a, I>(
) -> impl Parser<'a, I, Vec<Bound>, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    just(Token::Where)
        .ignore_then(bound_parser().separated_by(just(Token::Comma)).collect())
        .labelled("where")
}

#[cfg(test)]
mod tests {
    use chumsky::{extra::SimpleState, Parser};

    use crate::{parser::lexer::lex, state::TypeSystem};

    #[test]
    fn test_one() {
        let input = lex("where A: B");
        let mut state = SimpleState::from(TypeSystem::new());
        let result = super::where_parser()
            .parse_with_state(input, &mut state)
            .unwrap();

        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_many() {
        let input = lex("where A: B, C: D, E: F");
        let mut state = SimpleState::from(TypeSystem::new());
        let result = super::where_parser()
            .parse_with_state(input, &mut state)
            .unwrap();

        assert_eq!(result.len(), 3);
    }
}
