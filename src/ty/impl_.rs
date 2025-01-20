use core::fmt;
use std::{collections::HashMap, fmt::Display};

use super::{args::GeneircArgs, bound::Bound, Named, Type};

#[derive(Debug, Clone)]
pub struct Impl {
    pub args: GeneircArgs,
    pub from: Named,
    pub to: Named,
}

impl Display for Impl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "impl {}{} for {}", self.args, self.from, self.to)
    }
}

impl Impl {
    pub fn map(&self, ty: &Named) -> Option<(Named, Vec<Bound>)> {
        let mut params = HashMap::new();
        if self.from.imply_generic_params(ty, &mut params) {
            Some((
                self.to.parameterise(&params),
                self.args
                    .bounds
                    .iter()
                    .map(|b| b.parameterise(&params))
                    .collect(),
            ))
        } else {
            None
        }
    }
}

impl Type {
    fn imply_generic_params(&self, from: &Type, params: &mut HashMap<String, Type>) -> bool {
        match (self, from) {
            (Type::Named(this), Type::Named(from)) => this.imply_generic_params(from, params),
            (Type::Generic(id), _) => {
                if let Some(existing) = params.get(id) {
                    if existing != from {
                        return false;
                    }
                    true
                } else {
                    params.insert(id.clone(), from.clone());
                    true
                }
            }
            _ => self == from,
        }
    }
}

impl Named {
    pub fn imply_generic_params(&self, from: &Named, params: &mut HashMap<String, Type>) -> bool {
        if self.name != from.name {
            return false;
        }
        if self.args.len() != from.args.len() {
            return false;
        }
        for (arg, from_arg) in self.args.iter().zip(from.args.iter()) {
            if !arg.imply_generic_params(from_arg, params) {
                return false;
            }
        }
        true
    }
}
