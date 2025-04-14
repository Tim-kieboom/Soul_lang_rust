use std::collections::HashMap;

#[repr(u16)]
pub enum VarFlags {
    None = 0,
    IsAssigned = 1,
    IsLiteral = 2,
    IsBorrowchecked = 4,
}

pub struct VarInfo {
    pub name: String,
    pub type_name: String,
    var_flags: u16,
    // pub methodes: HashMap<String, NotYetImpl>,
}

impl VarInfo {
    pub fn new(name: String, type_name: String) -> Self {
        VarInfo {name, type_name, var_flags: 0}
    }

    pub fn set_var_flag(&mut self, flag: VarFlags) {
        self.var_flags |= flag as u16
    }

    pub fn clear_var_flag(&mut self, flag: VarFlags) {
        self.var_flags &= !(flag as u16); 
    }

    pub fn is_assigned(&self) -> bool {
        self.var_flags & VarFlags::IsAssigned as u16 == 1
    }

    pub fn is_literal(&self) -> bool {
        self.var_flags & VarFlags::IsLiteral as u16 == 1
    }

    pub fn is_borrow_checked(&self) -> bool {
        self.var_flags & VarFlags::IsBorrowchecked as u16 == 1
    }
}