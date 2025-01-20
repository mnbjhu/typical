use stmt::Stmt;

use crate::state::TypeSystem;

pub mod res;
pub mod stmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Logic {
    OneOf(Vec<Logic>),
    AllOf(Vec<Logic>),
    Stmt(Stmt),
    True,
    False,
}

impl Logic {
    pub fn is_blocked(&self, state: &TypeSystem) -> bool {
        match self {
            Logic::OneOf(items) | Logic::AllOf(items) => {
                items.iter().all(|item| item.is_blocked(state))
            }
            Logic::Stmt(stmt) => stmt.is_blocked(state),
            Logic::True | Logic::False => false,
        }
    }

    pub fn reduce(&self, state: &mut TypeSystem, infer: bool) -> Option<Logic> {
        match self {
            Logic::OneOf(_) => todo!(),
            Logic::AllOf(_) => todo!(),
            Logic::Stmt(_) => todo!(),
            Logic::True | Logic::False => None,
        }
    }
}
