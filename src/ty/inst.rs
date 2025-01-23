use std::collections::HashMap;

use tracing::info;

use crate::state::TypeSystem;

use super::{Named, Type};

impl Type {
    pub fn inst(&self, state: &mut TypeSystem) -> Type {
        match self {
            Type::Named(named) => {
                let mut params: HashMap<String, Type> = HashMap::new();
                let decl = state.decls.get(&named.name).unwrap().clone();
                let mut args = if decl.args.len() < named.args.len() {
                    info!("Too many arguments for {}, clipping!", named.name);
                    named.args[..decl.args.len()].to_vec()
                } else {
                    named.args.clone()
                };
                let missing = decl.args.len() - named.args.len();
                for _ in 0..missing {
                    args.push(Type::Var(state.new_type_var()));
                }
                for (param, arg) in decl.args.iter().zip(args.iter()) {
                    params.insert(param.clone(), arg.clone());
                }
                let bounds = decl.bounds.iter().map(|b| b.parameterise(&params));
                state.bounds.extend(bounds);
                Type::Named(Named {
                    name: named.name.clone(),
                    args,
                })
            }
            Type::Generic(name) => panic!(
                "Cannot instantiate generic type {} which doesn't have named parent",
                name
            ),
            Type::Var(_) => self.clone(),
            Type::Free => panic!("Cannot instantiate free type"),
        }
    }
}
