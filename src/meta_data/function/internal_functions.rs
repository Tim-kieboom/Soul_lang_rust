use once_cell::sync::Lazy;
use std::collections::{BTreeMap, HashMap};
use crate::{abstract_styntax_tree::operator_type::ALL_OPERATORS, meta_data::{soul_names::{NamesInternalType, NamesTypeWrapper, NAMES_INTERNAL_TYPE_NUMBER, SOUL_NAMES}, soul_type::generic::{Generic, GenericValidater}}};

use super::{argument_info::argument_info::ArgumentInfo, function_declaration::function_declaration::{FunctionDeclaration, FunctionID}, function_modifiers::FunctionModifiers};

fn new_function_id(id: &mut u32) -> FunctionID {
    *id += 1;
    FunctionID(*id)
}

const ANY_T_NAME: &str = "T";
static ANY_T_GENERIC: Lazy<(String, Generic)> = Lazy::new(|| (ANY_T_NAME.to_string(), Generic{type_name: ANY_T_NAME.to_string(), validater: None}));

pub static FIRST_FUNCTION_ID: Lazy<FunctionID> = Lazy::new(|| {
    FunctionID(INTERNAL_FUNCTIONS.len() as u32)
});

pub static INTERNAL_FUNCTIONS: Lazy<Vec<FunctionDeclaration>> = Lazy::new(|| {
    let mut current_id = 0u32;

    let mut functions = vec![
        FunctionDeclaration{
            name: "print".to_string(), 
            return_type: None,
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "msg".to_string(), 
                    /*type:*/     ANY_T_NAME.to_string(), 
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::from([ANY_T_GENERIC.clone()]),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: "println".to_string(), 
            return_type: None,
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "msg".to_string(), 
                    /*type:*/     ANY_T_NAME.to_string(), 
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::from([]),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: "println".to_string(), 
            return_type: None,
            args: vec![],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: "__Soul_format_string__".to_string(), 
            return_type: Some(SOUL_NAMES.get_name(NamesInternalType::String).to_string()),
            args: vec![
                ArgumentInfo{
                    name: "msg".to_string(),
                    value_type: ANY_T_NAME.to_string(),
                    default_value: None,
                    is_mutable: false,
                    arg_position: 0,
                    can_be_multiple: true,
                }   
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::from([ANY_T_GENERIC.clone()]),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: "panic".to_string(), 
            return_type: None,
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "msg".to_string(), 
                    /*type:*/     ANY_T_NAME.to_string(), 
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                ) 
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::from([ANY_T_GENERIC.clone()]),
            modifiers: FunctionModifiers::Default,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: "panic".to_string(), 
            return_type: None,
            args: vec![],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Default,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: "__Soul_internal_length__".to_string(), 
            return_type: Some(SOUL_NAMES.get_name(NamesInternalType::Uint).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "array".to_string(), 
                    /*type:*/     format!("{}{}", ANY_T_NAME, SOUL_NAMES.get_name(NamesTypeWrapper::Array)), 
                    // just an Generic array ^^^
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::from([ANY_T_GENERIC.clone()]),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: "__Soul_internal_length__".to_string(), 
            return_type: Some(SOUL_NAMES.get_name(NamesInternalType::Uint).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "array".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::String).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: SOUL_NAMES.get_name(NamesInternalType::String).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(NamesInternalType::Int64).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "num".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::Int64).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: SOUL_NAMES.get_name(NamesInternalType::String).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(NamesInternalType::Uint64).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "num".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::Uint64).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
        FunctionDeclaration{
            name: SOUL_NAMES.get_name(NamesInternalType::String).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(NamesInternalType::Float64).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "num".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::Float64).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        },
    ];

    //push default ctors
    for type_name in enum_iterator::all::<NamesInternalType>() {

        functions.push(FunctionDeclaration{
            name: SOUL_NAMES.get_name(type_name).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(type_name).to_string()),
            args: vec![],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        });
    }

    for type_name in NAMES_INTERNAL_TYPE_NUMBER {
        functions.push(FunctionDeclaration{
            name: SOUL_NAMES.get_name(type_name).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(type_name).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "value".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        });

        functions.push(FunctionDeclaration{
            name: SOUL_NAMES.get_name(type_name).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(type_name).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "value".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::UntypedUint).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        });

        functions.push(FunctionDeclaration{
            name: SOUL_NAMES.get_name(type_name).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(type_name).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "value".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::UntypedFloat).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        });

        functions.push(FunctionDeclaration{
            name: SOUL_NAMES.get_name(type_name).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(type_name).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "value".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::Character).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        });

        functions.push(FunctionDeclaration{
            name: SOUL_NAMES.get_name(type_name).to_string(), 
            return_type: Some(SOUL_NAMES.get_name(type_name).to_string()),
            args: vec![
                ArgumentInfo::new_argument(
                    /*name:*/     "value".to_string(), 
                    /*type:*/     SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string(),
                    /*is_mut:*/   false, 
                    /*position:*/ 0,
                )
            ],
            optionals: BTreeMap::new(),
            generics: BTreeMap::new(),
            modifiers: FunctionModifiers::Const,
            id: new_function_id(&mut current_id),
            is_forward_declared: false,
        });
    }

    functions.push(FunctionDeclaration{
        name: SOUL_NAMES.get_name(NamesInternalType::Character).to_string(), 
        return_type: Some(SOUL_NAMES.get_name(NamesInternalType::Character).to_string()),
        args: vec![
            ArgumentInfo::new_argument(
                /*name:*/     "value".to_string(), 
                /*type:*/     SOUL_NAMES.get_name(NamesInternalType::UntypedFloat).to_string(),
                /*is_mut:*/   false, 
                /*position:*/ 0,
            )
        ],
        optionals: BTreeMap::new(),
        generics: BTreeMap::new(),
        modifiers: FunctionModifiers::Const,
        id: new_function_id(&mut current_id),
            is_forward_declared: false,
    });

    functions.push(FunctionDeclaration{
        name: SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string(), 
        return_type: Some(SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string()),
        args: vec![
            ArgumentInfo::new_argument(
                /*name:*/     "value".to_string(), 
                /*type:*/     SOUL_NAMES.get_name(NamesInternalType::UntypedFloat).to_string(),
                /*is_mut:*/   false, 
                /*position:*/ 0,
            )
        ],
        optionals: BTreeMap::new(),
        generics: BTreeMap::new(),
        modifiers: FunctionModifiers::Const,
        id: new_function_id(&mut current_id),
            is_forward_declared: false,
    });

    functions
});