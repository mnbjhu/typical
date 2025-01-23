use std::{collections::HashMap, fmt::Display};

use crate::{
    logic::{stmt::Stmt, Logic},
    ty::Type,
};

#[derive(Debug, Clone)]
pub struct Bound {
    pub sub: Type,
    pub super_: Type,
}

impl Bound {
    pub fn parameterise(&self, params: &HashMap<String, Type>) -> Bound {
        Bound {
            sub: self.sub.parameterise(params),
            super_: self.super_.parameterise(params),
        }
    }
}

impl From<Bound> for Logic {
    fn from(bound: Bound) -> Logic {
        Logic::Stmt(Stmt::Extends {
            sub: bound.sub,
            super_: bound.super_,
        })
    }
}

impl Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.sub, self.super_)
    }
}
