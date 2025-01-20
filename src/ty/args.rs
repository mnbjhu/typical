use std::fmt::Display;

use super::bound::Bound;

#[derive(Debug, Clone)]
pub struct GeneircArgs {
    pub args: Vec<String>,
    pub bounds: Vec<Bound>,
}

impl GeneircArgs {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            bounds: Vec::new(),
        }
    }
}

impl Default for GeneircArgs {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for GeneircArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.args.is_empty() {
            return Ok(());
        }
        write!(f, "[")?;
        for (index, arg) in self.args.iter().enumerate() {
            if index != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        Ok(())
    }
}
