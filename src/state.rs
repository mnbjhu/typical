use std::collections::HashMap;

use tracing::info;

use crate::ty::{args::GeneircArgs, bound::Bound, impl_::Impl, Named, Type};

#[derive(Debug, Clone)]
pub struct TypeSystem {
    pub counter: u32,
    pub decls: HashMap<String, GeneircArgs>,
    pub bounds: Vec<Bound>,
    pub impls: Vec<Impl>,
    pub type_vars: HashMap<u32, Type>,
    pub generics: GeneircArgs,
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
        }
    }

    pub fn add_decl(&mut self, name: String, args: GeneircArgs) {
        info!("Adding decl: {}{}", name, args);
        self.decls.insert(name, args);
    }

    pub fn add_generic(&mut self, name: String) {
        info!("Adding generic: {}", name);
        self.generics.args.push(name);
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
        id
    }

    pub fn inst(&mut self, name: &str) -> Type {
        let args = self.decls.get(name).cloned().expect("inst: decl not found");
        let params = args
            .args
            .clone()
            .iter()
            .map(|name| (name.to_string(), Type::Var(self.new_type_var())))
            .collect::<HashMap<_, _>>();
        let ty = Type::Named(Named {
            name: name.to_string(),
            args: params.values().cloned().collect(),
        });
        self.bounds
            .extend(args.bounds.iter().map(|bound| bound.parameterise(&params)));
        ty
    }

    pub fn resolve(&self, type_var: u32) -> Option<Type> {
        self.type_vars.get(&type_var).cloned()
    }
}

impl Default for TypeSystem {
    fn default() -> Self {
        Self::new()
    }
}
