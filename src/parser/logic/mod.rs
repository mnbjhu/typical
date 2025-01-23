use bound::bound_parser;
use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::{choice, just},
    recursive::recursive,
    select,
    span::SimpleSpan,
    Parser,
};
use eq::eq_parser;

use crate::{logic::Logic, state::TypeSystem};

use super::lexer::Token;

pub mod bound;
pub mod eq;

pub fn logic_parser<'a, I>(
) -> impl Parser<'a, I, Logic, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    recursive(|logic| {
        let literal = select! {
            Token::True => Logic::True,
            Token::False => Logic::False,
        };
        let atom = choice((
            logic.delimited_by(just(Token::LParen), just(Token::RParen)),
            eq_parser().map(Logic::Stmt),
            bound_parser().map(Logic::from),
            literal,
        ));
        let all = atom
            .clone()
            .foldl(just(Token::And).ignore_then(atom).repeated(), |a, b| {
                if let Logic::AllOf(mut a) = a {
                    a.push(b);
                    Logic::AllOf(a)
                } else {
                    Logic::AllOf(vec![a, b])
                }
            });

        all.clone()
            .foldl(just(Token::Or).ignore_then(all).repeated(), |a, b| {
                if let Logic::OneOf(mut a) = a {
                    a.push(b);
                    Logic::OneOf(a)
                } else {
                    Logic::OneOf(vec![a, b])
                }
            })
    })
}

#[cfg(test)]
mod tests {
    use chumsky::{extra::SimpleState, Parser};

    use crate::{
        logic::{stmt::Stmt, Logic},
        parser::lexer::lex,
        state::TypeSystem,
        ty::{Named, Type},
    };

    use super::logic_parser;

    #[test]
    fn test_true() {
        let input = lex("true");
        let mut state = SimpleState::from(TypeSystem::default());
        let output = logic_parser().parse_with_state(input, &mut state).unwrap();
        assert_eq!(output, Logic::True);
    }

    #[test]
    fn test_false() {
        let input = lex("false");
        let mut state = SimpleState::from(TypeSystem::default());
        let output = logic_parser().parse_with_state(input, &mut state).unwrap();
        assert_eq!(output, Logic::False);
    }

    #[test]
    fn test_exactly() {
        let input = lex("A = B");
        let mut state = SimpleState::from(TypeSystem::default());
        let output = logic_parser().parse_with_state(input, &mut state).unwrap();
        if let Logic::Stmt(Stmt::Exactly { ty, is }) = output {
            assert_eq!(
                ty,
                Type::Named(Named {
                    name: "A".to_string(),
                    args: vec![],
                })
            );

            assert_eq!(
                is,
                Type::Named(Named {
                    name: "B".to_string(),
                    args: vec![],
                })
            );
        } else {
            panic!("Expected Eq");
        }
    }

    #[test]
    fn test_bound() {
        let input = lex("A : B");
        let mut state = SimpleState::from(TypeSystem::default());
        let output = logic_parser().parse_with_state(input, &mut state).unwrap();
        if let Logic::Stmt(Stmt::Extends { sub, super_ }) = output {
            assert_eq!(
                sub,
                Type::Named(Named {
                    name: "A".to_string(),
                    args: vec![],
                })
            );

            assert_eq!(
                super_,
                Type::Named(Named {
                    name: "B".to_string(),
                    args: vec![],
                })
            );
        } else {
            panic!("Expected Bound");
        }
    }

    #[test]
    fn test_all_of() {
        let input = lex("A = B & C = D & E = F");
        let mut state = SimpleState::from(TypeSystem::default());
        let output = logic_parser().parse_with_state(input, &mut state).unwrap();
        if let Logic::AllOf(logics) = output {
            assert_eq!(logics.len(), 3);
            for logic in logics {
                assert!(matches!(logic, Logic::Stmt(Stmt::Exactly { .. })));
            }
        }
    }

    #[test]
    fn test_one_of() {
        let input = lex("A = B | C = D | E = F");
        let mut state = SimpleState::from(TypeSystem::default());
        let output = logic_parser().parse_with_state(input, &mut state).unwrap();
        if let Logic::OneOf(logics) = output {
            assert_eq!(logics.len(), 3);
            for logic in logics {
                assert!(matches!(logic, Logic::Stmt(Stmt::Exactly { .. })));
            }
        }
    }

    #[test]
    fn test_prec() {
        let input = lex("A = B | C = D & E = F");
        let mut state = SimpleState::from(TypeSystem::default());
        let output = logic_parser().parse_with_state(input, &mut state).unwrap();
        if let Logic::OneOf(logics) = output {
            assert_eq!(logics.len(), 2);
            assert!(matches!(logics[0], Logic::Stmt(Stmt::Exactly { .. })));
            if let Logic::AllOf(logics) = &logics[1] {
                assert_eq!(logics.len(), 2);
                for logic in logics {
                    assert!(matches!(logic, Logic::Stmt(Stmt::Exactly { .. })));
                }
            } else {
                panic!("Expected AllOf");
            }
        } else {
            panic!("Expected OneOf");
        }
    }

    #[test]
    fn test_parens() {
        let input = lex("(A = B | C = D) & E = F");
        let mut state = SimpleState::from(TypeSystem::default());
        let output = logic_parser().parse_with_state(input, &mut state).unwrap();
        if let Logic::AllOf(logics) = output {
            assert_eq!(logics.len(), 2);
            if let Logic::OneOf(logics) = &logics[0] {
                assert_eq!(logics.len(), 2);
                for logic in logics {
                    assert!(matches!(logic, Logic::Stmt(Stmt::Exactly { .. })));
                }
            } else {
                panic!("Expected OneOf");
            }
            assert!(matches!(logics[1], Logic::Stmt(Stmt::Exactly { .. })));
        } else {
            panic!("Expected AllOf");
        }
    }
}
