#[derive(Debug, Clone)]
pub struct NamingPropertys {
    pub has_start_capital: bool,
    pub has_middle_capital: bool,
    pub has_middle_underscore: bool,
}

impl NamingPropertys {

    pub fn from_name(name: &str) -> Self {
    
        let mut chars = name.chars();
        let has_start_capital = chars.next().is_some_and(|char| char.is_uppercase());
        let mut has_middle_capital = false;
        let mut has_middle_underscore = false;

        for char in chars {
            if char.is_uppercase() {
                has_middle_capital = true;
            }
            if char == '_' {
                has_middle_underscore = true;
            }
        }

        Self{
            has_start_capital,
            has_middle_capital, 
            has_middle_underscore, 
        }
    }
}



