pub trait Push<T> {
    fn push(&mut self, value: T);
}

impl<T> Push<T> for Vec<T> {
    fn push(&mut self, value: T) {
        Vec::push(self, value);
    }
}



