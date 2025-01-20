use core::fmt;
use std::fmt::{Display, Formatter};

use thiserror::Error;

use crate::state::TypeSystem;

use super::{args::GeneircArgs, impl_::Impl, Named, Type};

pub struct ParserState<'ts> {
    ts: &'ts mut TypeSystem,
    pub generics: GeneircArgs,
    text: String,
    offset: usize,
}

#[derive(Error, Debug, PartialEq, Eq)]
#[error("Parse error at offset {offset}: {message}")]
pub struct ParseError {
    pub message: ErrorDetail,
    pub offset: usize,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ErrorDetail {
    #[error("Expected {expected} but found {found}",
        expected = one_of.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" or "),
        found = found.map(|it| it.to_string()).unwrap_or("EOF".to_string()))]
    Expected {
        one_of: Vec<Expected>,
        found: Option<char>,
    },
    #[error("Expected {expected} arguments for {name}, got {got}")]
    ArgsMismatch {
        name: String,
        expected: usize,
        got: usize,
    },
    #[error("Decl not found: '{0}'")]
    DeclNotFound(String),
    #[error("Generic '{0}' not found")]
    GenericNotFound(String),
    #[error("Generic '{0}' already defined")]
    GenericAlreadyDefined(String),
    #[error("Decl already defined '{0}'")]
    DeclAlreadyDefined(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expected {
    Char(char),
    Ident,
    Keyword(&'static str),
}

impl Display for Expected {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Expected::Char(c) => write!(f, "'{}'", c),
            Expected::Ident => write!(f, "identifier"),
            Expected::Keyword(k) => write!(f, "'{}'", k),
        }
    }
}

impl<'ts> ParserState<'ts> {
    pub fn add_generic(&mut self, name: String) {
        self.generics.args.push(name);
    }

    fn error<T>(&self, message: ErrorDetail) -> Result<T, ParseError> {
        Err(ParseError {
            message,
            offset: self.offset,
        })
    }

    pub fn new(ts: &'ts mut TypeSystem, text: String) -> Self {
        Self {
            ts,
            generics: GeneircArgs::default(),
            text,
            offset: 0,
        }
    }
    pub fn parse_type(&mut self, check: bool) -> Result<Type, ParseError> {
        if self.current_char() == Some('_') {
            self.offset += 1;
            let var = Type::Var(self.ts.new_type_var());
            Ok(var)
        } else if let Ok(generic) = self.parse_generic_ty() {
            Ok(generic)
        } else {
            self.parse_named_ty(check).map(Type::Named)
        }
    }

    pub fn new_type_var(&mut self) -> Type {
        Type::Var(self.ts.new_type_var())
    }

    fn parse_generic_ty(&mut self) -> Result<Type, ParseError> {
        let before = self.offset;
        let ident = self.parse_ident()?;
        if self.generics.args.contains(&ident) {
            Ok(Type::Generic(ident))
        } else {
            self.offset = before;
            self.error(ErrorDetail::GenericNotFound(ident))
        }
    }

    fn parse_named_ty(&mut self, check: bool) -> Result<Named, ParseError> {
        let ident = self.parse_ident()?;
        let args = self.parse_args(check)?;
        if check {
            if let Some(decl) = self.ts.decls.get(&ident) {
                if decl.args.len() != args.len() {
                    return self.error(ErrorDetail::ArgsMismatch {
                        name: ident,
                        expected: decl.args.len(),
                        got: args.len(),
                    });
                }
            } else {
                return self.error(ErrorDetail::DeclNotFound(ident));
            }
        }
        Ok(Named { name: ident, args })
    }

    fn is_eof(&self) -> bool {
        self.offset >= self.text.len()
    }

    fn parse_generic_arg(&mut self) -> Result<(), ParseError> {
        let name = self.parse_ident()?;
        if self.generics.args.contains(&name) {
            Err(self.error(ErrorDetail::GenericAlreadyDefined(name))?)
        } else {
            self.generics.args.push(name.clone());
            Ok(())
        }
    }

    fn expect(&mut self, c: char) -> Result<(), ParseError> {
        if self.current_char() == Some(c) {
            self.offset += 1;
            Ok(())
        } else {
            self.error(ErrorDetail::Expected {
                one_of: vec![Expected::Char(c)],
                found: self.current_char(),
            })
        }
    }

    fn parse_generics(&mut self) -> Result<(), ParseError> {
        if self.current_char() != Some('[') {
            return self.error(ErrorDetail::Expected {
                one_of: vec![Expected::Char('[')],
                found: self.current_char(),
            });
        }
        self.offset += 1;
        self.skip_ws();
        if self.current_char() == Some(']') {
            self.offset += 1;
            return Ok(());
        }
        loop {
            self.parse_generic_arg()?;
            self.skip_ws();
            if self.current_char() == Some(']') {
                self.offset += 1;
                break;
            }
            self.expect(',')?;
            self.skip_ws();
        }
        Ok(())
    }

    fn current_char(&self) -> Option<char> {
        self.text.chars().nth(self.offset)
    }

