use crate::meta_data::soul_names::{NamesAssignType, SOUL_NAMES};
use enum_iterator::Sequence;

#[derive(Debug, Sequence, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum AssignType {
    Invalid             = -1,

    Assign              = 0,
    AssignAdd           = 1,
    AssignSub           = 2,
    AssignMul           = 3,
    AssignDiv           = 4,
    AssignModulo        = 5,
    AssignBitAnd        = 6,
    AssignBitOr         = 7,
    AssignBitXor        = 8,

    GetObjectInner      = 9,
    Index               = 10,
}

impl AssignType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "<invalid>" => AssignType::Invalid,
            val if val == SOUL_NAMES.get_name(NamesAssignType::Assign) => AssignType::Assign,
            val if val == SOUL_NAMES.get_name(NamesAssignType::AddAssign) => AssignType::AssignAdd,
            val if val == SOUL_NAMES.get_name(NamesAssignType::SubAssign) => AssignType::AssignSub,
            val if val == SOUL_NAMES.get_name(NamesAssignType::MulAssign) => AssignType::AssignMul,
            val if val == SOUL_NAMES.get_name(NamesAssignType::DivAssign) => AssignType::AssignDiv,
            val if val == SOUL_NAMES.get_name(NamesAssignType::ModuloAssign) => AssignType::AssignModulo,
            val if val == SOUL_NAMES.get_name(NamesAssignType::BitAndAssign) => AssignType::AssignBitAnd,
            val if val == SOUL_NAMES.get_name(NamesAssignType::BitOrAssign) => AssignType::AssignBitOr,
            val if val == SOUL_NAMES.get_name(NamesAssignType::BitXorAssign) => AssignType::AssignBitXor,
            val if val == SOUL_NAMES.get_name(NamesAssignType::GetObjectInner) => AssignType::GetObjectInner,
            val if val == SOUL_NAMES.get_name(NamesAssignType::Index) => AssignType::Index,

            _ => AssignType::Invalid, // Default case for unrecognized strings
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match self {
            AssignType::Assign => SOUL_NAMES.get_name(NamesAssignType::Assign),
            AssignType::AssignAdd => SOUL_NAMES.get_name(NamesAssignType::AddAssign),
            AssignType::AssignSub => SOUL_NAMES.get_name(NamesAssignType::SubAssign),
            AssignType::AssignMul => SOUL_NAMES.get_name(NamesAssignType::MulAssign),
            AssignType::AssignDiv => SOUL_NAMES.get_name(NamesAssignType::DivAssign),
            AssignType::AssignModulo => SOUL_NAMES.get_name(NamesAssignType::ModuloAssign),
            AssignType::AssignBitAnd => SOUL_NAMES.get_name(NamesAssignType::BitAndAssign),
            AssignType::AssignBitOr => SOUL_NAMES.get_name(NamesAssignType::BitOrAssign),
            AssignType::AssignBitXor => SOUL_NAMES.get_name(NamesAssignType::BitXorAssign),
            AssignType::GetObjectInner => SOUL_NAMES.get_name(NamesAssignType::GetObjectInner),
            AssignType::Index => SOUL_NAMES.get_name(NamesAssignType::Index),
            AssignType::Invalid => "<invalid>",
        }
    }

    pub fn can_be_constant(&self) -> bool {
        match self {
            AssignType::Invalid => false,

            AssignType::AssignAdd |
            AssignType::AssignSub |
            AssignType::AssignMul |
            AssignType::AssignDiv |
            AssignType::AssignModulo |
            AssignType::AssignBitAnd |
            AssignType::AssignBitOr |
            AssignType::AssignBitXor => false,

            AssignType::Index |
            AssignType::Assign |
            AssignType::GetObjectInner => true,
        }
    }
}









