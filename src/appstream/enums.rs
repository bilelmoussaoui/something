use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Serialize, PartialEq)]
pub enum ApplicationType {
    Runtime,
    Cli,
    Desktop,
    Theme,
    Addon,
    Unknown(String),
}

impl From<&str> for ApplicationType {
    fn from(t: &str) -> Self {
        match t {
            "desktop" | "desktop-application" => ApplicationType::Desktop,
            "cli-application" | "cli" => ApplicationType::Cli,
            "runtime" => ApplicationType::Runtime,
            "theme" => ApplicationType::Theme,
            "addon" => ApplicationType::Addon,
            e => ApplicationType::Unknown(e.to_string()),
        }
    }
}

impl Default for ApplicationType {
    fn default() -> Self {
        ApplicationType::Unknown("unknown".to_string())
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub enum Launchable {
    DesktopId(String),
    Service(String),
    Url(Url),
    CockpitManifest(String),
    Unknown(String),
}

#[derive(Debug, Serialize, PartialEq)]
pub enum ProjectUrl {
    Donation(Url),
    Translate(Url),
    Homepage(Url),
    BugTracker(Url),
    Help(Url),
    Unknown(Url),
}

#[derive(Debug, Serialize, PartialEq)]
pub enum Kudo {
    HiDpiIcon,
    HighContrast,
    ModernToolkit,
    Notifications,
    SearchProvider,
    UserDocs,
    Unknown(String),
}

impl FromStr for Kudo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HiDpiIcon" => Ok(Kudo::HiDpiIcon),
            "HighContrast" => Ok(Kudo::HighContrast),
            "ModernToolkit" => Ok(Kudo::ModernToolkit),
            "Notifications" => Ok(Kudo::Notifications),
            "SearchProvider" => Ok(Kudo::SearchProvider),
            "UserDocs" => Ok(Kudo::UserDocs),
            e => Ok(Kudo::Unknown(e.into())),
        }
    }
}

use std::convert::TryFrom;
#[non_exhaustive]
pub enum Arch {
    Arm,
    Aarch64,
    I386,
    X86_64,
}

impl TryFrom<&'static str> for Arch {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "arm" => Ok(Arch::Arm),
            "aarch64" => Ok(Arch::Aarch64),
            "i386" => Ok(Arch::I386),
            "x86_64" => Ok(Arch::X86_64),
            _ => Err("Failed to find the corresponding arch"),
        }
    }
}

impl Into<&'static str> for Arch {
    fn into(self) -> &'static str {
        match self {
            Arch::Arm => "arm",
            Arch::Aarch64 => "aarch64",
            Arch::I386 => "i386",
            Arch::X86_64 => "x86_64",
        }
    }
}
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Provide {
    Library(PathBuf),
    Binary(String),
    Font(String),
    Modalias(String),
    Firmware(String),
    Python2(String),
    Python3(String),
    Dbus(String),
    Id(String),
    Unknown(String),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "id", content = "$value")]
pub enum ContentAttribute {
    #[serde(rename = "violence-cartoon")]
    ViolenceCartoon(ContentState),
    #[serde(rename = "violence-fantasy")]
    ViolenceFantasy(ContentState),
    #[serde(rename = "violence-fealistic")]
    ViolenceFealistic(ContentState),
    #[serde(rename = "violence-bloodshed")]
    ViolenceBloodshed(ContentState),
    #[serde(rename = "violence-sexual")]
    ViolenceSexual(ContentState),
    #[serde(rename = "violence-desecration")]
    ViolenceDesecration(ContentState),
    #[serde(rename = "violence-slavery")]
    ViolenceSlavery(ContentState),
    #[serde(rename = "violence-realistic")]
    ViolenceRealistic(ContentState),
    #[serde(rename = "violence-worship")]
    ViolenceWorship(ContentState),
    #[serde(rename = "drugs-alcohol")]
    DrugsAlcohol(ContentState),
    #[serde(rename = "drugs-narcotics")]
    DrugsNarcotics(ContentState),
    #[serde(rename = "drugs-tobacco")]
    DrugsTobacco(ContentState),
    #[serde(rename = "sex-nudity")]
    SexNudity(ContentState),
    #[serde(rename = "sex-themes")]
    SexThemes(ContentState),
    #[serde(rename = "sex-homosexuality")]
    SexHomosexuality(ContentState),
    #[serde(rename = "sex-prostitution")]
    SexProstitution(ContentState),
    #[serde(rename = "sex-adultery")]
    SexAdultery(ContentState),
    #[serde(rename = "sex-appearance")]
    SexAppearance(ContentState),
    #[serde(rename = "language-profanity")]
    LanguageProfanity(ContentState),
    #[serde(rename = "language-humor")]
    LanguageHumor(ContentState),
    #[serde(rename = "language-discrimination")]
    LanguageDiscrimination(ContentState),
    #[serde(rename = "social-chat")]
    SocialChat(ContentState),
    #[serde(rename = "social-info")]
    SocialInfo(ContentState),
    #[serde(rename = "social-audio")]
    SocialAudio(ContentState),
    #[serde(rename = "social-location")]
    SocialLocation(ContentState),
    #[serde(rename = "social-contacts")]
    SocialContacts(ContentState),
    #[serde(rename = "money-advertising")]
    MoneyAdvertising(ContentState),
    #[serde(rename = "money-purchasing")]
    MoneyPurchasing(ContentState),
    #[serde(rename = "money-gambling")]
    MoneyGambling(ContentState),
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum ContentState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "mild")]
    Mild,
    #[serde(rename = "moderate")]
    Moderate,
    #[serde(rename = "intense")]
    Intense,
}
