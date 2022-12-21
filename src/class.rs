use crate::{field::Field, generics::Generics, ident::Ident, method::Method};

#[derive(Clone, Debug, PartialEq)]
pub struct Class {
    pub name: Ident,
    pub generics: Generics,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
}
