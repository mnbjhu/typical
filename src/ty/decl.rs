use core::fmt;
use std::fmt::Display;

use crate::ty::args::GeneircArgsExt as _;

use super::{args::GeneircArgs, bound::Bound};

#[derive(Debug, Clone)]
pub struct Decl {
    pub name: String,
    pub args: GeneircArgs,
    pub bounds: Vec<Bound>,
}

impl Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.bounds.is_empty() {
            write!(
                f,
                "{}{} where {}",
                self.name,
                self.args.get_string(),
                self.bounds
                    .iter()
                    .map(Bound::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            write!(f, "{}{}", self.name, self.args.get_string())
        }
    }
}
