use bitflags::bitflags;
use crate::meta_data::soul_names::{NamesTypeModifiers, SOUL_NAMES};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct TypeModifiers: u8 {
        const Default = 0b0000_0000;
        const Const = 0b0000_0001;
        const Literal = 0b0000_0010;
        const Volatile = 0b0000_0100;
        const Static = 0b0000_1000;
    }
}

impl TypeModifiers {

    pub fn from_str(str: &str) -> TypeModifiers {
        match str {
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Constent) => TypeModifiers::Const,
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Literal)=> TypeModifiers::Literal,
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Volatile) => TypeModifiers::Volatile,
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Static) => TypeModifiers::Static,
            _ => TypeModifiers::Default,
        }
    }

    pub fn is_mutable(&self) -> bool {
        !(self.contains(TypeModifiers::Const) || self.contains(TypeModifiers::Literal))
    }

    pub fn to_str(&self) -> String {
        let mut string_builder = String::new();

        if self.contains(TypeModifiers::Const) {
            string_builder.push_str(SOUL_NAMES.get_name(NamesTypeModifiers::Constent));
            string_builder.push(' ');
        }
        if self.contains(TypeModifiers::Literal) {
            string_builder.push_str(SOUL_NAMES.get_name(NamesTypeModifiers::Literal));
            string_builder.push(' ');
        }

        if self.contains(TypeModifiers::Volatile) {
            string_builder.push_str(SOUL_NAMES.get_name(NamesTypeModifiers::Volatile));
            string_builder.push(' ');
        }

        if self.contains(TypeModifiers::Static) {
            string_builder.push_str(SOUL_NAMES.get_name(NamesTypeModifiers::Static));
            string_builder.push(' ');
        }

        string_builder
    }
}