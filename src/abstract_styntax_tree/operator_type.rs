use crate::meta_data::soul_names::{NamesOperator, SOUL_NAMES};
use enum_iterator::Sequence;
use once_cell::sync::Lazy;

#[derive(Debug, Sequence, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i8)]
pub enum ExprOperatorType {
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

    Range               = 22,
}

impl ExprOperatorType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "<invalid>" => ExprOperatorType::Invalid,
            val if val == SOUL_NAMES.get_name(NamesOperator::Not) => ExprOperatorType::Not,
            val if val == SOUL_NAMES.get_name(NamesOperator::Equals) => ExprOperatorType::Equals,
            val if val == SOUL_NAMES.get_name(NamesOperator::NotEquals) => ExprOperatorType::NotEquals,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsSmaller) => ExprOperatorType::IsSmaller,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsSmallerEquals) => ExprOperatorType::IsSmallerEquals,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsBigger) => ExprOperatorType::IsBigger,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsBiggerEquals) => ExprOperatorType::IsBiggerEquals,

            val if val == SOUL_NAMES.get_name(NamesOperator::Addition) => ExprOperatorType::Add,
            val if val == SOUL_NAMES.get_name(NamesOperator::Increment) => ExprOperatorType::Increment,
            val if val == SOUL_NAMES.get_name(NamesOperator::Decrement) => ExprOperatorType::Decrement,
            val if val == SOUL_NAMES.get_name(NamesOperator::Subtract) => ExprOperatorType::Sub,
            val if val == SOUL_NAMES.get_name(NamesOperator::Multiple) => ExprOperatorType::Mul,
            val if val == SOUL_NAMES.get_name(NamesOperator::Divide) => ExprOperatorType::Div,
            val if val == SOUL_NAMES.get_name(NamesOperator::Modulo) => ExprOperatorType::Modulo,
            val if val == SOUL_NAMES.get_name(NamesOperator::Power) => ExprOperatorType::Pow,
            val if val == SOUL_NAMES.get_name(NamesOperator::Root) => ExprOperatorType::Root,
            val if val == SOUL_NAMES.get_name(NamesOperator::Logarithm) => ExprOperatorType::Log,

            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseOr) => ExprOperatorType::BitWiseOr,
            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseAnd) => ExprOperatorType::BitWiseAnd,
            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseXor) => ExprOperatorType::BitWiseXor,
            val if val == SOUL_NAMES.get_name(NamesOperator::LogicalOr) => ExprOperatorType::LogicalOr,
            val if val == SOUL_NAMES.get_name(NamesOperator::LogicalAnd) => ExprOperatorType::LogicalAnd,

            val if val == SOUL_NAMES.get_name(NamesOperator::LogicalAnd) => ExprOperatorType::LogicalAnd,
            _ => ExprOperatorType::Invalid, // Default case for unrecognized strings
        }
    }

    pub fn to_str<'a>(&self) -> &'a str {
        match self {
            ExprOperatorType::Invalid => "<invalid>",
            ExprOperatorType::Not => SOUL_NAMES.get_name(NamesOperator::Not),
            ExprOperatorType::Equals => SOUL_NAMES.get_name(NamesOperator::Equals),
            ExprOperatorType::NotEquals => SOUL_NAMES.get_name(NamesOperator::NotEquals),
            ExprOperatorType::IsSmaller => SOUL_NAMES.get_name(NamesOperator::IsSmaller),
            ExprOperatorType::IsSmallerEquals => SOUL_NAMES.get_name(NamesOperator::IsSmallerEquals),
            ExprOperatorType::IsBigger => SOUL_NAMES.get_name(NamesOperator::IsBigger),
            ExprOperatorType::IsBiggerEquals => SOUL_NAMES.get_name(NamesOperator::IsBiggerEquals),

            ExprOperatorType::Add => SOUL_NAMES.get_name(NamesOperator::Addition),
            ExprOperatorType::Sub => SOUL_NAMES.get_name(NamesOperator::Subtract),
            ExprOperatorType::Mul => SOUL_NAMES.get_name(NamesOperator::Multiple),
            ExprOperatorType::Div => SOUL_NAMES.get_name(NamesOperator::Divide),
            ExprOperatorType::Modulo => SOUL_NAMES.get_name(NamesOperator::Modulo),
            ExprOperatorType::Pow => SOUL_NAMES.get_name(NamesOperator::Power),
            ExprOperatorType::Root => SOUL_NAMES.get_name(NamesOperator::Root),
            ExprOperatorType::Log => SOUL_NAMES.get_name(NamesOperator::Logarithm),

            ExprOperatorType::BitWiseOr => SOUL_NAMES.get_name(NamesOperator::BitWiseOr),
            ExprOperatorType::BitWiseAnd => SOUL_NAMES.get_name(NamesOperator::BitWiseAnd),
            ExprOperatorType::BitWiseXor => SOUL_NAMES.get_name(NamesOperator::BitWiseXor),
            ExprOperatorType::LogicalOr => SOUL_NAMES.get_name(NamesOperator::LogicalOr),
            ExprOperatorType::LogicalAnd => SOUL_NAMES.get_name(NamesOperator::LogicalAnd),

            ExprOperatorType::Increment => SOUL_NAMES.get_name(NamesOperator::Increment),
            ExprOperatorType::Decrement => SOUL_NAMES.get_name(NamesOperator::Decrement),

            ExprOperatorType::Range => SOUL_NAMES.get_name(NamesOperator::Range),
        }
    }

    pub fn get_precedence_str(str: &str) -> u8 {
        ExprOperatorType::from_str(str).get_precedence()
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            ExprOperatorType::Increment |
            ExprOperatorType::Decrement => 9,
            
            ExprOperatorType::Not => 8,
            
            ExprOperatorType::Log |
            ExprOperatorType::Pow |
            ExprOperatorType::Root => 7,
            
            ExprOperatorType::Mul |
            ExprOperatorType::Div |
            ExprOperatorType::Modulo => 6,
            
            ExprOperatorType::Add |
            ExprOperatorType::Sub => 5,
            
            ExprOperatorType::Range |
            ExprOperatorType::IsSmaller |
            ExprOperatorType::IsSmallerEquals |
            ExprOperatorType::IsBigger |
            ExprOperatorType::IsBiggerEquals => 4,

            ExprOperatorType::Equals |
            ExprOperatorType::NotEquals => 3,

            ExprOperatorType::BitWiseOr |
            ExprOperatorType::BitWiseAnd |
            ExprOperatorType::BitWiseXor => 2,

            ExprOperatorType::LogicalOr |
            ExprOperatorType::LogicalAnd => 1,


            ExprOperatorType::Invalid => 0,
        }
    }

    pub fn is_boolean_operator(&self) -> bool {
        match self {
            ExprOperatorType::Invalid |
            ExprOperatorType::Add |
            ExprOperatorType::Sub |
            ExprOperatorType::Mul |
            ExprOperatorType::Div |
            ExprOperatorType::Pow |
            ExprOperatorType::Log |
            ExprOperatorType::Root |
            ExprOperatorType::Range |
            ExprOperatorType::Modulo |
            ExprOperatorType::BitWiseOr |
            ExprOperatorType::Increment |
            ExprOperatorType::Decrement |
            ExprOperatorType::BitWiseAnd |
            ExprOperatorType::BitWiseXor => false,

            ExprOperatorType::Not |
            ExprOperatorType::Equals |
            ExprOperatorType::IsBigger |
            ExprOperatorType::NotEquals |
            ExprOperatorType::IsSmaller |
            ExprOperatorType::LogicalOr |
            ExprOperatorType::IsBiggerEquals |
            ExprOperatorType::IsSmallerEquals |
            ExprOperatorType::LogicalAnd => true,
        }
    }
}

pub static ALL_OPERATORS: Lazy<Vec<ExprOperatorType>> = Lazy::new(||
    enum_iterator::all::<ExprOperatorType>().collect()
);

pub static BOOLEAN_OPERATOR: Lazy<Vec<ExprOperatorType>> = Lazy::new(||
    enum_iterator::all::<ExprOperatorType>().filter(|op| op.is_boolean_operator()).collect()
);









