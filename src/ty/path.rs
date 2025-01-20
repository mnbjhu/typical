use crate::state::TypeSystem;

use super::{impl_::Impl, Named, Type};

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

#[cfg(test)]
mod tests {

    use crate::{state::TypeSystem, ty::Named};

    fn basic_ts() -> TypeSystem {
        let mut ts = TypeSystem::new();
        ts.parse_decl("Int").expect("parse_decl failed");
        ts.parse_decl("Number").expect("parse_decl failed");
        ts.parse_decl("Vec[T]").expect("parse_decl failed");
        ts.parse_decl("Iter[T]").expect("parse_decl failed");
        ts.parse_impl("[] Number for Int")
            .expect("parse_impl failed");

        ts.parse_impl("[T] Iter[T] for Vec[T]")
            .expect("parse_impl failed");
        ts
    }

    #[test]
    fn test_eq() {
        let mut ts = basic_ts();
        let int = Named {
            name: "Int".to_string(),
            args: vec![],
        };

        let paths = int.paths_to_sub_ty(&int, &ts);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 0);
    }

    #[test]
    fn test_basic() {
        let ts = basic_ts();
        let int = Named {
            name: "Int".to_string(),
            args: vec![],
        };
        let number = Named {
            name: "Number".to_string(),
            args: vec![],
        };

        let paths = int.paths_to_sub_ty(&number, &ts);

        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_generic() {
        let mut ts = basic_ts();
        let iter = ts.parse_type("Iter[Int]").expect("parse_type failed");
        let vec = ts.parse_type("Vec[Int]").expect("parse_type failed");
        let paths = vec.as_named().paths_to_sub_ty(iter.as_named(), &ts);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].len(), 1);
        let impl_ = &paths[0][0];
        let ty = impl_.map(vec.as_named());
        if let Some((ty, bounds)) = ty {
            assert_eq!(&ty, iter.as_named());
            assert_eq!(bounds.len(), 0);
        } else {
            panic!("Expected Some");
        }
    }
}

impl Type {
    pub fn as_named(&self) -> &Named {
        match self {
            Type::Named(n) => n,
            _ => panic!("Expected Named"),
        }
    }
}
