use deref_derive::{Deref, DerefMut};

use crate::{
    error::Error,
    generics::{Generic, Generics},
    ty::Type,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Spec {
    pub generics: Vec<Type>,
}

impl Spec {
    pub const fn empty() -> Self {
        Self {
            generics: Vec::new(),
        }
    }
}

impl std::fmt::Display for Spec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        write!(f, "<")?;

        let generics: Vec<_> = self.generics.iter().map(Type::to_string).collect();
        write!(f, "{}", generics.join(", "))?;

        write!(f, ">")?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpecGeneric {
    pub generic: Generic,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct SpecGenerics {
    pub generics: Vec<SpecGeneric>,
}

impl SpecGenerics {
    pub const fn empty() -> Self {
        Self {
            generics: Vec::new(),
        }
    }

    pub fn new(generics: &Generics, specialization: Spec) -> Self {
        debug_assert_eq!(generics.len(), specialization.len());

        let mut this = Self::empty();

        for (generic, ty) in generics.iter().zip(specialization.iter()) {
            this.push_generic(generic.clone(), ty.clone());
        }

        this
    }

    pub fn spec(&self) -> Spec {
        let mut specialization = Spec::empty();

        for specialized_generic in self.iter() {
            specialization.push(specialized_generic.ty.clone());
        }

        specialization
    }

    pub fn push_generic(&mut self, generic: Generic, ty: Type) {
        self.generics.push(SpecGeneric { generic, ty });
    }

    pub fn get_generic(&self, generic: &Generic) -> Option<&SpecGeneric> {
        self.generics.iter().find(|g| &g.generic == generic)
    }

    pub fn specialize_generic(&self, generic: Generic) -> Result<Type, Error> {
        let Some(generic) = self.get_generic(&generic) else {
            let err = Error::new(format!("generic '{}' not specialized", generic.name))
                .with_span(generic.span());

            return Err(err);
        };

        Ok(generic.ty.clone())
    }
}
