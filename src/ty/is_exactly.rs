use crate::{
    logic::{stmt::Stmt, Logic},
    state::TypeSystem,
};

use super::{Named, Type};

impl Type {
    pub fn is_exactly(&self, other: &Type, state: &mut TypeSystem, infer: bool) -> Logic {
        match (self, other) {
            (_, Type::Var(id)) => match state.type_vars.get(id).unwrap() {
                Type::Free => Logic::Stmt(Stmt::Exactly {
                    ty: self.clone(),
                    is: other.clone(),
                }),
                ty => self.is_exactly(&ty.clone(), state, infer),
            },
            (Type::Var(id), _) => match state.type_vars.get(id).unwrap() {
                Type::Free => {
                    if infer {
                        state.type_vars.insert(*id, other.clone());
                        Logic::True
                    } else {
                        Logic::Stmt(Stmt::Exactly {
                            ty: self.clone(),
                            is: other.clone(),
                        })
                    }
                }
                ty => ty.clone().is_exactly(other, state, infer),
            },
            (Type::Named(this), Type::Named(other)) => this.is_exactly(other, state, infer),
            (Type::Generic(first), Type::Generic(second)) if first == second => Logic::True,
            _ => Logic::False,
        }
    }
}

impl Named {
    pub fn is_exactly(&self, other: &Named, state: &mut TypeSystem, infer: bool) -> Logic {
        if self.name != other.name {
            return Logic::False;
        }
        if self.args.len() != other.args.len() {
            panic!("Type args length mismatch");
        }
        let mut logics = vec![];
        for (a, b) in self.args.iter().zip(other.args.iter()) {
            match a.is_exactly(b, state, infer) {
                Logic::False => return Logic::False,
                Logic::AllOf(l) => l.into_iter().for_each(|l| logics.push(l)),
                logic => logics.push(logic),
            }
        }
        logics.into()
    }
}

impl From<Vec<Logic>> for Logic {
    fn from(mut logics: Vec<Logic>) -> Self {
        logics.retain(|l| l != &Logic::True);
        if logics.iter().any(|l| l == &Logic::False) {
            Logic::False
        } else {
            match logics.len() {
                0 => Logic::True,
                1 => logics.pop().unwrap(),
                _ => Logic::AllOf(logics),
            }
        }
    }
}
