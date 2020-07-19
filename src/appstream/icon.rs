use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Icon {
    Stock(String),
    Cached(String),
    Remote {
        url: Url,
        width: Option<u32>,
        height: Option<u32>,
    },
    Local {
        path: PathBuf,
        width: Option<u32>,
        height: Option<u32>,
    },
}
/*
#[cfg(test)]
mod tests {
    use super::Url;
    use crate::appstream::Icon;
    use quick_xml::de::from_str;
    use std::str::FromStr;

    #[test]
    fn stock_icon() {
        let x = r"<icon type='stock'>gimp</icon>";
        let i: Icon = from_str(&x).unwrap();
        assert_eq!(i, Icon::Stock("gimp".into()));
    }

    #[test]
    fn cached_icon() {
        let x = r"<icon type='cached'>firefox.png</icon>";
        let i: Vec<Icon> = from_str(&x).unwrap();
        assert_eq!(i.first().unwrap(), &Icon::Cached("firefox.png".into()));
    }
    #[test]
    fn remote_icon() {
        let x = r"<icon type='remote' width='64' height='64'>https://example.com/icons/foobar.png</icon>";
        let i: Icon = from_str(&x).unwrap();
        assert_eq!(
            i,
            Icon::Remote(
                Url::from_str("https://example.com/icons/foobar.png").unwrap(),
                Some(64),
                Some(64)
            )
        );
    }

    #[test]
    fn local_icon() {
        let x = r"<icon type='local' width='64' height='64'>/usr/share/pixmaps/foobar.png</icon>";
        let i: Icon = from_str(&x).unwrap();
        assert_eq!(
            i,
            Icon::Local("/usr/share/pixmaps/foobar.png".into(), Some(64), Some(64))
        );
    }
}
*/
