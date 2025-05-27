use std::collections::HashMap;

use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct ShowTimes: u8 {
        const SHOW_NONE = 0x0;
        const SHOW_TOTAL = 0b0000_0001;
        const SHOW_TOKENIZER = 0b0000_0010;
        const SHOW_ABSTRACT_SYNTAX_TREE = 0b0000_0100;
        const SHOW_CPP_CONVERTION = 0b0000_1000;
        const SHOW_ALL = 0b1111_1111;
    }
}

const OPTIONS: &[(&str, ShowTimes)] = &[
    ("SHOW_TOTAL", ShowTimes::SHOW_TOTAL),
    ("SHOW_TOKENIZER", ShowTimes::SHOW_TOKENIZER),
    ("SHOW_ABSTRACT_SYNTAX_TREE", ShowTimes::SHOW_ABSTRACT_SYNTAX_TREE),
    ("SHOW_CPP_CONVERTION", ShowTimes::SHOW_CPP_CONVERTION)
];

impl ShowTimes {
    pub fn from_str(str: &str) -> Result<Self, String> {
        let mut this = ShowTimes::SHOW_NONE;
        if str == "SHOW_NONE" {
            return Ok(this);
        }
        if str == "SHOW_ALL" {
            return Ok(ShowTimes::SHOW_ALL);
        }

        validate_options(str)?;
        
        let splits = str.split("+");
        let options = splits
            .filter_map(|token| OPTIONS.iter().find(|op| op.0 == token))
            .map(|option| option.1);

        for option in options {
            this |= option;
        }

        Ok(this)
    }

    pub fn to_string(&self) -> String {
        if self == &ShowTimes::SHOW_NONE {
            return "SHOW_NONE".to_string();
        }

        let mut sb = Vec::with_capacity(8);

        for (name, option) in OPTIONS {
            self.contains(*option);
            sb.push(*name);
        }

        let string;
        if sb.len() > 1 {
            string = sb.join("+")
        } 
        else {
            string = sb[0].to_string()
        }

        string
    }
}

fn validate_options(str: &str) -> Result<(), String> {

    let splits = str.split("+");
    let mut counts: HashMap<&str, usize> = HashMap::new();

    for token in splits {
        if !OPTIONS.iter().any(|op| op.0 == token) {
            return Err(format!("option: '{}' is not known option", token));
        }
        *counts.entry(token).or_insert(0) += 1;
    }

    for (option, count) in counts {
        if count > 1 {
            return Err(format!("option: '{}' is specified more than once", option));
        }
    }

    Ok(())
}

