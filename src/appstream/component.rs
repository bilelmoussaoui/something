use super::de::*;
use super::{
    AppId, ApplicationType, Bundle, Category, ContentRating, Icon, Kudo, Language, Launchable,
    ProjectUrl, Provide, Release, Screenshot,
};
use crate::types::{TranslatableString, TranslatableVec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Component {
    #[serde(
        rename = "type",
        deserialize_with = "component_type_deserialize",
        default
    )]
    pub _type: ApplicationType,
    #[serde(deserialize_with = "app_id_deserialize")]
    pub id: AppId,
    #[serde(rename = "name", deserialize_with = "translatable_deserialize")]
    pub name: TranslatableString,
    #[serde(rename = "summary", deserialize_with = "translatable_deserialize")]
    pub summary: TranslatableString,
    pub project_license: Option<String>,
    pub metadata_license: Option<String>,
    pub project_group: Option<String>,
    pub compulsory_for_desktop: Option<String>,
    pub extends: Option<String>,

    #[serde(rename = "icon", default)]
    pub icons: Vec<Icon>,
    #[serde(deserialize_with = "screenshots_deserialize", default)]
    pub screenshots: Vec<Screenshot>,
    #[serde(rename = "url", deserialize_with = "urls_deserialize", default)]
    pub urls: Vec<ProjectUrl>,
    #[serde(
        rename = "developer_name",
        deserialize_with = "some_translatable_deserialize",
        default
    )]
    pub developer_name: Option<TranslatableString>,
    pub update_contact: Option<String>,
    #[serde(default, deserialize_with = "category_deserialize")]
    pub categories: Vec<Category>,
    #[serde(
        rename = "launchable",
        deserialize_with = "launchable_deserialize",
        default
    )]
    pub launchables: Vec<Launchable>,
    #[serde(rename = "bundle", default)]
    pub bundle: Vec<Bundle>,
    #[serde(default, deserialize_with = "releases_deserialize")]
    pub releases: Vec<Release>,
    #[serde(deserialize_with = "languages_deserialize", default)]
    pub languages: Vec<Language>,

    #[serde(default, deserialize_with = "mimetypes_deserialize")]
    pub mimetypes: Vec<String>,
    #[serde(default, deserialize_with = "kudos_deserialize")]
    pub kudos: Vec<Kudo>,

    #[serde(default, deserialize_with = "keywords_deserialize")]
    pub keywords: TranslatableVec,
    #[serde(default, deserialize_with = "content_rating_deserialize")]
    pub content_rating: Option<ContentRating>,
    #[serde(default, deserialize_with = "provides_deserialize")]
    pub provides: Vec<Provide>,
}
