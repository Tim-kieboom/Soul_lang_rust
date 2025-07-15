use itertools::Itertools;
use std::collections::HashMap;
use crate::errors::soul_error::Result;
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Uint(u64),
    Float(f64),
    Bool(bool),
    Char(char),
    Str(String),
    Array{ty: LiteralType, values: Vec<Literal>},
    Tuple{ty: LiteralType, values: Vec<Literal>},
    NamedTuple{ty: LiteralType, values: HashMap<Ident, Literal>},
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralType {
    Int,
    Uint,
    Float,
    Bool,
    Char,
    Str,
    Array(Box<LiteralType>),
    Tuple(Vec<LiteralType>),
    NamedTuple(HashMap<Ident, LiteralType>),
}

impl LiteralType {
    pub fn type_to_string(&self) -> String {
        match self {
            LiteralType::Int => format!("untypedInt"),
            LiteralType::Uint => format!("untypedUint"),
            LiteralType::Float => format!("untypedFloat"),
            LiteralType::Bool => format!("bool"),
            LiteralType::Char => format!("char"),
            LiteralType::Str => format!("str"),
            LiteralType::Array(ty) => format!("{}[]", ty.type_to_string()),
            LiteralType::Tuple(tys) => format!("({})", tys.iter().map(|value| value.type_to_string()).join(",")),
            LiteralType::NamedTuple(tys) => format!("({})", tys.iter().map(|(name, value)| format!("{}: {}", name.0, value.type_to_string())).join(",")),
        }
    }

    fn precedence(&self) -> u8 {
        match self {
            LiteralType::Int => 2,
            LiteralType::Uint => 1,
            LiteralType::Float => 3,
            LiteralType::Bool => 1,
            LiteralType::Char => 1,
            LiteralType::Str => 1,
            LiteralType::Array(ty) => ty.precedence(),
            LiteralType::Tuple(..) => 1,
            LiteralType::NamedTuple(..) => 1,
        }
    }

    pub fn max(self, other: LiteralType) -> Self {
        if self.precedence() > other.precedence() {
            return self
        }

        other
    }

    pub fn force_array_type(&mut self, other: &LiteralType) {
        match self {
            LiteralType::Int |
            LiteralType::Uint |
            LiteralType::Float |
            LiteralType::Bool |
            LiteralType::Char |
            LiteralType::Str => (),

            LiteralType::Array(literal_type) => *literal_type.as_mut() = other.clone(),
            LiteralType::Tuple(literal_types) => literal_types.iter_mut().filter(|ty| !matches!(ty, LiteralType::Array(..))).for_each(|ty| ty.force_array_type(other)),
            LiteralType::NamedTuple(hash_map) => hash_map.iter_mut().filter(|(_name, ty)| !matches!(ty, LiteralType::Array(..))).for_each(|(_name, ty)| ty.force_array_type(other)),
        }
    }

    fn is_numeric(&self) -> bool {
        matches!(self, LiteralType::Int | LiteralType::Uint | LiteralType::Float)
    }

    fn is_compatible(&self, other: &Self) -> bool {
        if self.is_numeric() && other.is_numeric() {
            return true;
        }

        if let (LiteralType::Array(a), LiteralType::Array(b)) = (&self, &other) {
            return a.is_compatible(b.as_ref());
        }
        
        self == other
    }
}

impl Literal {
    pub fn new_array(literals: Vec<Literal>, span: &SoulSpan) -> Result<Literal> {
        let mut common_ty = literals
            .first()
            .map(|v| v.get_literal_type())
            .unwrap_or(LiteralType::Int);

        for lit in &literals {
            let next_ty = lit.get_literal_type();

            if common_ty.is_compatible(&next_ty) {
                common_ty = common_ty.max(next_ty);
            } else {
                return Err(new_soul_error(
                    SoulErrorKind::WrongType, 
                    *span, 
                    format!("Incompatible array literal types: {:?} vs {:?}", common_ty, next_ty)
                ));
            }
        }

        Ok(Literal::Array { ty: common_ty, values: literals })
    }

    pub fn new_tuple(literals: Vec<Literal>) -> Literal {
        let mut this = Literal::Tuple{ty: LiteralType::Int, values: literals};
        let lit_type = this.get_literal_type();
        if let Literal::Array{ty, ..} = &mut this {
            *ty = lit_type;
        }

        this
    }

