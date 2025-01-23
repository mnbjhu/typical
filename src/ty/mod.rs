use std::collections::HashMap;

use crate::state::TypeSystem;

pub mod args;
pub mod bound;
pub mod decl;
pub mod impl_;
pub mod inst;
pub mod is_bound;
pub mod is_exactly;
pub mod path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Named {
    pub name: String,
    pub args: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Named(Named),
    Generic(String),
    Var(u32),
    Free,
}

impl Type {
    pub fn parameterise(&self, params: &HashMap<String, Type>) -> Type {
        match self {
            Type::Named(named) => Type::Named(named.parameterise(params)),
            Type::Generic(name) => params[name].clone(),
            Type::Var(_) => self.clone(),
            Type::Free => panic!("Cannot parameterise free type"),
        }
    }

    pub fn resolve(&self, state: &TypeSystem) -> Type {
        match self {
            Type::Named(Named { name: id, args }) => Type::Named(Named {
                name: id.to_string(),
                args: args.iter().map(|arg| arg.resolve(state)).collect(),
            }),
            Type::Var(id) => state.resolve(*id).unwrap_or_else(|| self.clone()),
            Type::Free => panic!("Cannot resolve free type"),
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
