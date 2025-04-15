use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone)]
    pub struct RuleSet: u8 {
        const EMPTY = 0b0000_0000;
        const CONST = 0b0000_0001;
        const LITERAL = 0b0000_0010;
        const UNSAFE = 0b0000_0100;
        const BORROW_CHECKED = 0b0000_1000;
        const GARBAGE_COLLECTED = 0b0001_0000;
    } 
}

