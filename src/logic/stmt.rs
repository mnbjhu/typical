use std::fmt::Display;

use crate::{state::TypeSystem, ty::Type};

use super::Logic;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    // If 'is' is a type-var resolves to 'Free' then it should block until it is known.
    // If the type-checker is stuck on a type var, it should set it's value to 'Unknown'
    // and emit an error, unblocking the type-checker.
    // If 'ty' is a type-var which is 'Free' then it should be set to 'is'.
    Exactly {
        ty: Type,
        is: Type,
    },
    Extends {
        sub: Type,
        super_: Type,
    },
    #[allow(dead_code)]
    HasMember {
        ty: Type,
        member: String,
        member_ty: Type,
    },
}

impl Stmt {
    pub fn reduce(&self, state: &mut TypeSystem, infer: bool) -> Logic {
        match self {
            Stmt::Exactly { ty, is } => ty.is_exactly(is, state, infer),
            Stmt::Extends { sub, super_ } => sub.is_bound_by(super_, state, infer),
            Stmt::HasMember { .. } => todo!(),
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Exactly { ty, is } => {
                write!(f, "{ty} = {is}")
            }
            Stmt::Extends { sub, super_ } => {
                write!(f, "{super_}: {sub}")
            }
            Stmt::HasMember {
                ty,
                member,
                member_ty,
            } => {
                write!(f, "{ty} has member '{member}' with type {member_ty}")
            }
        }
    }
}
