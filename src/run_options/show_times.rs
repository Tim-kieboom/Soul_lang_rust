use std::collections::HashMap;
use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct ShowTimes: u8 {
        const SHOW_NONE = 0x0;
        const SHOW_TOTAL = 0b0000_0001;
        const SHOW_SOURCE_READER = 0b0001_0000;
        const SHOW_TOKENIZER = 0b0000_0010;
        const SHOW_PARSER = 0b0000_0100;
        const SHOW_SEMENTIC_ANALYSER = 0b0001_0000;
        const SHOW_CODE_GENERATOR = 0b0000_1000;
        const SHOW_ALL = 0b1111_1111;
    }
}

const OPTIONS: &[(&str, ShowTimes)] = &[
    ("SHOW_TOTAL", ShowTimes::SHOW_TOTAL),
    ("SHOW_SOURCE_READER", ShowTimes::SHOW_SOURCE_READER),
    ("SHOW_TOKENIZER", ShowTimes::SHOW_TOKENIZER),
    ("SHOW_PARSER", ShowTimes::SHOW_PARSER),
    ("SHOW_SEMENTIC_ANALYSER", ShowTimes::SHOW_SEMENTIC_ANALYSER),
    ("SHOW_CODE_GENERATOR", ShowTimes::SHOW_CODE_GENERATOR),
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

