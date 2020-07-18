use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, PartialEq)]
pub struct TranslatableString {
    pub variants: HashMap<String, String>,
}

impl TranslatableString {
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Default)]
pub struct TranslatableVec {
    pub variants: HashMap<String, Vec<String>>,
}

impl TranslatableVec {
    pub fn new() -> Self {
        Self {
            variants: HashMap::new(),
        }
    }

    pub fn add_for_lang(&mut self, lang: &str, text: &str) {
        self.variants
            .entry(lang.into())
            .and_modify(|sentenses| {
                sentenses.push(text.into());
            })
            .or_insert(vec![text.to_string()]);
    }
}
