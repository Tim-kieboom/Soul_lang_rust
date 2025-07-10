use regex::Regex;

pub trait SplitOn {fn split_on(&self, delims: &[&str]) -> Vec<&str>;}
impl SplitOn for &str {
    fn split_on(&self, delims: &[&str]) -> Vec<&str> {
        let regex = Regex::new(
            &delims.iter()
                .map(|token| regex::escape(token))
                .collect::<Vec<String>>()
                .join("|")
        ).unwrap();
        
        let mut result = Vec::with_capacity(self.len() / 4);
        let mut last_end = 0;

        for find in regex.find_iter(self) {
            if find.start() > last_end {
                result.push(&self[last_end..find.start()]);
            }

            result.push(find.as_str());
            last_end = find.end();
        }

        if last_end < self.len() {
            result.push(&self[last_end..]);
        }

        result
    }
}

impl SplitOn for String {
    fn split_on(&self, delims: &[&str]) -> Vec<&str> {
        let regex = Regex::new(
            &delims.iter()
                .map(|token| regex::escape(token))
                .collect::<Vec<String>>()
                .join("|")
        ).unwrap();
        
        let mut result = Vec::with_capacity(self.len() / 4);
        let mut last_end = 0;

        for find in regex.find_iter(self) {
            if find.start() > last_end {
                result.push(&self[last_end..find.start()]);
            }

            result.push(find.as_str());
            last_end = find.end();
        }

        if last_end < self.len() {
            result.push(&self[last_end..]);
        }

        result
    }
}












