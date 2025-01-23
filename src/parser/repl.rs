use chumsky::{
    error::Rich,
    extra::{self, SimpleState},
    input::ValueInput,
    prelude::{choice, just},
    select,
    span::SimpleSpan,
    Parser,
};

use crate::{logic::Logic, state::TypeSystem, ty::Type};

use super::{
    def::{decl::decl_parser, impl_::impl_parser},
    lexer::Token,
    logic::logic_parser,
    ty::type_parser,
};

pub enum ReplCommand {
    Goal(#[allow(dead_code)] Logic),
    Eval(Logic),
    Impl,
    Decl,
    List(ListItem),
    New(Type),
    Resolve(Type),
}

pub enum ListItem {
    Decl,
    Impl(Option<String>),
    Vars,
}

pub fn repl_parser<'a, I>(
) -> impl Parser<'a, I, ReplCommand, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let goal = just(Token::Goal)
        .ignore_then(logic_parser())
        .map(ReplCommand::Goal);
    let eval = just(Token::Eval)
        .ignore_then(logic_parser())
        .map(ReplCommand::Eval);
    let impl_ = impl_parser().map(|_| ReplCommand::Impl);
    let decl = just(Token::Decl)
        .ignore_then(decl_parser())
        .map(|_| ReplCommand::Decl);
    let list = just(Token::List)
        .ignore_then(list_item_parser())
        .map(ReplCommand::List);
    let new = just(Token::New)
        .ignore_then(type_parser())
        .map(ReplCommand::New);
    let resolve = just(Token::Resolve)
        .ignore_then(type_parser())
        .map(ReplCommand::Resolve);
    choice((goal, eval, decl, list, new, impl_, resolve))
}

fn list_item_parser<'a, I>(
) -> impl Parser<'a, I, ListItem, extra::Full<Rich<'a, Token>, SimpleState<TypeSystem>, ()>> + Clone
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    let decl = just(Token::Decl).map(|_| ListItem::Decl);
    let vars = just(Token::Vars).map(|_| ListItem::Vars);
    let ident = select! {
        Token::Ident(ident) => ident,
    };
    let impl_ = just(Token::Impl)
        .ignore_then(ident.or_not())
        .map(ListItem::Impl);

    vars.or(decl).or(impl_)
}
