use super::{
    AppId, ApplicationType, Category, ContentRating, Kudo, Language, Launchable, ProjectUrl,
    Provide, Release, Screenshot,
};
use crate::types::{TranslatableString, TranslatableVec};
use serde::de;
use serde::Deserialize;
use std::str::FromStr;
use url::Url;

pub(crate) fn app_id_deserialize<'de, D>(deserializer: D) -> Result<AppId, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(AppId { 0: s })
}

pub(crate) fn provides_deserialize<'de, D>(deserializer: D) -> Result<Vec<Provide>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PProvides {
        #[serde(rename = "$value", default)]
        pub val: Vec<Provide>,
    };

    let provides = PProvides::deserialize(deserializer)?;
    Ok(provides.val)
}

pub(crate) fn content_rating_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<ContentRating>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let mut contents: Vec<ContentRating> = Vec::deserialize(deserializer)?;
    contents.sort_by(|a, b| a.version.cmp(&b.version));

    Ok(contents.into_iter().next())
}

pub(crate) fn keywords_deserialize<'de, D>(deserializer: D) -> Result<TranslatableVec, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PKeywords {
        #[serde(rename = "keyword")]
        keywords: Vec<PKeyword>,
    };
    #[derive(Debug, Deserialize)]
    struct PKeyword {
        #[serde(rename = "xml:lang", default)]
        pub lang: Option<String>,
        #[serde(rename = "$value")]
        text: String,
    };

    let s: PKeywords = PKeywords::deserialize(deserializer)?;

    let mut translatable = TranslatableVec::new();
    s.keywords.into_iter().for_each(|t| {
        translatable.add_for_lang(&t.lang.unwrap_or("default".to_string()), &t.text);
    });
    Ok(translatable)
}

pub(crate) fn component_type_deserialize<'de, D>(
    deserializer: D,
) -> Result<ApplicationType, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(ApplicationType::from(s.as_str()))
}

pub(crate) fn kudos_deserialize<'de, D>(deserializer: D) -> Result<Vec<Kudo>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize, PartialEq, Default)]
    pub struct Kudos {
        #[serde(rename = "$value", default)]
        kudos: Vec<String>,
    }

    let k: Kudos = Kudos::deserialize(deserializer)?;
    Ok(k.kudos
        .into_iter()
        .map(|k| Kudo::from_str(&k).unwrap())
        .collect::<Vec<Kudo>>())
}

pub(crate) fn mimetypes_deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    pub struct Mimetypes {
        #[serde(rename = "$value", default)]
        mimes: Vec<String>,
    }

    let m: Mimetypes = Mimetypes::deserialize(deserializer)?;
    Ok(m.mimes)
}

pub(crate) fn releases_deserialize<'de, D>(deserializer: D) -> Result<Vec<Release>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PReleases {
        #[serde(rename = "release")]
        releases: Vec<Release>,
    };

    let r: PReleases = PReleases::deserialize(deserializer)?;
    Ok(r.releases)
}

pub(crate) fn languages_deserialize<'de, D>(deserializer: D) -> Result<Vec<Language>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PLanguages {
        #[serde(rename = "lang")]
        languages: Vec<Language>,
    };

    let l: PLanguages = PLanguages::deserialize(deserializer)?;
    Ok(l.languages)
}

pub(crate) fn translatable_deserialize<'de, D>(
    deserializer: D,
) -> Result<TranslatableString, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PTranslatable {
        #[serde(rename = "xml:lang", default)]
        pub lang: Option<String>,
        #[serde(rename = "$value", default)]
        pub val: String,
    };

    let s: Vec<PTranslatable> = Vec::deserialize(deserializer)?;

    let mut translatable = TranslatableString::new();
    s.into_iter().for_each(|t| {
        translatable
            .variants
            .insert(t.lang.unwrap_or("default".to_string()), t.val);
    });
    Ok(translatable)
}

pub(crate) fn some_translatable_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<TranslatableString>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PTranslatable {
        #[serde(rename = "xml:lang", default)]
        pub lang: Option<String>,
        #[serde(rename = "$value", default)]
        pub val: String,
    };

    let s: Option<Vec<PTranslatable>> = Option::deserialize(deserializer)?;

    let mut translatable = TranslatableString::new();
    match s {
        Some(a) => {
            a.into_iter().for_each(|t| {
                translatable
                    .variants
                    .insert(t.lang.unwrap_or("default".to_string()), t.val);
            });
            Ok(Some(translatable))
        }
        None => Ok(None),
    }
}

pub(crate) fn screenshots_deserialize<'de, D>(deserializer: D) -> Result<Vec<Screenshot>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PScreenshot {
        #[serde(rename = "screenshot", default)]
        pub screenshots: Vec<Screenshot>,
    };

    let s: PScreenshot = PScreenshot::deserialize(deserializer)?;
    Ok(s.screenshots)
}

pub(crate) fn launchable_deserialize<'de, D>(deserializer: D) -> Result<Vec<Launchable>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PLaunchable {
        #[serde(rename = "type", default)]
        pub _type: String,
        #[serde(rename = "$value", default)]
        pub val: String,
    };

    let launchables: Vec<PLaunchable> = Vec::deserialize(deserializer)?;

    Ok(launchables
        .into_iter()
        .map(|l| match l._type.as_ref() {
            "desktop-id" => Launchable::DesktopId(l.val),
            "service" => Launchable::Service(l.val),
            "url" => Launchable::Url(Url::from_str(&l.val).unwrap()),
            "cockpit-manifest" => Launchable::CockpitManifest(l.val),
            _ => Launchable::Unknown(l.val),
        })
        .collect::<Vec<Launchable>>())
}

pub(crate) fn category_deserialize<'de, D>(deserializer: D) -> Result<Vec<Category>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PCategory {
        #[serde(rename = "$value", default)]
        pub names: Vec<String>,
    };

    let categories: PCategory = PCategory::deserialize(deserializer)?;
    Ok(categories
        .names
        .into_iter()
        .map(|c| Category::from_str(&c).unwrap())
        .collect::<Vec<Category>>())
}

pub(crate) fn urls_deserialize<'de, D>(deserializer: D) -> Result<Vec<ProjectUrl>, D::Error>
where
    D: de::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    struct PUrl {
        #[serde(rename = "type", default)]
        pub _type: String,
        #[serde(rename = "$value", default)]
        pub url: String,
    };

    let urls: Vec<PUrl> = Vec::deserialize(deserializer)?;

    Ok(urls
        .into_iter()
        .map(|u| {
            let url = Url::from_str(&u.url).expect("Failed to parse url, invalid");
            match u._type.as_str() {
                "homepage" => ProjectUrl::Homepage(url),
                "help" => ProjectUrl::Help(url),
                "donation" => ProjectUrl::Donation(url),
                "bugtracker" => ProjectUrl::BugTracker(url),
                "translate" => ProjectUrl::Translate(url),
                _ => ProjectUrl::Unknown(url),
            }
        })
        .collect::<Vec<ProjectUrl>>())
}
