use crate::meta_data::soul_names::{NamesOperator, SOUL_NAMES};
use enum_iterator::Sequence;
use once_cell::sync::Lazy;

#[derive(Debug, Sequence, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum OperatorType {
    Invalid             = -1,

    Not                 = 0,
	Equals              = 1,
	NotEquals           = 2,
	IsSmaller           = 3,
	IsSmallerEquals     = 4,
	IsBigger            = 5,
	IsBiggerEquals      = 6,

	Add                 = 7,
	Sub                 = 8,
	Mul                 = 9,
	Div                 = 10,
	Modulo              = 11,
	Pow                 = 12,
	Root                = 13,
	Log                 = 14,

	BitWiseOr           = 15,
	BitWiseAnd          = 16,
	BitWiseXor          = 17,
	LogicalOr           = 18,
	LogicalAnd          = 19,

    Increment           = 20,
    Decrement           = 21,
}

impl OperatorType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "<invalid>" => OperatorType::Invalid,
            val if val == SOUL_NAMES.get_name(NamesOperator::Not) => OperatorType::Not,
            val if val == SOUL_NAMES.get_name(NamesOperator::Equals) => OperatorType::Equals,
            val if val == SOUL_NAMES.get_name(NamesOperator::NotEquals) => OperatorType::NotEquals,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsSmaller) => OperatorType::IsSmaller,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsSmallerEquals) => OperatorType::IsSmallerEquals,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsBigger) => OperatorType::IsBigger,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsBiggerEquals) => OperatorType::IsBiggerEquals,

            val if val == SOUL_NAMES.get_name(NamesOperator::Addition) => OperatorType::Add,
            val if val == SOUL_NAMES.get_name(NamesOperator::Increment) => OperatorType::Increment,
            val if val == SOUL_NAMES.get_name(NamesOperator::Decrement) => OperatorType::Decrement,
            val if val == SOUL_NAMES.get_name(NamesOperator::Subtract) => OperatorType::Sub,
            val if val == SOUL_NAMES.get_name(NamesOperator::Multiple) => OperatorType::Mul,
            val if val == SOUL_NAMES.get_name(NamesOperator::Divide) => OperatorType::Div,
            val if val == SOUL_NAMES.get_name(NamesOperator::Modulo) => OperatorType::Modulo,
            val if val == SOUL_NAMES.get_name(NamesOperator::Power) => OperatorType::Pow,
            val if val == SOUL_NAMES.get_name(NamesOperator::Root) => OperatorType::Root,
            val if val == SOUL_NAMES.get_name(NamesOperator::Logarithm) => OperatorType::Log,

            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseOr) => OperatorType::BitWiseOr,
            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseAnd) => OperatorType::BitWiseAnd,
            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseXor) => OperatorType::BitWiseXor,
            val if val == SOUL_NAMES.get_name(NamesOperator::LogicalOr) => OperatorType::LogicalOr,
            val if val == SOUL_NAMES.get_name(NamesOperator::LogicalAnd) => OperatorType::LogicalAnd,
            _ => OperatorType::Invalid, // Default case for unrecognized strings
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match self {
            OperatorType::Invalid => "<invalid>",
            OperatorType::Not => SOUL_NAMES.get_name(NamesOperator::Not),
            OperatorType::Equals => SOUL_NAMES.get_name(NamesOperator::Equals),
            OperatorType::NotEquals => SOUL_NAMES.get_name(NamesOperator::NotEquals),
            OperatorType::IsSmaller => SOUL_NAMES.get_name(NamesOperator::IsSmaller),
            OperatorType::IsSmallerEquals => SOUL_NAMES.get_name(NamesOperator::IsSmallerEquals),
            OperatorType::IsBigger => SOUL_NAMES.get_name(NamesOperator::IsBigger),
            OperatorType::IsBiggerEquals => SOUL_NAMES.get_name(NamesOperator::IsBiggerEquals),

            OperatorType::Add => SOUL_NAMES.get_name(NamesOperator::Addition),
            OperatorType::Sub => SOUL_NAMES.get_name(NamesOperator::Subtract),
            OperatorType::Mul => SOUL_NAMES.get_name(NamesOperator::Multiple),
            OperatorType::Div => SOUL_NAMES.get_name(NamesOperator::Divide),
            OperatorType::Modulo => SOUL_NAMES.get_name(NamesOperator::Modulo),
            OperatorType::Pow => SOUL_NAMES.get_name(NamesOperator::Power),
            OperatorType::Root => SOUL_NAMES.get_name(NamesOperator::Root),
            OperatorType::Log => SOUL_NAMES.get_name(NamesOperator::Logarithm),

            OperatorType::BitWiseOr => SOUL_NAMES.get_name(NamesOperator::BitWiseOr),
            OperatorType::BitWiseAnd => SOUL_NAMES.get_name(NamesOperator::BitWiseAnd),
            OperatorType::BitWiseXor => SOUL_NAMES.get_name(NamesOperator::BitWiseXor),
            OperatorType::LogicalOr => SOUL_NAMES.get_name(NamesOperator::LogicalOr),
            OperatorType::LogicalAnd => SOUL_NAMES.get_name(NamesOperator::LogicalAnd),

            OperatorType::Increment => SOUL_NAMES.get_name(NamesOperator::Increment),
            OperatorType::Decrement => SOUL_NAMES.get_name(NamesOperator::Decrement),
        }
    }

    pub fn get_precedence_str(str: &str) -> u8 {
        OperatorType::from_str(str).get_precedence()
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            OperatorType::Increment |
            OperatorType::Decrement => 9,

            OperatorType::Not => 8,

            OperatorType::Log |
            OperatorType::Pow |
            OperatorType::Root => 7,

            OperatorType::Mul |
            OperatorType::Div |
            OperatorType::Modulo => 6,

            OperatorType::Add |
            OperatorType::Sub => 5,

            OperatorType::IsSmaller |
            OperatorType::IsSmallerEquals |
            OperatorType::IsBigger |
            OperatorType::IsBiggerEquals => 4,

            OperatorType::Equals |
            OperatorType::NotEquals => 3,

            OperatorType::BitWiseOr |
            OperatorType::BitWiseAnd |
            OperatorType::BitWiseXor => 2,

            OperatorType::LogicalOr |
            OperatorType::LogicalAnd => 1,

            OperatorType::Invalid => 0,
        }
    }

    pub fn is_boolean_operator(&self) -> bool {
        match self {
            OperatorType::Invalid |
            OperatorType::Add |
            OperatorType::Sub |
            OperatorType::Mul |
            OperatorType::Div |
            OperatorType::Modulo |
            OperatorType::Pow |
            OperatorType::Root |
            OperatorType::Log |
            OperatorType::BitWiseOr |
            OperatorType::BitWiseAnd |
            OperatorType::Increment |
            OperatorType::Decrement |
            OperatorType::BitWiseXor => false,

            OperatorType::Not |
            OperatorType::Equals |
            OperatorType::NotEquals |
            OperatorType::IsSmaller |
            OperatorType::IsSmallerEquals |
            OperatorType::IsBigger |
            OperatorType::IsBiggerEquals |
            OperatorType::LogicalOr |
            OperatorType::LogicalAnd => true,
        }
    }

    pub fn is_calculus_operator(&self) -> bool {
        match self {
            OperatorType::Invalid |
            OperatorType::Add |
            OperatorType::Sub |
            OperatorType::Mul |
            OperatorType::Div |
            OperatorType::Modulo |
            OperatorType::Pow |
            OperatorType::Root |
            OperatorType::Log |
            OperatorType::BitWiseOr |
            OperatorType::BitWiseAnd |
            OperatorType::Increment |
            OperatorType::Decrement |
            OperatorType::BitWiseXor => true,

            OperatorType::Not |
            OperatorType::Equals |
            OperatorType::NotEquals |
            OperatorType::IsSmaller |
            OperatorType::IsSmallerEquals |
            OperatorType::IsBigger |
            OperatorType::IsBiggerEquals |
            OperatorType::LogicalOr |
            OperatorType::LogicalAnd => false,
        }
    }
}

pub static ALL_OPERATORS: Lazy<Vec<OperatorType>> = Lazy::new(||
    enum_iterator::all::<OperatorType>().collect()
);

pub static BOOLEAN_OPERATOR: Lazy<Vec<OperatorType>> = Lazy::new(||
    enum_iterator::all::<OperatorType>().filter(|op| op.is_boolean_operator()).collect()
);

pub static CALCULUS_OPERATOR: Lazy<Vec<OperatorType>> = Lazy::new(||
    enum_iterator::all::<OperatorType>().filter(|op| op.is_calculus_operator()).collect()
);









