use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type", content = "$value", rename_all = "kebab-case")]
pub enum Bundle {
    Limba(String),
    Flatpak(String),
}

#[cfg(test)]
mod tests {
    use super::Bundle;
    use quick_xml::de::from_str;

    #[test]
    fn bundle_limbda() {
        let x = r"<bundle type='limba'>foobar-1.0.2</bundle>";
        let b: Bundle = from_str(&x).unwrap();
        assert_eq!(b, Bundle::Limba("foobar-1.0.2".into()));
    }
}
