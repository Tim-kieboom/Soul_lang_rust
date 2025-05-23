use std::io::Result;

use super::soul_type::SoulType;
use crate::{meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, soul_names::{NamesInternalType, NamesTypeModifiers, NamesTypeWrapper, SOUL_NAMES}, soul_type::{type_modifiers::TypeModifiers, type_wrappers::TypeWrappers}}, tokenizer::{file_line::FileLine, token::TokenIterator, tokenizer::tokenize_line}};
fn try_simple_from_iterator(line: &str) -> Result<SoulType> {
    let mut meta_data = MetaData::new();
    let context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    
    let mut in_multi_line_commend = false;
    let tokens = tokenize_line(FileLine{text: line.to_string(), line_number: 0}, 0, &mut in_multi_line_commend, &mut meta_data)?;

    let mut iter = TokenIterator::new(tokens);

    SoulType::from_iterator(&mut iter, &meta_data.type_meta_data, &context.current_generics)
}

fn simple_from_iterator(line: &str) -> SoulType {
    try_simple_from_iterator(line)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap()
}

fn try_simple_from_stringed_type(line: &str) -> Result<SoulType> {
    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    
    let mut in_multi_line_commend = false;
    let tokens = tokenize_line(FileLine{text: line.to_string(), line_number: 0}, 0, &mut in_multi_line_commend, &mut meta_data)?;

    let iter = TokenIterator::new(tokens);

    SoulType::from_stringed_type(line, iter.current(), &meta_data.type_meta_data, &mut context.current_generics)
}

fn simple_from_stringed_type(line: &str) -> SoulType { 
    try_simple_from_stringed_type(line)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap()
}

#[test]
fn test_soul_type_from_iterator() {
    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);
    let const_ = SOUL_NAMES.get_name(NamesTypeModifiers::Constent);
    let static_ = SOUL_NAMES.get_name(NamesTypeModifiers::Static);
    let volatile = SOUL_NAMES.get_name(NamesTypeModifiers::Volatile);

    let mut_ref = SOUL_NAMES.get_name(NamesTypeWrapper::MutRef);
    let const_ref = SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef);
    let array = SOUL_NAMES.get_name(NamesTypeWrapper::Array);
    let pointer = SOUL_NAMES.get_name(NamesTypeWrapper::Pointer);
    
    let int = SOUL_NAMES.get_name(NamesInternalType::Int);
    let str = SOUL_NAMES.get_name(NamesInternalType::String);

    
    let mut ty = simple_from_iterator(int);
    assert_eq!(ty, SoulType::new(int.to_string()));

    let mut_ref_int = format!("{}{}", int, mut_ref);
    ty = simple_from_iterator(&mut_ref_int);
    assert_eq!(ty, SoulType::from_wrappers(int.to_string(), vec![TypeWrappers::MutRef]));

    let const_ref_int = format!("{}{}", int, const_ref);
    ty = simple_from_iterator(&const_ref_int);
    assert_eq!(ty, SoulType::from_wrappers(int.to_string(), vec![TypeWrappers::ConstRef]));
   
    let array_int = format!("{}{}", int, array);
    ty = simple_from_iterator(&array_int);
    assert_eq!(ty, SoulType::from_wrappers(int.to_string(), vec![TypeWrappers::Array])); 
    
    let pointer_int = format!("{}{}", int, pointer);
    ty = simple_from_iterator(&pointer_int);
    assert_eq!(ty, SoulType::from_wrappers(int.to_string(), vec![TypeWrappers::Pointer]));


    let lit_int = format!("{} {}", literal, int);
    ty = simple_from_iterator(&lit_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Literal));

    let const_int = format!("{} {}", const_, int);
    ty = simple_from_iterator(&const_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Const));

    let static_int = format!("{} {}", static_, int);
    ty = simple_from_iterator(&static_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Static));

    let volatile_int = format!("{} {}", volatile, int);
    ty = simple_from_iterator(&volatile_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Volatile));

    let lit_int = format!("{} {}", literal, int);
    ty = simple_from_iterator(&lit_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Literal));

    let lit_const_int = format!("{} {} {}", literal, const_, int);
    let res = try_simple_from_iterator(&lit_const_int);
    assert!(res.is_err());


    ty = simple_from_iterator(str);
    assert_eq!(ty, SoulType::new(str.to_string()));
}

#[test]
fn test_soul_type_from_stringed_type() {
    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);
    let const_ = SOUL_NAMES.get_name(NamesTypeModifiers::Constent);
    let static_ = SOUL_NAMES.get_name(NamesTypeModifiers::Static);
    let volatile = SOUL_NAMES.get_name(NamesTypeModifiers::Volatile);

    let mut_ref = SOUL_NAMES.get_name(NamesTypeWrapper::MutRef);
    let const_ref = SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef);
    let array = SOUL_NAMES.get_name(NamesTypeWrapper::Array);
    let pointer = SOUL_NAMES.get_name(NamesTypeWrapper::Pointer);
    
    let int = SOUL_NAMES.get_name(NamesInternalType::Int);
    let str = SOUL_NAMES.get_name(NamesInternalType::String);

    
    let mut ty = simple_from_stringed_type(int);
    assert_eq!(ty, SoulType::new(int.to_string()));

    let mut_ref_int = format!("{}{}", int, mut_ref);
    ty = simple_from_stringed_type(&mut_ref_int);
    assert_eq!(ty, SoulType::from_wrappers(int.to_string(), vec![TypeWrappers::MutRef]));

    let const_ref_int = format!("{}{}", int, const_ref);
    ty = simple_from_stringed_type(&const_ref_int);
    assert_eq!(ty, SoulType::from_wrappers(int.to_string(), vec![TypeWrappers::ConstRef]));
   
    let array_int = format!("{}{}", int, array);
    ty = simple_from_stringed_type(&array_int);
    assert_eq!(ty, SoulType::from_wrappers(int.to_string(), vec![TypeWrappers::Array])); 
    
    let pointer_int = format!("{}{}", int, pointer);
    ty = simple_from_stringed_type(&pointer_int);
    assert_eq!(ty, SoulType::from_wrappers(int.to_string(), vec![TypeWrappers::Pointer]));


    let lit_int = format!("{} {}", literal, int);
    ty = simple_from_stringed_type(&lit_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Literal));

    let const_int = format!("{} {}", const_, int);
    ty = simple_from_stringed_type(&const_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Const));

    let static_int = format!("{} {}", static_, int);
    ty = simple_from_stringed_type(&static_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Static));

    let volatile_int = format!("{} {}", volatile, int);
    ty = simple_from_stringed_type(&volatile_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Volatile));

    let lit_int = format!("{} {}", literal, int);
    ty = simple_from_stringed_type(&lit_int);
    assert_eq!(ty, SoulType::from_modifiers(int.to_string(), TypeModifiers::Literal));

    let lit_const_int = format!("{} {} {}", literal, const_, int);
    let res = try_simple_from_stringed_type(&lit_const_int);
    assert!(res.is_err());


    ty = simple_from_stringed_type(str);
    assert_eq!(ty, SoulType::new(str.to_string()));
}
