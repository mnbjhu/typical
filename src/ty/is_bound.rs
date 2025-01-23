use std::fmt::Display;

use tracing::info;

use crate::{logic::Logic, state::TypeSystem};

use super::{Named, Type};

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Named(named) => write!(f, "{}", named),
            Type::Generic(id) => write!(f, "{}", id),
            Type::Var(id) => write!(f, "${}", id),
            Type::Free => write!(f, "<FREE>"),
        }
    }
}

impl Display for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.args.is_empty() {
            write!(f, "[")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", arg)?;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

impl Type {
    pub fn is_bound_by(&self, other: &Type, state: &mut TypeSystem, infer: bool) -> Logic {
        match (self, other) {
            (Type::Named(this), Type::Named(other)) => {
                let paths = this.paths_to_sub_ty(other, state);
                info!("Found {} paths from {} to {}", paths.len(), this, other);
                let mut logics = vec![];
                for impls in paths {
                    let mut ty = this.clone();
                    for impl_ in impls {
                        let (named, bounds) =
                            impl_.map(&ty).expect("Know to be mapped at this point");
                        logics.extend(bounds.iter().cloned().map(|b| b.into()));
                        ty = named;
                    }
                    logics.push(ty.is_exactly(other, state, infer));
                }

                if logics.is_empty() {
                    Logic::False
                } else if logics.len() == 1 {
                    logics[0].clone()
                } else {
                    Logic::OneOf(logics)
                }
            }
            _ => todo!(),
        }
    }
}
