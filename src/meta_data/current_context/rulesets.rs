use core::fmt;
use std::fmt::Display;
use bitflags::bitflags;
use serde::Serializer;

bitflags! {
    #[derive(Clone, PartialEq)]
    pub struct RuleSet: u8  {
        const Default = 0b0000_0000;
        const Const = 0b0000_0001;
        const Literal = 0b0000_0010;
        const Unsafe = 0b0000_0100;
        const BorrowChecked = 0b0000_1000;
        const GarbageCollected = 0b0001_0000;
    } 
}

impl RuleSet {
    pub fn is_mutable(&self) -> bool {
        !(self.contains(RuleSet::Literal) || self.contains(RuleSet::Const))
    }
}


impl fmt::Debug for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let mut flags = vec![];
        if self.is_empty() {
            flags.push("Default");
        } 
        else {
            if self.contains(RuleSet::Const)            { flags.push("Const"); }
            if self.contains(RuleSet::Literal)          { flags.push("Literal"); }
            if self.contains(RuleSet::Unsafe)           { flags.push("Unsafe"); }
            if self.contains(RuleSet::BorrowChecked)    { flags.push("BorrowChecked"); }
            if self.contains(RuleSet::GarbageCollected) { flags.push("GarbageCollected"); }
        }

        f.debug_tuple("RuleSet")
            .field(&flags.join(" | "))
            .finish()
    }
}

