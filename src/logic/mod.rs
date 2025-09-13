use std::fmt::Display;

use stmt::Stmt;
use tracing::info;

use crate::state::TypeSystem;

pub mod pretty;
pub mod stmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Logic {
    OneOf(Vec<Logic>),
    AllOf(Vec<Logic>),
    Stmt(Stmt),
    True,
    False,
}

impl Display for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logic::OneOf(logics) => {
                write!(f, "(")?;
                for (index, logic) in logics.iter().enumerate() {
                    if index != 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{logic}")?;
                }
                write!(f, ")")
            }
            Logic::AllOf(logics) => {
                write!(f, "(")?;
                for (index, logic) in logics.iter().enumerate() {
                    if index != 0 {
                        write!(f, " & ")?;
                    }
                    write!(f, "{logic}")?;
                }
                write!(f, ")")
            }
            Logic::Stmt(stmt) => {
                write!(f, "{stmt}")
            }
            Logic::True => write!(f, "true"),
            Logic::False => write!(f, "false"),
        }
    }
}

impl Logic {
    pub fn reduce(&self, state: &mut TypeSystem, infer: bool) -> Logic {
        let res = match self {
            Logic::OneOf(logics) => {
                let total = logics.len();
                let reduced = logics
                    .iter()
                    .map(|l| l.reduce(state, false))
                    .collect::<Vec<_>>();
                let mut none_of = 0;
                for new in &reduced {
                    if new == &Logic::False {
                        none_of += 1;
                    }
                }
                if none_of == total {
                    Logic::False
                } else if none_of == total - 1 {
                    reduced
                        .iter()
                        .find(|it| it != &&Logic::False)
                        .unwrap()
                        .clone()
                } else {
                    Logic::OneOf(reduced)
                }
            }
            Logic::AllOf(logics) => logics
                .iter()
                .map(|l| l.reduce(state, infer))
                .collect::<Vec<_>>()
                .into(),
            Logic::Stmt(stmt) => stmt.reduce(state, infer),
            Logic::True | Logic::False => self.clone(),
        };
        info!("Reduced {self} to {res}");
        res
    }

    // TODO: Implement this, but also think of XOR the is reserved in OneOf
    // pub fn union(&self, other: &Logic) -> Logic {
    //     match (self, other) {
    //         (Logic::AllOf(a), b) | (b, Logic::AllOf(a)) => {
    //             let mut res = Logic::True;
    //         }
    //         (a, b) if a == b => a.clone(),
    //         (Logic::True, other) | (other, Logic::True) => other.clone(),
    //         _ => Logic::True,
    //     }
    // }
}
