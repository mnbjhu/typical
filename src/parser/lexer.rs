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
    #[token("=")]
    Eq,
    #[token("goal")]
    Goal,
    #[token("eval")]
    Eval,
    #[token("resolve")]
    Resolve,
    #[token("decl")]
    Decl,
    #[token("list")]
    List,
    #[token(",")]
    Comma,
    #[token("[")]
    LBacket,
    #[token("]")]
    RBacket,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("_")]
    Underscore,
    #[token(";")]
    Semi,
    #[regex("-+")]
    Sep,
    #[token("vars")]
    Vars,
    #[token("impl")]
    Impl,
    #[token("where")]
    Where,
    #[regex("new")]
    New,
    #[token("for")]
    For,
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[regex("\\$[0-9]+", |lex| lex.slice()[1..].to_string().parse::<u32>().unwrap())]
    TypeVar(u32),
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
            Token::Eq => write!(f, "="),
            Token::Comma => write!(f, ","),
            Token::LBacket => write!(f, "["),
            Token::RBacket => write!(f, "]"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::And => write!(f, "&"),
            Token::Or => write!(f, "|"),
            Token::Underscore => write!(f, "_"),
            Token::Semi => write!(f, ";"),
            Token::Sep => write!(f, "-"),
            Token::Impl => write!(f, "impl"),
            Token::New => write!(f, "new"),
            Token::For => write!(f, "for"),
            Token::List => write!(f, "list"),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Goal => write!(f, "goal"),
            Token::Vars => write!(f, "vars"),
            Token::Eval => write!(f, "eval"),
            Token::Decl => write!(f, "decl"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "true"),
            Token::Error => write!(f, "error"),
            Token::Where => write!(f, "where"),
            Token::Resolve => write!(f, "resolve"),
            Token::TypeVar(var) => write!(f, "${}", var),
        }
    }
}
