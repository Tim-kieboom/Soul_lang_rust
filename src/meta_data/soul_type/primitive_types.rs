use crate::meta_data::{key_tokens::InternalType, meta_data::MetaData};

#[derive(Debug, PartialEq)]
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
}

impl PrimitiveType {
    pub fn from_str(str: &str, meta_data: &MetaData) -> Self {
        
        match str {
            val if val == meta_data.get_soul_name(InternalType::Boolean) => PrimitiveType::Bool,
            val if val == meta_data.get_soul_name(InternalType::Character) => PrimitiveType::Char,

            val if val == meta_data.get_soul_name(InternalType::UntypedInt) => PrimitiveType::UntypedInt,
            val if val == meta_data.get_soul_name(InternalType::Int) => PrimitiveType::Int,
            val if val == meta_data.get_soul_name(InternalType::Int8) => PrimitiveType::I8,
            val if val == meta_data.get_soul_name(InternalType::Int16) => PrimitiveType::I16,
            val if val == meta_data.get_soul_name(InternalType::Int32) => PrimitiveType::I32,
            val if val == meta_data.get_soul_name(InternalType::Int64) => PrimitiveType::I64,

            val if val == meta_data.get_soul_name(InternalType::UntypedUint) => PrimitiveType::UntypedUint,
            val if val == meta_data.get_soul_name(InternalType::Uint) => PrimitiveType::Uint,
            val if val == meta_data.get_soul_name(InternalType::Uint8) => PrimitiveType::U8,
            val if val == meta_data.get_soul_name(InternalType::Uint16) => PrimitiveType::U16,
            val if val == meta_data.get_soul_name(InternalType::Uint32) => PrimitiveType::U32,
            val if val == meta_data.get_soul_name(InternalType::Uint64) => PrimitiveType::U64,

            val if val == meta_data.get_soul_name(InternalType::UntypedFloat) => PrimitiveType::UntypedFloat,
            val if val == meta_data.get_soul_name(InternalType::Float32) => PrimitiveType::F32,
            val if val == meta_data.get_soul_name(InternalType::Float64) => PrimitiveType::F64,

            _ => PrimitiveType::Invalid,
        }
    }
}