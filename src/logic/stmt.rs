use crate::{state::TypeSystem, ty::Type};

use super::{res::CheckRes, Logic};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    // If 'is' is a type-var resolves to 'Free' then it should block until it is known.
    // If the type-checker is stuck on a type var, it should set it's value to 'Unknown'
    // and emit an error, unblocking the type-checker.
    // If 'ty' is a type-var which is 'Free' then it should be set to 'is'.
    Exactly {
        ty: Type,
        is: Type,
    },
    Extends {
        sub: Type,
        super_: Type,
    },
    HasMember {
        ty: Type,
        member: String,
        member_ty: Type,
    },
}

impl Stmt {
    pub fn is_blocked(&self, state: &TypeSystem) -> bool {
        match self {
            Stmt::Exactly { is, .. } => matches!(is.resolve(state), Type::Var(_)),
            Stmt::Extends { sub, super_ } => {
                matches!(sub.resolve(state), Type::Var(_))
                    || matches!(super_.resolve(state), Type::Var(_))
            }
            Stmt::HasMember { ty, .. } => matches!(ty.resolve(state), Type::Var(_)),
        }
    }

    pub fn reduce(&self, state: &mut TypeSystem, infer: bool) -> Logic {
        match self {
            Stmt::Exactly { ty, is } => ty.is_exactly(is, state, infer),
            Stmt::Extends { sub, super_ } => sub.is_bound_by(super_, state, infer),
            Stmt::HasMember {
                ty,
                member,
                member_ty,
            } => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::Logic,
        state::TypeSystem,
        ty::{args::GeneircArgs, Named, Type},
    };

    impl TypeSystem {
        pub fn list(&self, inner: Type) -> Type {
            Type::Named(Named {
                name: "List".to_string(),
                args: vec![inner],
            })
        }
    }

    pub fn simple_system() -> TypeSystem {
        let mut sys = TypeSystem::new();
        sys.add_decl("String".to_string(), GeneircArgs::default());
        sys.add_decl("Int".to_string(), GeneircArgs::default());
        sys.add_decl("Number".to_string(), GeneircArgs::default());
        sys.add_decl(
            "List".to_string(),
            GeneircArgs {
                args: vec!["T".to_string()],
                ..Default::default()
            },
        );
        sys
    }

    #[test]
    fn test_basic_logic() {
        let mut state = simple_system();
        assert_eq!(state.type_vars.len(), 0);
        let str_ty = state.inst("String");
        let int_ty = state.inst("Int");
        assert_eq!(state.type_vars.len(), 0);
        assert_eq!(str_ty.is_exactly(&str_ty, &mut state, false), Logic::True);

        assert_eq!(str_ty.is_exactly(&int_ty, &mut state, false), Logic::False);

        assert_eq!(str_ty.is_exactly(&int_ty, &mut state, true), Logic::False);
    }
}
