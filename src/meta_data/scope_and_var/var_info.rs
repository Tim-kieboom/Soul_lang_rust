use bitflags::bitflags;
bitflags! {
    #[derive(Debug, Clone)]
    pub struct VarFlags: u8 {
        const Empty = 0;
        const IsAssigned = 0b0000_0010;
        const IsMutable = 0b0000_0100;
        const IsLiteral = 0b0000_1000;
    }
}


#[derive(Debug, Clone)]
pub struct VarInfo {
    pub name: String,
    pub type_name: String,
    var_flags: VarFlags,
    pub is_forward_declared: bool,
    // pub methodes: HashMap<String, NotYetImpl>,
}

impl VarInfo {
    pub fn new(name: String, type_name: String) -> Self {
        VarInfo {name, type_name, var_flags: VarFlags::Empty, is_forward_declared: false}
    }

    pub fn with_var_flag(name: String, type_name: String, flag: VarFlags, is_forward_declared: bool) -> Self {
        VarInfo {name, type_name, var_flags: flag, is_forward_declared}
    }

    pub fn add_var_flag(&mut self, flag: VarFlags) {
        self.var_flags.insert(flag);
    }

    pub fn remove_var_flag(&mut self, flag: VarFlags) {
        self.var_flags.remove(flag); 
    }

    pub fn is_assigned(&self) -> bool {
        self.var_flags.contains(VarFlags::IsAssigned)
    }

    pub fn is_mutable(&self) -> bool {
        self.var_flags.contains(VarFlags::IsMutable)
    }

    pub fn is_literal(&self) -> bool {
        self.var_flags.contains(VarFlags::IsLiteral)
    }

    pub fn get_raw_var_flags(&self) -> &VarFlags {
        &self.var_flags
    }
}



