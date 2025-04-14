use crate::meta_data::soul_type::primitive_types::PrimitiveType;

pub fn get_primitive_type_from_literal(literal: &str) -> PrimitiveType {
    if literal.is_empty() {
        return PrimitiveType::Invalid;
    }

    let number_type = get_number_from_literal(literal);

    let is_bool = literal == "true" || literal == "false";
    let is_char = literal.len() > 2 && literal.chars().nth(0).unwrap() == '\'' && literal.chars().nth_back(1).unwrap() == '\'';
    let is_number = number_type != PrimitiveType::Invalid;

    if is_bool {
        PrimitiveType::Bool
    } else if is_char {
        PrimitiveType::Char
    } else if is_number {
        number_type
    } else {
        PrimitiveType::Invalid
    }
}

fn get_number_from_literal(s: &str) -> PrimitiveType {
    if s.len() > 2 && (&s[..2] == "0x" || &s[..2] == "0b") {
        
        if s[2..].parse::<u64>().is_err() {
            return PrimitiveType::Invalid;
        }

        let mut num_bytes = s[2..].len();
        if &s[..2] == "0b" {
            num_bytes /= 8;
        }

        return match num_bytes {
            var if var <= 1 => PrimitiveType::U8,
            var if var <= 2 => PrimitiveType::U16,
            var if var <= 4 => PrimitiveType::U32,
            var if var <= 8 => PrimitiveType::U64,
            _ => PrimitiveType::Invalid,
        }
    }

    if s.parse::<i64>().is_ok() {
        return PrimitiveType::UntypedInt;
    }

    if s.parse::<f64>().is_ok() {
        return PrimitiveType::UntypedFloat;
    }

    PrimitiveType::Invalid
}