use crate::{
    logic::{stmt::Stmt, Logic},
    state::TypeSystem,
};

use super::{Named, Type};

impl Type {
    pub fn is_exactly(&self, other: &Type, state: &mut TypeSystem, infer: bool) -> Logic {
        match (self, other) {
            (_, Type::Var(_)) => Logic::Stmt(Stmt::Exactly {
                ty: self.clone(),
                is: other.clone(),
            }),
            (Type::Var(id), _) => {
                if infer {
                    state.type_vars.insert(*id, other.clone());
                    Logic::True
                } else {
                    Logic::Stmt(Stmt::Exactly {
                        ty: self.clone(),
                        is: other.clone(),
                    })
                }
            }
            (Type::Named(this), Type::Named(other)) => this.is_exactly(other, state, infer),
            (Type::Generic(first), Type::Generic(second)) if first == second => Logic::True,
            _ => Logic::False,
        }
    }
}

impl Named {
    pub fn is_exactly(&self, other: &Named, state: &mut TypeSystem, infer: bool) -> Logic {
        if self.name != other.name {
            return Logic::False;
        }
        if self.args.len() != other.args.len() {
            panic!("Type args length mismatch");
        }
        let mut logics = vec![];
        for (a, b) in self.args.iter().zip(other.args.iter()) {
            match a.is_exactly(b, state, infer) {
                Logic::False => return Logic::False,
                Logic::AllOf(l) => l.into_iter().for_each(|l| logics.push(l)),
                logic => logics.push(logic),
            }
        }
        logics.into()
    }
}

impl From<Vec<Logic>> for Logic {
    fn from(mut logics: Vec<Logic>) -> Self {
        logics.retain(|l| l != &Logic::True);
        if logics.iter().any(|l| l == &Logic::False) {
            Logic::False
        } else {
            match logics.len() {
                0 => Logic::True,
                1 => logics.pop().unwrap(),
                _ => Logic::AllOf(logics),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::{stmt::Stmt, Logic},
        state::TypeSystem,
        ty::{Named, Type},
    };

    fn basic_ts() -> TypeSystem {
        let mut ts = TypeSystem::new();
        ts.parse_decl("Int").expect("parse_decl failed");
        ts.parse_decl("String").expect("parse_decl failed");
        ts.parse_decl("Number").expect("parse_decl failed");
        ts.parse_decl("List[T]").expect("parse_decl failed");
        ts
    }

    #[test]
    fn test_basic() {
        let mut state = basic_ts();
        let int = state.parse_type("Int").expect("parse_type failed");
        let string = state.parse_type("String").expect("parse_type failed");
        assert_eq!(int.is_exactly(&int, &mut state, true), Logic::True);
        assert_eq!(string.is_exactly(&string, &mut state, true), Logic::True);
        assert_eq!(int.is_exactly(&string, &mut state, true), Logic::False);
    }

    #[test]
    fn test_type_var() {
        let mut state = basic_ts();
        let int = state.parse_type("Int").expect("parse_type failed");
        let var = Type::Var(state.new_type_var());
        assert_eq!(var.is_exactly(&int, &mut state, true), Logic::True);
        let resolved = var.resolve(&state);
        assert_eq!(resolved, int);
    }

    #[test]
    fn test_named_args() {
        let mut state = basic_ts();
        let list_int = state.parse_type("List[Int]").expect("parse_type failed");
        let list_string = state.parse_type("List[String]").expect("parse_type failed");
        assert_eq!(
            list_int.is_exactly(&list_int, &mut state, true),
            Logic::True
        );
        assert_eq!(
            list_int.is_exactly(&list_string, &mut state, true),
            Logic::False
        );
    }

    #[test]
    fn test_named_with_type_var() {
        let mut state = basic_ts();
        let list_int = state.parse_type("List[Int]").expect("parse_type failed");
        let some_list = state.parse_type("List[_]").expect("parse_type failed");
        assert_eq!(
            list_int.is_exactly(&some_list, &mut state, true),
            Logic::Stmt(Stmt::Exactly {
                ty: Type::Named(Named {
                    name: "Int".to_string(),
                    args: vec![]
                }),
                is: Type::Var(0)
            })
        );
    }

    #[test]
    fn test_imply_generic_param() {
        let mut state = basic_ts();
        let list_int = state.parse_type("List[Int]").expect("parse_type failed");
        let some_list = state.parse_type("List[_]").expect("parse_type failed");
        assert_eq!(
            some_list.is_exactly(&list_int, &mut state, true),
            Logic::True
        );
        assert_eq!(some_list.resolve(&state), list_int);
    }
}
