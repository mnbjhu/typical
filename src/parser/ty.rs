use std::collections::HashMap;

use chumsky::{
    container::Seq,
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    primitive::just,
    recursive::recursive,
    select,
    span::SimpleSpan,
    IterParser, Parser,
};

use crate::{
    state::TypeSystem,
    ty::{args::GeneircArgs, Named, Type},
};

use super::lexer::Token;

#[derive(Debug, Clone)]
pub struct PState {
    generics: GeneircArgs,
    type_vars: HashMap<u32, Type>,
    counter: u32,
}

impl PState {
    pub fn new() -> Self {
        Self {
            generics: GeneircArgs::new(),
            type_vars: HashMap::new(),
            counter: 0,
        }
    }

    pub fn new_type_var(&mut self) -> Type {
        let id = self.counter;
        self.counter += 1;
        Type::Var(id)
    }
}

pub fn type_parser<'a, I>(
) -> impl Parser<'a, I, Type, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let wildcard = just(Token::Underscore).map_with(|_, e| {
        let state: &mut SimpleState<TypeSystem> = e.state();
        Type::Var(state.new_type_var())
    });

    recursive(|ty| {
        wildcard.or(named_parser(ty).map_with(|named, extra| {
            let state: &mut SimpleState<TypeSystem> = extra.state();
            if state.generics.args.contains(&named.name) {
                Type::Generic(named.name)
            } else {
                Type::Named(named)
            }
        }))
    })
}

pub fn named_parser<'a, I>(
    ty: impl Parser<'a, I, Type, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone,
) -> impl Parser<'a, I, Named, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let name = select! {
        Token::Ident(name) => name,
    };
    let args = ty
        .separated_by(just(Token::Comma))
        .collect()
        .delimited_by(just(Token::LBacket), just(Token::RBacket));
    name.then(args.or_not()).map(|(name, args)| Named {
        name,
        args: args.unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use crate::state::TypeSystem;
}
