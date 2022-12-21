use crate::{
    error::Error, generics::Generics, ident::Ident, specialization::SpecGenerics, ty::Type,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Argument {
    pub name: Ident,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Ident,
    pub generics: Generics,
    pub arguments: Vec<Argument>,
    pub return_ty: Type,
}

impl Function {
    pub fn signature(&self, generics: &SpecGenerics) -> Result<Signature, Error> {
        let mut arguments = Vec::with_capacity(self.arguments.len());

        for argument in &self.arguments {
            arguments.push(argument.ty.clone());
        }

        Ok(Signature {
            generics: self.generics.specialize(generics)?,
            arguments,
            return_ty: self.return_ty.clone(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Signature {
    pub generics: SpecGenerics,
    pub arguments: Vec<Type>,
    pub return_ty: Type,
}

impl std::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(")?;

        let arguments: Vec<_> = self.arguments.iter().map(Type::to_string).collect();
        write!(f, "{}", arguments.join(", "))?;

        write!(f, ") -> {}", self.return_ty)
    }
}
