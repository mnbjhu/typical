use std::collections::HashMap;

use crate::{
    logic::{stmt::Stmt, Logic},
    ty::Type,
};

#[derive(Debug, Clone)]
pub struct Bound {
    pub ty: Type,
    pub super_: Type,
}

impl Bound {
    pub fn parameterise(&self, params: &HashMap<String, Type>) -> Bound {
        Bound {
            ty: self.ty.parameterise(params),
            super_: self.super_.parameterise(params),
        }
    }
}

impl From<Bound> for Logic {
    fn from(bound: Bound) -> Logic {
        Logic::Stmt(Stmt::Extends {
            sub: bound.ty,
            super_: bound.super_,
        })
    }
}
