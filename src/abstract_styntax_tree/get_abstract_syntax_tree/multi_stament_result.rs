use crate::abstract_styntax_tree::abstract_styntax_tree::IStatment;

#[derive(Debug)]
pub struct MultiStamentResult<T> {
    pub before: Option<Vec<IStatment>>,
    pub value: T,
    pub after: Option<Vec<IStatment>>,
}

impl<T> MultiStamentResult<T> {
    pub fn new(value: T) -> Self {
        MultiStamentResult { before: None, value, after: None }
    }

    pub fn add_result<K>(&mut self, other: &MultiStamentResult<K>) {
        if let Some(other) = &other.before {
            self.before.get_or_insert_with(Vec::new)
                       .extend_from_slice(&other);
        }

        if let Some(other) = &other.after {
            self.after.get_or_insert_with(Vec::new)
                      .extend_from_slice(&other);
        }
    }
}





