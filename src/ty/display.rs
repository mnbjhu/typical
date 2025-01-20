use std::fmt::Display;

use super::{Named, Type};

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Named(Named { name, args }) => {
                write!(f, "{}", name)?;
                if !args.is_empty() {
                    write!(f, "[")?;
                    for (i, arg) in args.iter().enumerate() {
                        if i != 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", arg)?;
                    }
                    write!(f, "]")?;
                }
                Ok(())
            }
            Type::Generic(id) => write!(f, "{}", id),
            Type::Var(id) => write!(f, "${}", id),
        }
    }
}
