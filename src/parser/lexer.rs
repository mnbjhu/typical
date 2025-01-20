use std::fmt::Display;

use chumsky::{
    input::{Input as _, Stream, ValueInput},
    span::SimpleSpan,
};
use logos::Logos;

#[derive(Logos, Clone, PartialEq, Debug)]
#[logos(skip "[ \t\n]+")]
pub enum Token {
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("[")]
    LBacket,
    #[token("]")]
    RBacket,
    #[token("_")]
    Underscore,
    #[token(";")]
    Semi,
    #[regex("-+")]
    Sep,
    #[token("impl")]
    Impl,
    #[token("for")]
    For,
    #[regex("[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
    Error,
}

pub fn lex(input: &str) -> impl ValueInput<'_, Token = Token, Span = SimpleSpan> {
    let token_iter = Token::lexer(input)
        .spanned()
        // Convert logos errors into tokens. We want parsing to be recoverable and not fail at the lexing stage, so
        // we have a dedicated `Token::Error` variant that represents a token error that was previously encountered
        .map(|(tok, span)| match tok {
            // Turn the `Range<usize>` spans logos gives us into chumsky's `SimpleSpan` via `Into`, because it's easier
            // to work with
            Ok(tok) => (tok, span.into()),
            Err(()) => (Token::Error, span.into()),
        });

    // Turn the token iterator into a stream that chumsky can use for things like backtracking
    Stream::from_iter(token_iter).map((0..input.len()).into(), |(t, s): (_, _)| (t, s))
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::LBacket => write!(f, "["),
            Token::RBacket => write!(f, "]"),
            Token::Underscore => write!(f, "_"),
            Token::Semi => write!(f, ";"),
            Token::Sep => write!(f, "-"),
            Token::Impl => write!(f, "impl"),
            Token::For => write!(f, "for"),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Error => write!(f, "error"),
        }
    }
}
