use once_cell::sync::Lazy;
use std::{collections::{BTreeSet, HashMap}, result, sync::Arc};
use crate::abstract_styntax_tree::{assign_type::AssignType, operator_type::{ExprOperatorType, ALL_OPERATORS, BOOLEAN_OPERATOR}};
use super::{soul_names::{NamesInternalType, NAMES_INTERNAL_TYPE_NUMBER, SOUL_NAMES}, soul_type::{soul_type::SoulType, type_wrappers::{TypeWrappers, ALL_TYPE_WRAPPERS}}};

static NO_DEFAULT_OPERATORS: Lazy<ImplOperators> = Lazy::new(|| {
    ImplOperators{
        operator: BTreeSet::new(),
    }
});

static NUMBER_DEFAULT_OPERATORS: Lazy<ImplOperators> = Lazy::new(|| {
    ImplOperators{
        operator: ALL_OPERATORS.iter().cloned().map(|op| ImplOperator::ExprOperatorType(op)).collect(),
    }
});

static BOOLEAN_DEFAULT_OPERATORS: Lazy<ImplOperators> = Lazy::new(|| {
    ImplOperators{
        operator: BOOLEAN_OPERATOR.iter().cloned().map(|op| ImplOperator::ExprOperatorType(op)).collect(),
    }
});

static DEFAULT_TYPESTORE: Lazy<TypeStore> = Lazy::new(|| {
    let mut this = TypeStore{ 
        to_id: HashMap::new(), 
        to_name: HashMap::new(), 
        typedef_store: HashMap::new(),
        highest_id: TypeID(0),
        implemented_type_operators: HashMap::new(),
        implemented_wrapper_operators: HashMap::new(),
    };

    add_internal_types(&mut this);

    return this;
});

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeID(pub u64);
#[derive(Debug, Clone)]
pub struct TypeDef {pub type_id: TypeID, pub from_stringed: String}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImplOperator {
    ExprOperatorType(ExprOperatorType),
    AssignType(AssignType),
}

impl ImplOperator {
    pub fn to_str(&self) -> &str {
        match self {
            ImplOperator::ExprOperatorType(expr_operator_type) => expr_operator_type.to_str(),
            ImplOperator::AssignType(assign_type) => assign_type.to_str(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct ImplOperators {
    pub operator: BTreeSet<ImplOperator>,
}

#[derive(Debug, Clone)]
pub struct TypeStore {
    pub to_id: HashMap<Arc<String>, TypeID>,
    pub to_name: HashMap<TypeID, Arc<String>>,
    pub typedef_store: HashMap<TypeID, TypeDef>,
    pub highest_id: TypeID,
    pub implemented_type_operators: HashMap<TypeID, ImplOperators>,
    pub implemented_wrapper_operators: HashMap<TypeWrappers, ImplOperators>,
}

impl TypeStore {
    pub fn new() -> Self {
        DEFAULT_TYPESTORE.clone()
    }

    pub fn convert_typedef_to_original(&self, id: &TypeID) -> Option<&String> {
        self.typedef_store.get(id).map(|val| &val.from_stringed)
    }

    pub fn add_type(&mut self, name: String, possible_from_type: Option<String>) -> result::Result<TypeID, String> {
        let arc_name = Arc::new(name.clone());
        self.highest_id = TypeID(self.highest_id.0 + 1);

        if self.to_id.get(&arc_name).is_some() {
            return Err(format!("type: '{}' already exsists", name));
        }

        if let Some(from_type) = possible_from_type  {
            self.typedef_store.insert(self.highest_id, TypeDef{type_id: self.highest_id, from_stringed: from_type});
        }

        self.to_id.insert(arc_name.clone(), self.highest_id);
        self.to_name.insert(self.highest_id, arc_name.clone());

        Ok(self.highest_id)
    }
}

fn add_internal_types(type_store: &mut TypeStore) {
    for number in NAMES_INTERNAL_TYPE_NUMBER {
        let number_name = SOUL_NAMES.get_name(number).to_string(); 
        let id = type_store.add_type(number_name, None).expect("unexpeced error from internal impl_primitive_type operators");

        type_store.implemented_type_operators.insert(id, NUMBER_DEFAULT_OPERATORS.clone());
    }

    for bool_ops in [NamesInternalType::Boolean, NamesInternalType::Character] {
        let bool_ops_name = SOUL_NAMES.get_name(bool_ops).to_string(); 
        let id = type_store.add_type(bool_ops_name, None).expect("unexpeced error from internal impl_primitive_type operators");

        type_store.implemented_type_operators.insert(id, BOOLEAN_DEFAULT_OPERATORS.clone());
    }

    let array_operators: ImplOperators = ImplOperators{operator: BTreeSet::from([ExprOperatorType::Equals, ExprOperatorType::NotEquals, ExprOperatorType::Add].map(|op| ImplOperator::ExprOperatorType(op)))};
    for wrap in ALL_TYPE_WRAPPERS {
        let ops = match wrap {
            TypeWrappers::ConstRef |
            TypeWrappers::MutRef => BOOLEAN_DEFAULT_OPERATORS.clone(),
            TypeWrappers::Pointer => NO_DEFAULT_OPERATORS.clone(),
            TypeWrappers::Array => array_operators.clone(),

            TypeWrappers::Invalid => NO_DEFAULT_OPERATORS.clone(),
        };
        
        
        type_store.implemented_wrapper_operators.insert(wrap, ops);
    }

    let string_name = SOUL_NAMES.get_name(NamesInternalType::String).to_string(); 
    let char_array_name = SoulType::from_wrappers(
        SOUL_NAMES.get_name(NamesInternalType::Character).to_string(), 
        vec![TypeWrappers::Array]
    ).to_string();

    let id = type_store.add_type(string_name, Some(char_array_name)).expect("unexpeced error from internal impl_primitive_type operators");

    type_store.implemented_type_operators.insert(id, array_operators);

}








