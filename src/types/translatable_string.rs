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

    pub fn with_default(text: &str) -> Self {
        let mut t = Self::new();
        t.add_for_lang("default", text);
        t
    }

    pub fn add_for_lang(&mut self, lang: &str, text: &str) {
        self.variants.insert(lang.into(), text.to_string());
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
            .or_insert_with(|| vec![text.to_string()]);
    }
}
