use deref_derive::{Deref, DerefMut};

use crate::{error::Error, ident::Ident, specialization::SpecGenerics};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Generic {
    pub name: Ident,
}

impl std::fmt::Display for Generic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Generics {
    pub params: Vec<Generic>,
}

impl Generics {
    pub const fn empty() -> Self {
        Self { params: Vec::new() }
    }

    pub fn specialize(&self, specialized: &SpecGenerics) -> Result<SpecGenerics, Error> {
        if self.len() != specialized.len() {
            let err = Error::new(format!("invalid number of generics")).with_note(format!(
                "expected {} generics, found {}",
                self,
                specialized.spec()
            ));

            return Err(err);
        }

        let mut generics = SpecGenerics::empty();

        for generic in self.iter() {
            let ty = specialized.specialize_generic(generic.clone())?;
            generics.push_generic(generic.clone(), ty);
        }

        Ok(generics)
    }
}

impl std::fmt::Display for Generics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }

        write!(f, "<")?;

        let generics: Vec<_> = self.params.iter().map(Generic::to_string).collect();
        write!(f, "{}", generics.join(", "))?;

        write!(f, ">")?;

        Ok(())
    }
}
