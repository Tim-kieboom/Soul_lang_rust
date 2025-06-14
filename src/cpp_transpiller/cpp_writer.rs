pub struct CppWriter {
    buffer: String
}
impl CppWriter {
    pub fn new() -> Self {
        Self { buffer: String::new() }
    }

    pub fn push(&mut self, char: char) {
        self.buffer.push(char);
    }

    pub fn push_str(&mut self, str: &str) {
        self.buffer.push_str(str);
    }

    pub fn consume_to_string(self) -> String {
        self.buffer
    }
}









