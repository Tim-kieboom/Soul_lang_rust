use crate::meta_data::{meta_data::MetaData, soul_names::{NamesInternalType, SOUL_NAMES}, type_meta_data::TypeMetaData};

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
pub enum PrimitiveType {
    Invalid = -1,

    //void type
    None = 0,

    //int from literal value (can be automaticly cast to all numbers) 
    UntypedInt = 1,
    //int with system based size
	Int = 2,
	//int 8 bits
	I8 = 3,
	//int 16 bits
	I16 = 4,
	//int 32 bits
	I32 = 5,
	//int 64 bits
	I64 = 6,

    //uint from literal value (can be automaticly cast to all numbers) 
    UntypedUint = 7,
	//unsigned int with system based size
	Uint = 8,
	//unsigned int 8 bits
	U8 = 9,
	//unsigned int 16 bits
	U16 = 10,
	//unsigned int 32 bits
	U32 = 11,
	//unsigned int 64 bits
	U64 = 12,

    //flaot from literal value (can be automaticly cast to all numbers) 
    UntypedFloat = 13,
	//float
	F32 = 14,
	//double
	F64 = 15,

	//boolean
	Bool = 16,
	//character
	Char = 17,

    //stuct or class or enum ect...
    Object = 18,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
pub enum  DuckType {
    Invalid = -1,

    //void type
    None = 0,

    //number from literal value (can be automaticly cast to all numbers) 
    UntypedNumber = 1,
    Number = 2,

    Boolean = 3,
    Character = 4,

    //stuct or class or enum ect...
    Object = 5,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
pub enum NumberCategory {
    Invalid,
    Interger,
    UnsignedInterger,
    FloatingPoint
}

impl PrimitiveType {
    pub fn from_str(str: &str, type_meta_data: &TypeMetaData) -> Self {
        
        if type_meta_data.class_store.contains_key(str) {
            return PrimitiveType::Object;
        }

        match str {
            val if val == SOUL_NAMES.get_name(NamesInternalType::Boolean) => PrimitiveType::Bool,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Character) => PrimitiveType::Char,

            val if val == SOUL_NAMES.get_name(NamesInternalType::UntypedInt) => PrimitiveType::UntypedInt,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Int) => PrimitiveType::Int,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Int8) => PrimitiveType::I8,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Int16) => PrimitiveType::I16,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Int32) => PrimitiveType::I32,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Int64) => PrimitiveType::I64,

            val if val == SOUL_NAMES.get_name(NamesInternalType::UntypedUint) => PrimitiveType::UntypedUint,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Uint) => PrimitiveType::Uint,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Uint8) => PrimitiveType::U8,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Uint16) => PrimitiveType::U16,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Uint32) => PrimitiveType::U32,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Uint64) => PrimitiveType::U64,

            val if val == SOUL_NAMES.get_name(NamesInternalType::UntypedFloat) => PrimitiveType::UntypedFloat,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Float32) => PrimitiveType::F32,
            val if val == SOUL_NAMES.get_name(NamesInternalType::Float64) => PrimitiveType::F64,

            _ => PrimitiveType::Invalid,
        }
    }

    pub fn to_str<'a>(&'a self, type_meta_data: &TypeMetaData) -> Option<&'a str> {
        match self {
            PrimitiveType::Invalid => Some("<invalid>"),
            PrimitiveType::None => Some(SOUL_NAMES.get_name(NamesInternalType::None)),
            PrimitiveType::UntypedInt => Some(SOUL_NAMES.get_name(NamesInternalType::UntypedInt)),
            PrimitiveType::Int => Some(SOUL_NAMES.get_name(NamesInternalType::Int)),
            PrimitiveType::I8 => Some(SOUL_NAMES.get_name(NamesInternalType::Int8)),
            PrimitiveType::I16 => Some(SOUL_NAMES.get_name(NamesInternalType::Int16)),
            PrimitiveType::I32 => Some(SOUL_NAMES.get_name(NamesInternalType::Int32)),
            PrimitiveType::I64 => Some(SOUL_NAMES.get_name(NamesInternalType::Int64)),
            PrimitiveType::UntypedUint => Some(SOUL_NAMES.get_name(NamesInternalType::UntypedUint)),
            PrimitiveType::Uint => Some(SOUL_NAMES.get_name(NamesInternalType::Uint)),
            PrimitiveType::U8 => Some(SOUL_NAMES.get_name(NamesInternalType::Uint8)),
            PrimitiveType::U16 => Some(SOUL_NAMES.get_name(NamesInternalType::Uint16)),
            PrimitiveType::U32 => Some(SOUL_NAMES.get_name(NamesInternalType::Uint32)),
            PrimitiveType::U64 => Some(SOUL_NAMES.get_name(NamesInternalType::Uint64)),
            PrimitiveType::UntypedFloat => Some(SOUL_NAMES.get_name(NamesInternalType::UntypedFloat)),
            PrimitiveType::F32 => Some(SOUL_NAMES.get_name(NamesInternalType::Float32)),
            PrimitiveType::F64 => Some(SOUL_NAMES.get_name(NamesInternalType::Float64)),
            PrimitiveType::Bool => Some(SOUL_NAMES.get_name(NamesInternalType::Boolean)),
            PrimitiveType::Char => Some(SOUL_NAMES.get_name(NamesInternalType::Character)),
            PrimitiveType::Object => None,
        }
    }

    pub fn is_untyped_type(&self) -> bool {
        match self {
            PrimitiveType::UntypedInt |
            PrimitiveType::UntypedUint |
            PrimitiveType::UntypedFloat => true,
            _ => false,
        }
    }

    pub fn to_number_category(&self) -> NumberCategory {
        match self {
            PrimitiveType::UntypedInt |
            PrimitiveType::Int |
            PrimitiveType::I8 |
            PrimitiveType::I16 |
            PrimitiveType::I32 |
            PrimitiveType::I64 => NumberCategory::Interger,

            PrimitiveType::UntypedUint |
            PrimitiveType::Uint |
            PrimitiveType::U8 |
            PrimitiveType::U16 |
            PrimitiveType::U32 |
            PrimitiveType::U64 => NumberCategory::UnsignedInterger,

            PrimitiveType::UntypedFloat |
            PrimitiveType::F32 |
            PrimitiveType::F64 => NumberCategory::FloatingPoint,

            _ => NumberCategory::Invalid
        }   
    }

    pub fn to_duck_type(&self) -> DuckType {
        match self {
            PrimitiveType::Invalid => DuckType::Invalid,
            PrimitiveType::None => DuckType::None,
            PrimitiveType::UntypedInt |
            PrimitiveType::UntypedUint |
            PrimitiveType::UntypedFloat |
            PrimitiveType::Int |
            PrimitiveType::I8 |
            PrimitiveType::I16 |
            PrimitiveType::I32 |
            PrimitiveType::I64 |
            PrimitiveType::Uint |
            PrimitiveType::U8 |
            PrimitiveType::U16 |
            PrimitiveType::U32 |
            PrimitiveType::U64 |
            PrimitiveType::F32 |
            PrimitiveType::F64 => DuckType::Number,
            PrimitiveType::Bool => DuckType::Boolean,
            PrimitiveType::Char => DuckType::Character,
            PrimitiveType::Object => DuckType::Object,
        }
    }
}