    pub fn new_named_tuple(literals: HashMap<Ident, Literal>) -> Literal {
        let mut this = Literal::NamedTuple{ty: LiteralType::Int, values: literals};
        let lit_type = this.get_literal_type();
        if let Literal::Array{ty, ..} = &mut this {
            *ty = lit_type;
        }

        this
    }

    pub fn are_compatible(&self, other: &Self) -> bool {
        match (self, other) {
            (a, b) if a.is_numeric() && b.is_numeric() => true,

            (
                Literal::Array { ty: a_ty, .. },
                Literal::Array { ty: b_ty, .. },
            ) => {
                a_ty.is_compatible(b_ty)
            }

            // Same variant type
            _ => std::mem::discriminant(self) == std::mem::discriminant(other),
        }
    }

    pub fn get_literal_type(&self) -> LiteralType {
        match self {
            Literal::Int(_) => LiteralType::Int,
            Literal::Uint(_) => LiteralType::Uint,
            Literal::Float(_) => LiteralType::Float,
            Literal::Bool(_) => LiteralType::Bool,
            Literal::Char(_) => LiteralType::Char,
            Literal::Str(_) => LiteralType::Str,
            Literal::Array{ty, .. } => LiteralType::Array(Box::new(ty.clone())),
            Literal::Tuple{values, .. } => LiteralType::Tuple(values.iter().map(|val| val.get_literal_type()).collect::<Vec<_>>()),
            Literal::NamedTuple{values, .. } => LiteralType::NamedTuple(values.iter().map(|val| (val.0.clone(), val.1.get_literal_type())).collect::<HashMap<_,_>>()),
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            Literal::Bool(_) |
            Literal::Char(_) |
            Literal::Str(_) |
            Literal::Array{..} |
            Literal::Tuple{..} |
            Literal::NamedTuple{..}  => false,
                    
            Literal::Int(_) |
            Literal::Uint(_) |
            Literal::Float(_) => true,
        }
    }

    pub fn type_to_string(&self) -> String {
        match self {
            Literal::Int(_) => format!("Literal untypedInt"),
            Literal::Uint(_) => format!("Literal untypedUint"),
            Literal::Float(_) => format!("Literal untypedFloat"),
            Literal::Bool(_) => format!("Literal bool"),
            Literal::Char(_) => format!("Literal char"),
            Literal::Str(_) => format!("Literal str"),
            Literal::Array{ty, ..} => format!("Literal {}[]", ty.type_to_string()),
            Literal::Tuple{ty, ..} => format!("Literal ({})", ty.type_to_string()),
            Literal::NamedTuple{ty, ..} => format!("Literal ({})", ty.type_to_string()),
        }
    }

    pub fn value_to_string(&self) -> String {
        match self {
            Literal::Int(val) => format!("{}", val),
            Literal::Uint(val) => format!("{}", val),
            Literal::Float(val) => format!("{}", val),
            Literal::Bool(val) => format!("{}", val),
            Literal::Char(char) => format!("{}", char),
            Literal::Str(str) => format!("{}", str),
            Literal::Array{values, ..} => format!("[{}]", values.iter().map(|lit| lit.value_to_string()).join(",")),
            Literal::Tuple{values, ..} => format!("({})", values.iter().map(|value| value.value_to_string()).join(",")),
            Literal::NamedTuple{values, ..} => format!("({})", values.iter().map(|(name, value)| format!("{}: {}", name.0, value.value_to_string())).join(",")),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Literal::Int(val) => format!("Literal untypedInt {}", val),
            Literal::Uint(val) => format!("Literal untypedUint {}", val),
            Literal::Float(val) => format!("Literal untypedFloat {}", val),
            Literal::Bool(val) => format!("Literal bool {}", val),
            Literal::Char(char) => format!("Literal char {}", char),
            Literal::Str(str) => format!("Literal str {}", str),
            Literal::Array{values, ..} => format!("Literal [{}; {}]", values.last().map(|lit| lit.type_to_string()).unwrap_or(format!("<unknown>")), values.iter().map(|lit| lit.value_to_string()).join(",")),
            Literal::Tuple{values, ..} => format!("Literal ({})", values.iter().map(|value| value.to_string()).join(",")),
            Literal::NamedTuple{values, ..} => format!("Literal ({})", values.iter().map(|(name, value)| format!("{}: {}", name.0, value.to_string())).join(",")),
        }
    }
}











































