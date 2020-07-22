use crate::types::Repository;

pub struct Store {
    repositories: Vec<Repository>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }
}
