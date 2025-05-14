use crate::meta_data::soul_names::{NamesTypeWrapper, SOUL_NAMES};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum TypeWrappers {
    Invalid = -1,
    ConstRef = 0,
    MutRef = 1,
    Pointer = 2,
    Array = 3,
}

impl TypeWrappers {
    pub fn from_str(str: &str) -> TypeWrappers {
        match str {
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef) => TypeWrappers::ConstRef,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) => TypeWrappers::MutRef,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Pointer) => TypeWrappers::Pointer,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Array)  => TypeWrappers::Array,
            _ => TypeWrappers::Invalid,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            TypeWrappers::ConstRef => SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef),
            TypeWrappers::MutRef => SOUL_NAMES.get_name(NamesTypeWrapper::MutRef),
            TypeWrappers::Pointer => SOUL_NAMES.get_name(NamesTypeWrapper::Pointer),
            TypeWrappers::Array => SOUL_NAMES.get_name(NamesTypeWrapper::Array),
            TypeWrappers::Invalid => "<invalid>",
        }
    }

    pub fn is_any_ref(&self) -> bool {
        self == &TypeWrappers::ConstRef || 
        self == &TypeWrappers::MutRef
    }
}

pub const ALL_TYPE_WRAPPERS: [TypeWrappers; 5] = [
    TypeWrappers::Invalid,

    TypeWrappers::ConstRef,
    TypeWrappers::MutRef,
    TypeWrappers::Pointer,
    TypeWrappers::Array,
];




