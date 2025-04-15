#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
}

impl OperatorType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "<invalid>" => OperatorType::Invalid,
            "!" => OperatorType::Not,
            "==" => OperatorType::Equals,
            "!=" => OperatorType::NotEquals,
            "<" => OperatorType::IsSmaller,
            "<=" => OperatorType::IsSmallerEquals,
            ">" => OperatorType::IsBigger,
            ">=" => OperatorType::IsBiggerEquals,
            "+" => OperatorType::Add,
            "-" => OperatorType::Sub,
            "*" => OperatorType::Mul,
            "/" => OperatorType::Div,
            "%" => OperatorType::Modulo,
            "**" => OperatorType::Pow,
            "</" => OperatorType::Root,
            "log" => OperatorType::Log,
            "|" => OperatorType::BitWiseOr,
            "&" => OperatorType::BitWiseAnd,
            "^" => OperatorType::BitWiseXor,
            "||" => OperatorType::LogicalOr,
            "&&" => OperatorType::LogicalAnd,
            _ => OperatorType::Invalid, // Default case for unrecognized strings
        }
    }

    pub const fn to_str<'a>(&self) -> &'a str {
        match self {
            OperatorType::Invalid => "<invalid>",
    
            OperatorType::Not => "!",
            OperatorType::Equals => "==",
            OperatorType::NotEquals => "!=",
            OperatorType::IsSmaller => "<",
            OperatorType::IsSmallerEquals => "<=",
            OperatorType::IsBigger => ">",
            OperatorType::IsBiggerEquals => ">=",
    
            OperatorType::Add => "+",
            OperatorType::Sub => "-",
            OperatorType::Mul => "*",
            OperatorType::Div => "/",
            OperatorType::Modulo => "%",
            OperatorType::Pow => "**",
            OperatorType::Root => "</",
            OperatorType::Log => "log",
    
            OperatorType::BitWiseOr => "|",
            OperatorType::BitWiseAnd => "&",
            OperatorType::BitWiseXor => "^",
            OperatorType::LogicalOr => "||",
            OperatorType::LogicalAnd => "&&",
        }
    }
}


