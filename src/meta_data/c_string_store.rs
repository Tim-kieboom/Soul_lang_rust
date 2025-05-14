use std::{collections::HashMap, sync::Arc};

pub struct CStrPair {
    pub name: String,
    pub c_str: String,
}

pub struct CStringStore {
    from_c_str_map: HashMap<String, Arc<CStrPair>>,
    from_name_map: HashMap<String, Arc<CStrPair>>,
}

#[allow(dead_code)]
impl CStringStore {

    pub fn new() -> Self {
        CStringStore{ 
            from_c_str_map: HashMap::new(), 
            from_name_map: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.from_c_str_map.len()
    }

    pub fn add(&mut self, c_str: String, str_name: String) {
        let value = Arc::new(CStrPair{name: str_name.clone(), c_str: c_str.clone()});
        self.from_c_str_map.insert(c_str, value.clone());
        self.from_name_map.insert(str_name, value);
    }

    pub fn from_c_str(&self, c_str: &str) -> Option<&Arc<CStrPair>> {
        self.from_c_str_map.get(c_str)
    }

    pub fn from_name(&self, str_name: &str) -> Option<&Arc<CStrPair>> {
        self.from_name_map.get(str_name)
    }

    pub fn from_c_str_map(&self) -> &HashMap<String, Arc<CStrPair>> {
        &self.from_c_str_map
    }

    pub fn from_name_map(&self) -> &HashMap<String, Arc<CStrPair>> {
        &self.from_name_map
    }
}




