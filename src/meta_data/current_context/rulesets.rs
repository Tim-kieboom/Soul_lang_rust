use std::fmt::Display;
use bitflags::bitflags;
use serde::Serializer;

bitflags! {
    #[derive(Debug, Clone)]
    pub struct RuleSet: u8  {
        const Default = 0b0000_0000;
        const Const = 0b0000_0001;
        const Literal = 0b0000_0010;
        const Unsafe = 0b0000_0100;
        const BorrowChecked = 0b0000_1000;
        const GarbageCollected = 0b0001_0000;
    } 
}

impl Display for RuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // List of all flag names and their corresponding values
        let flags = [
            (RuleSet::Const, "Const"),
            (RuleSet::Literal, "Literal"),
            (RuleSet::Unsafe, "Unsafe"),
            (RuleSet::BorrowChecked, "BorrowChecked"),
            (RuleSet::GarbageCollected, "GarbageCollected"),
        ];

        // Collect all set flag names
        let names: Vec<&str> = flags
            .iter()
            .filter_map(|(flag, name)| if self.contains(flag.clone()) { Some(*name) } else { None })
            .collect();

        if names.is_empty() {
            write!(f, "Default")
        } else {
            write!(f, "{}", names.join(" | "))
        }
    }
}
