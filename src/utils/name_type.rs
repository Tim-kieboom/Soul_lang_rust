use itertools::Itertools;

/// Represents different naming conventions for identifiers.
///
/// Variants include:
/// - `SingleLetterLowercase`: A single lowercase letter (e.g., "a").
/// - `SingleLetterCapital`: A single uppercase letter (e.g., "A").
/// - `CamelCase`: camelCase style (e.g., "camelCase").
/// - `PascalCase`: PascalCase style (e.g., "PascalCase").
/// - `SnakeCase`: snake_case style (e.g., "snake_case").
/// - `ScreamingSnakeCase`: SCREAMING_SNAKE_CASE style (e.g., "SCREAMING_SNAKE_CASE").
#[derive(Debug, Clone)]
pub enum NameType {
    /// only 1 char lowercase
    SingleLetterLowercase,
    /// only 1 char uppercase, 
    SingleLetterCapital, 

    ///camelCase
    CamelCase, 
    ///PascalCase
    PascalCase, 
    ///snake_case
    SnakeCase, 
    ///SCREAMING_SNAKE_CASE
    ScreamingSnakeCase, 
}
impl NameType {
    /// Creates a new `NameType` from a `name` string by inspecting its characteristics.
    ///
    /// Returns `Ok(NameType)` if it matches one of the naming conventions,
    /// or `Err` if the name violates rules like starting with a capital in snake_case.
    fn new(name: &str) -> Result<Self, String> {
        let propertys = NamingPropertys::from_name(name);

        if propertys.all_capital {

            if propertys.name_len == 1 {
                Ok(NameType::SingleLetterCapital)
            }
            else {
                Ok(NameType::ScreamingSnakeCase)
            }
        }
        else if propertys.has_middle_underscore {

            if propertys.has_start_capital {
                Err(format!("name: '{}' snake_case can not start with a capital letter", name))
            }
            else if propertys.has_middle_capital {
                Err(format!("name: '{}' snake_case can not have capital letters", name))
            }
            else {
                Ok(Self::SnakeCase)
            }
        }
        else {

            if propertys.has_start_capital {
                Ok(Self::PascalCase)
            }
            else if propertys.has_middle_capital {
                Ok(Self::CamelCase)
            }
            else {
                Ok(Self::CamelCase)
            }
        }
    }

    /// Checks if a given `name` matches any of the `name_type` possibilities.
    ///
    /// Returns `Ok(())` if any matches, otherwise an `Err` with a description of mismatch.
    pub fn could_be(name: &str, name_type: &[NameType]) -> Result<(), String> {
        let this = Self::new(name)?;

        let mut passed = name_type.len();
        for ty in name_type {

            if let Err(_) = this.inner_should_be(name, ty) {
                passed -= 1;
            }
        }
        
        if passed == 0 {
            Err(format!("name: '{}' could be ['{}'] but is {}", name, name_type.iter().map(|el| el.to_str()).join("' or '"), this.to_str()))
        }
        else {
            Ok(())
        }
    }

    /// Checks if a given `name` exactly matches the specified `name_type`.
    ///
    /// Returns `Ok(())` if it matches, otherwise an `Err` describing the mismatch.
    pub fn should_be(name: &str, name_type: &NameType) -> Result<(), String> {
        let this = Self::new(name)?;
        this.inner_should_be(name, name_type)
    }
    
    /// Returns the string representation of the naming style.
    ///
    /// Examples: "camelCase", "PascalCase", "snake_case", etc.
    pub const fn to_str(&self) -> &str {
        match self {
            NameType::CamelCase => "camelCase",
            NameType::PascalCase => "PascalCase",
            NameType::SnakeCase => "snake_case",
            NameType::ScreamingSnakeCase => "SCREAMING_SNAKE_CASE",
            NameType::SingleLetterLowercase => "singleLetterLowercase",
            NameType::SingleLetterCapital => "singleLetterCapital",
        }
    }

    fn inner_should_be(&self, name: &str, name_type: &NameType) -> Result<(), String> {
        match self {
            NameType::SingleLetterLowercase => {
                
                if name_type.eq(&Self::SnakeCase) || name_type.eq(&Self::CamelCase) {
                    return Ok(())
                }
            },
            NameType::SingleLetterCapital => {
                
                if name_type.eq(&Self::ScreamingSnakeCase) || name_type.eq(&Self::PascalCase) {
                    return Ok(())
                }
            },

            _ => if self.eq(&name_type) {
                return Ok(())
            },
        }

        Err(format!("name: '{}' should be {} but is {}", name, name_type.to_str(), self.to_str()))
    }

    ///made this fn to avoid impl Eq Trait to force use of should_be() and could_be()
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

struct NamingPropertys {
    pub has_start_capital: bool,
    pub has_middle_capital: bool,
    pub has_middle_underscore: bool,
    pub all_capital: bool,
    pub name_len: usize,
}

impl NamingPropertys {

    pub fn from_name(name: &str) -> Self {
    
        let mut chars = name.chars();
        let has_start_capital = chars.next().is_some_and(|char| char.is_uppercase());
        let mut has_middle_capital = false;
        let mut has_middle_underscore = false;
        let mut all_capital = has_start_capital;

        for char in chars {
            if char.is_uppercase() {
                has_middle_capital = true;
            }
            else if char != '_' {
                all_capital = false;
            }
            
            if char == '_' {
                has_middle_underscore = true;
            }
        }

        Self{
            has_start_capital,
            has_middle_capital, 
            has_middle_underscore,
            all_capital,
            name_len: name.len(), 
        }
    }
}
