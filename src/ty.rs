use std::hash::{Hash, Hasher};

use crate::{
    data::ClassId, float::Float, function::Signature, int::Int, path::Path, specialization::Spec,
};

#[derive(Clone, Debug)]
pub struct ClassType {
    pub id: ClassId,
    pub path: Path,
    pub spec: Spec,
}

impl PartialEq for ClassType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.spec == other.spec
    }
}

impl Eq for ClassType {}

impl Hash for ClassType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.spec.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Void,
    Bool,
    Int(Int),
    Float(Float),
    Pointer(Box<Type>),
    Function(Box<Signature>),
    Array(Box<Type>, usize),
    Class(ClassType),
}

impl Type {
    pub fn pointer(ty: Type) -> Self {
        Self::Pointer(Box::new(ty))
    }

    pub fn function(signature: Signature) -> Self {
        Self::Function(Box::new(signature))
    }

    pub fn array(ty: Type, size: usize) -> Self {
        Self::Array(Box::new(ty), size)
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Bool => write!(f, "bool"),
            Self::Int(int) => write!(f, "{}", int),
            Self::Float(float) => write!(f, "{}", float),
            Self::Pointer(ty) => write!(f, "*{}", ty),
            Self::Function(signature) => write!(f, "{}", signature),
            Self::Array(ty, size) => write!(f, "[{}; {}]", ty, size),
            Self::Class(class) => write!(f, "{}", class.path),
        }
    }
}

impl Type {
    pub const fn i8() -> Self {
        Self::Int(Int::i8())
    }

    pub const fn i16() -> Self {
        Self::Int(Int::i16())
    }

    pub const fn i32() -> Self {
        Self::Int(Int::i32())
    }

    pub const fn i64() -> Self {
        Self::Int(Int::i64())
    }

    pub const fn i128() -> Self {
        Self::Int(Int::i128())
    }

    pub const fn isize() -> Self {
        Self::Int(Int::isize())
    }

    pub const fn u8() -> Self {
        Self::Int(Int::u8())
    }

    pub const fn u16() -> Self {
        Self::Int(Int::u16())
    }

    pub const fn u32() -> Self {
        Self::Int(Int::u32())
    }

    pub const fn u64() -> Self {
        Self::Int(Int::u64())
    }

    pub const fn u128() -> Self {
        Self::Int(Int::u128())
    }

    pub const fn usize() -> Self {
        Self::Int(Int::usize())
    }

    pub const fn f16() -> Self {
        Self::Float(Float::f16())
    }

    pub const fn f32() -> Self {
        Self::Float(Float::f32())
    }

    pub const fn f64() -> Self {
        Self::Float(Float::f64())
    }
}