    fn parse_ident(&mut self) -> Result<String, ParseError> {
        let mut ident = String::new();
        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.offset += 1;
            } else {
                break;
            }
        }
        if ident.is_empty() {
            self.error(ErrorDetail::Expected {
                one_of: vec![Expected::Ident],
                found: self.current_char(),
            })
        } else {
            Ok(ident)
        }
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.text.chars().nth(self.offset) {
            if c.is_whitespace() {
                self.offset += 1;
            } else {
                break;
            }
        }
    }

    fn parse_args(&mut self, check: bool) -> Result<Vec<Type>, ParseError> {
        let mut found = Vec::new();
        if self.text.chars().nth(self.offset) != Some('[') {
            return Ok(found);
        }
        self.offset += 1;
        loop {
            if let Ok(ty) = self.parse_type(check) {
                found.push(ty);
            }
            Self::skip_ws(self);
            if self.text.chars().nth(self.offset) == Some(']') {
                self.offset += 1;
                break;
            } else if self.text.chars().nth(self.offset) == Some(',') {
                self.offset += 1;
            } else {
                return self.error(ErrorDetail::Expected {
                    one_of: vec![Expected::Char(','), Expected::Char(']')],
                    found: self.text.chars().nth(self.offset),
                });
            }
        }
        Ok(found)
    }

    pub fn parse_impl(&mut self) -> Result<Impl, ParseError> {
        self.parse_generics()?;
        self.skip_ws();
        let to = self.parse_named_ty(true)?;
        self.skip_ws();
        let for_ = self.parse_ident()?;
        if for_ != "for" {
            return self.error(ErrorDetail::Expected {
                one_of: vec![Expected::Keyword("for")],
                found: Some(for_.chars().next().unwrap()),
            });
        }
        self.skip_ws();
        let from = self.parse_named_ty(true)?;
        let args = self.generics.clone();
        self.generics = GeneircArgs::default();
        Ok(Impl { from, to, args })
    }
}

impl TypeSystem {
    pub fn parse_decl(&mut self, text: &str) -> Result<(), ParseError> {
        let mut state = ParserState::new(self, text.to_string());
        let name = state.parse_ident()?;
        if state.ts.decls.contains_key(&name) {
            return state.error(ErrorDetail::DeclAlreadyDefined(name));
        }
        state.skip_ws();
        if !state.is_eof() {
            state.parse_generics()?;
        }
        let generics = state.generics.clone();
        state.generics = GeneircArgs::default();
        state.ts.add_decl(name, generics);
        Ok(())
    }

    pub fn parse_impl(&mut self, text: &str) -> Result<(), ParseError> {
        let mut state = ParserState::new(self, text.to_string());
        let impl_ = state.parse_impl()?;
        self.add_impl(impl_);
        Ok(())
    }

    pub fn parse_type(&mut self, text: &str) -> Result<Type, ParseError> {
        let mut state = ParserState::new(self, text.to_string());
        state.parse_type(true)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        state::TypeSystem,
        ty::{
            parse::{ErrorDetail, Expected, ParseError},
            Named, Type,
        },
    };

    fn basic_ts() -> TypeSystem {
        let mut ts = TypeSystem::new();
        ts.parse_decl("Int").expect("parse_decl failed");
        ts.parse_decl("String").expect("parse_decl failed");
        ts.parse_decl("Number").expect("parse_decl failed");
        ts.parse_decl("List[T]").expect("parse_decl failed");
        ts
    }

    #[test]
    fn test_create_basic_decl() {
        let mut ts = TypeSystem::new();
        assert_eq!(ts.parse_decl("Int"), Ok(()));
        assert_eq!(
            ts.parse_decl(""),
            Err(ParseError {
                message: ErrorDetail::Expected {
                    one_of: vec![Expected::Ident],
                    found: None,
                },
                offset: 0,
            })
        );
        assert_eq!(
            ts.parse_decl("Int"),
            Err(ParseError {
                message: ErrorDetail::DeclAlreadyDefined("Int".to_string()),
                offset: 3,
            })
        );
        let int = ts.decls.get("Int").expect("decl not found");
        assert_eq!(int.args.len(), 0);
        assert_eq!(int.bounds.len(), 0);
    }

    #[test]
    fn test_create_generic_decl() {
        let mut ts = TypeSystem::new();
        assert_eq!(ts.parse_decl("List[T]"), Ok(()));
        let hello = ts.decls.get("List").expect("decl not found");
        assert_eq!(hello.args.len(), 1);
        assert_eq!(hello.bounds.len(), 0);
    }

    #[test]
    fn test_create_type() {
        let mut ts = basic_ts();
        assert_eq!(
            ts.parse_type("String"),
            Ok(Type::Named(Named {
                name: "String".to_string(),
                args: vec![],
            }))
        );
    }

    #[test]
    fn test_create_generic_type() {
        let mut ts = basic_ts();
        assert_eq!(
            ts.parse_type("List[Int]"),
            Ok(Type::Named(Named {
                name: "List".to_string(),
                args: vec![Type::Named(Named {
                    name: "Int".to_string(),
                    args: vec![],
                })],
            }))
        );
    }

    #[test]
    fn test_create_impl() {
        let mut ts = basic_ts();
        assert_eq!(ts.parse_impl("[] Number for Int"), Ok(()));
        assert_eq!(ts.impls.len(), 1);
        let impl_ = &ts.impls[0];
        assert_eq!(
            impl_.to,
            Named {
                name: "Number".to_string(),
                args: vec![]
            }
        );

        assert_eq!(
            impl_.from,
            Named {
                name: "Int".to_string(),
                args: vec![]
            }
        );
        assert_eq!(impl_.args.args.len(), 0);
    }
}
