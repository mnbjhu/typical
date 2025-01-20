use std::collections::HashMap;

use crate::state::TypeSystem;

pub mod args;
pub mod bound;
pub mod display;
pub mod impl_;
pub mod is_bound;
pub mod is_exactly;
pub mod parse;
pub mod path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Named {
    pub name: String,
    pub args: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Named(Named),
    Generic(String),
    Var(u32),
}

impl Type {
    pub fn parameterise(&self, params: &HashMap<String, Type>) -> Type {
        match self {
            Type::Named(named) => Type::Named(named.parameterise(params)),
            Type::Generic(id) => params[id].clone(),
            Type::Var(_) => self.clone(),
            _ => self.clone(),
        }
    }

    pub fn resolve(&self, state: &TypeSystem) -> Type {
        match self {
            Type::Named(Named { name: id, args }) => Type::Named(Named {
                name: id.to_string(),
                args: args.iter().map(|arg| arg.resolve(state)).collect(),
            }),
            Type::Var(id) => state.resolve(*id).unwrap_or_else(|| self.clone()),
            _ => self.clone(),
        }
    }
}

impl Named {
    pub fn parameterise(&self, params: &HashMap<String, Type>) -> Named {
        Named {
            name: self.name.to_string(),
            args: self
                .args
                .iter()
                .map(|arg| arg.parameterise(params))
                .collect(),
        }
    }
}
