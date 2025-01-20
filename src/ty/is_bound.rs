use crate::{logic::Logic, state::TypeSystem};

use super::Type;

impl Type {
    pub fn is_bound_by(&self, other: &Type, state: &mut TypeSystem, infer: bool) -> Logic {
        match (self, other) {
            (Type::Named(this), Type::Named(other)) => {
                let paths = this.paths_to_sub_ty(other, state);
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
            (a, b) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{logic::Logic, state::TypeSystem};

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
        let int = ts.parse_type("Int").expect("parse_type failed");
        assert_eq!(int.is_bound_by(&int, &mut ts, false), Logic::True);
    }

    #[test]
    fn test_basic() {
        let mut ts = basic_ts();
        let int = ts.parse_type("Int").expect("parse_type failed");
        let number = ts.parse_type("Number").expect("parse_type failed");

        assert_eq!(int.is_bound_by(&number, &mut ts, false), Logic::True);
    }

    #[test]
    fn test_generic() {
        let mut ts = basic_ts();
        let int_list = ts.parse_type("Vec[Int]").expect("parse_type failed");
        let int_iter = ts.parse_type("Iter[Int]").expect("parse_type failed");
        let num_list = ts.parse_type("Vec[Number]").expect("parse_type failed");

        assert_eq!(int_list.is_bound_by(&int_iter, &mut ts, false), Logic::True);
        assert_eq!(
            num_list.is_bound_by(&int_list, &mut ts, false),
            Logic::False
        );
    }
}
