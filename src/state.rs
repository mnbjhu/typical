use std::collections::HashMap;

use tracing::info;

use crate::{
    logic::Logic,
    ty::{args::GeneircArgs, bound::Bound, decl::Decl, impl_::Impl, Type},
};

#[derive(Debug, Clone)]
pub struct TypeSystem {
    pub counter: u32,
    pub decls: HashMap<String, Decl>,
    pub bounds: Vec<Bound>,
    pub impls: Vec<Impl>,
    pub type_vars: HashMap<u32, Type>,
    pub generics: GeneircArgs,
    pub goal: Logic,
}

impl TypeSystem {
    pub fn new() -> Self {
        Self {
            counter: 0,
            decls: HashMap::new(),
            impls: Vec::new(),
            type_vars: HashMap::new(),
            bounds: Vec::new(),
            generics: GeneircArgs::default(),
            goal: Logic::True,
        }
    }

    pub fn add_decl(&mut self, decl: Decl) {
        info!("Adding decl: {}", decl.name);
        self.decls.insert(decl.name.to_string(), decl);
    }

    pub fn add_generic(&mut self, name: String) {
        info!("Adding generic: {}", name);
        self.generics.push(name);
    }

    pub fn clear_generics(&mut self) {
        self.generics = GeneircArgs::default();
    }

    pub fn add_impl(&mut self, impl_: Impl) {
        info!("Adding impl: {}", impl_);
        self.impls.push(impl_);
    }

    pub fn new_type_var(&mut self) -> u32 {
        let id = self.counter;
        self.counter += 1;
        self.type_vars.insert(id, Type::Free);
        id
    }

    pub fn resolve(&self, type_var: u32) -> Option<Type> {
        match self.type_vars.get(&type_var).unwrap() {
            Type::Free => None,
            ty => Some(ty.clone()),
        }
    }

    pub fn add_goal(&mut self, goal: Logic) {
        let goal = match &self.goal {
            Logic::True => goal,
            Logic::AllOf(existing) => Logic::AllOf(
                existing
                    .iter()
                    .cloned()
                    .chain(std::iter::once(goal))
                    .collect(),
            ),
            _ => Logic::AllOf(vec![self.goal.clone(), goal]),
        };
        self.goal = goal.reduce(self, true);
    }
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self::new()
    }
}
