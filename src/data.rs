use std::ops::{Index, IndexMut};

use deref_derive::{Deref, DerefMut};

use crate::{class::Class, function::Function, module::Module};

macro_rules! vec_map {
    ($id:ident, $map:ident<$ty:ty>) => {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $id(usize);

        #[derive(Clone, Debug, Default, PartialEq, Deref, DerefMut)]
        pub struct $map {
            map: Vec<$ty>,
        }

        impl $map {
            pub const fn new() -> Self {
                Self { map: Vec::new() }
            }

            pub fn get(&self, id: $id) -> &$ty {
                &self.map[id.0]
            }

            pub fn get_mut(&mut self, id: $id) -> &mut $ty {
                &mut self.map[id.0]
            }

            pub fn push(&mut self, value: $ty) -> $id {
                let id = $id(self.map.len());
                self.map.push(value);
                id
            }

            pub fn iter(&self) -> impl Iterator<Item = &$ty> {
                self.map.iter()
            }

            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut $ty> {
                self.map.iter_mut()
            }
        }

        impl Index<$id> for $map {
            type Output = $ty;

            fn index(&self, id: $id) -> &Self::Output {
                self.get(id)
            }
        }

        impl IndexMut<$id> for $map {
            fn index_mut(&mut self, id: $id) -> &mut Self::Output {
                self.get_mut(id)
            }
        }
    };
}

vec_map!(ModuleId, Modules<Module>);
vec_map!(FunctionId, GenericFunctions<Function>);
vec_map!(ClassId, Classes<Class>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Data {
    pub modules: Modules,
    pub classes: Classes,
    pub functions: GenericFunctions,
}
