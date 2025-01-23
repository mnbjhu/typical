use crate::state::TypeSystem;

use super::{impl_::Impl, Named};

impl Named {
    pub fn paths_to_sub_ty(&self, other: &Named, ts: &TypeSystem) -> Vec<Vec<Impl>> {
        if self.name == other.name {
            return vec![vec![]];
        }
        ts.impls
            .iter()
            .filter(|impl_| impl_.from.name == self.name)
            .flat_map(|impl_| {
                let mut paths = impl_.to.paths_to_sub_ty(other, ts);
                for path in paths.iter_mut() {
                    path.insert(0, impl_.clone());
                }
                paths
            })
            .collect()
    }
}
