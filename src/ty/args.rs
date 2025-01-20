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
