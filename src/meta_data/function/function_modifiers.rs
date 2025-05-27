use crate::{meta_data::soul_names::{NamesTypeModifiers, SOUL_NAMES}};
use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct FunctionModifiers: u8 {
        const Default = 0b0000_0001;
        const Const   = 0b0000_0010;
        const Literal = 0b0000_0100;
        const Static  = 0b0000_1000;
    }
}

impl FunctionModifiers {
    pub fn from_str(string: &str) -> Self {
        match string {
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Constent) => FunctionModifiers::Const,
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Literal) => FunctionModifiers::Literal,
            val if val == SOUL_NAMES.get_name(NamesTypeModifiers::Static) => FunctionModifiers::Static,
            _ => FunctionModifiers::Default,
        }
    }

    pub fn to_str(&self) -> &'static str {
        if self.contains(FunctionModifiers::Default) {
            ""
        }
        else if self.contains(FunctionModifiers::Const) {
            SOUL_NAMES.get_name(NamesTypeModifiers::Constent)
        }
        else if self.contains(FunctionModifiers::Literal) {
            SOUL_NAMES.get_name(NamesTypeModifiers::Literal)
        }
        else if self.contains(FunctionModifiers::Static) {
            SOUL_NAMES.get_name(NamesTypeModifiers::Static)
        }
        else {
            ""
        }
    }
}