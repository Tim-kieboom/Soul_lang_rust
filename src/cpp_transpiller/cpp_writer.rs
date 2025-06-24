pub struct CppWriter {
    buffer: String,
    pretty: bool,
    tab: usize,
}
impl CppWriter {
    pub fn new(pretty: bool) -> Self {
        Self { buffer: String::new(), pretty, tab: 0 }
    }

    pub fn push(&mut self, char: char) {
        self.buffer.push(char);
    }

    pub fn push_str(&mut self, str: &str) {
        self.buffer.push_str(str);
    }

    pub fn start_line(&mut self) {
        
        if self.pretty {
            
            for _ in 0..self.tab {
                self.push('\t');
            }
        }
    }

    pub fn push_tab(&mut self) {
        self.tab += 1
    }
    
    pub fn pop_tab(&mut self) {
        self.tab -= 1
    }

    pub fn end_line(&mut self) {
        
        if self.pretty {
            self.push('\n');
        }
    }

    pub fn consume_to_string(self) -> String {
        self.buffer
    }
}









