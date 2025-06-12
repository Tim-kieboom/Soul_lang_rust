use crate::meta_data::soul_error::soul_error::{new_soul_error, Result, SoulSpan};
use crate::meta_data::soul_type::type_wrappers::TypeWrappers;
use crate::{meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, soul_type::{primitive_types::PrimitiveType, soul_type::SoulType, type_modifiers::TypeModifiers}}, tokenizer::token::Token};

pub struct CppType(pub String);
impl CppType {
    pub fn from_soul_type(_soul_type: &SoulType, meta_data: &MetaData, context: &CurrentContext, span: &SoulSpan) -> Result<Self> {
        
        let soul_type = _soul_type.convert_typedef_to_original(&token_from_span(span), &meta_data.type_meta_data, &context.current_generics)
            .ok_or(new_soul_error(&token_from_span(span), "Internal error in CppType::from_soul_type() convert_typedef_to_original failed"))?;
        
        let is_class = soul_type.is_class(&meta_data.type_meta_data.class_store);
        
        let mut cpp_type = String::new();
        soul_modifier_to_cpp(&mut cpp_type, soul_type.modifiers);
        if !cpp_type.is_empty() {
            cpp_type.push(' ');
        }

        let mut last_part = String::new(); 

        let cpp_name = if is_class {
            &soul_type.name
        }
        else {
            soul_primitive_to_cpp(soul_type.to_primitive_type(&meta_data.type_meta_data))
        };

        last_part.push_str(cpp_name);
        last_part.push(' ');

        soul_wrappers_to_cpp(&mut last_part, &soul_type.wrappers);
        last_part.push(' ');

        cpp_type.push_str(&last_part);
        Ok(Self(cpp_type))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn soul_wrappers_to_cpp(string_builder: &mut String, wrappers: &Vec<TypeWrappers>) {
    
    for wrap in wrappers {
        match wrap {
            TypeWrappers::Invalid => panic!("Internal Error in soul_wrappers_to_cpp() type is invalid"),
            TypeWrappers::ConstRef => string_builder.push_str("&"),
            TypeWrappers::MutRef => string_builder.push_str("&"),
            TypeWrappers::Pointer => string_builder.push_str("&"),
            TypeWrappers::Array => *string_builder = format!("__Soul_ARRAY__<{}>", string_builder),
        }
    }
}

fn soul_modifier_to_cpp(string_builder: &mut String, modifier: TypeModifiers) {
    
    if modifier.contains(TypeModifiers::Literal) {
        string_builder.push_str("constexpr");
    }
    if modifier.contains(TypeModifiers::Const) {
        string_builder.push_str("const");
    }
    if modifier.contains(TypeModifiers::Static) {
        string_builder.push_str("static");
    }
    if modifier.contains(TypeModifiers::Volatile) {
        string_builder.push_str("volatile");
    }
}

fn soul_primitive_to_cpp(prim: PrimitiveType) -> &'static str {
    match prim {
        PrimitiveType::Invalid => panic!("Internal Error in soul_primitive_to_cpp() type is invalid"),
        PrimitiveType::Object => panic!("Internal Error in soul_primitive_to_cpp() type is object"),
        PrimitiveType::None => "void",
        PrimitiveType::UntypedInt => "int",
        PrimitiveType::Int => "int",
        PrimitiveType::I8 => "int8_t",
        PrimitiveType::I16 => "int16_t",
        PrimitiveType::I32 => "int32_t",
        PrimitiveType::I64 => "int64_t",
        PrimitiveType::UntypedUint => "unsigned int",
        PrimitiveType::Uint => "unsigned int",
        PrimitiveType::U8 => "uint8_t",
        PrimitiveType::U16 => "uint16_t",
        PrimitiveType::U32 => "uint32_t",
        PrimitiveType::U64 => "uint64_t",
        PrimitiveType::UntypedFloat => "float",
        PrimitiveType::F32 => "float",
        PrimitiveType::F64 => "double",
        PrimitiveType::Bool => "bool",
        PrimitiveType::Char => "char",
    }
}

fn token_from_span(span: &SoulSpan) -> Token {
    Token{line_number: span.line_number, line_offset: span.line_offset, text: String::new()}
}










