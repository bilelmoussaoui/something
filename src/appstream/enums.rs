use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum_macros::EnumString;
use url::Url;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ComponentType {
    Runtime,
    #[serde(alias = "console")]
    ConsoleApplication,
    #[serde(alias = "desktop")]
    DesktopApplication,
    #[serde(rename = "inputmethod")]
    InputMethod,
    #[serde(alias = "operating-system")]
    OS,
    Theme,
    Firmware,
    Addon,
    Font,
    Generic,
    IconTheme,
    Localization,
    Driver,
    Codec,
}

impl Default for ComponentType {
    fn default() -> Self {
        ComponentType::Generic
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

#[derive(Debug, Deserialize, Serialize, PartialEq, EnumString)]
pub enum Kudo {
    AppMenu,
    HiDpiIcon,
    HighContrast,
    ModernToolkit,
    Notifications,
    SearchProvider,
    UserDocs,
}
pub enum Arch {
    Arm,
    Aarch64,
    I386,
    X86_64,
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
    Codec(String),
}

#[test]
fn test_provide_firmware() {
    let x = r"<firmware type='runtime'>ipw2200-bss.fw</firmware>";
    let p: Provide = quick_xml::de::from_str(&x).unwrap();
    assert_eq!(p, Provide::Firmware("ipw2200-bss.fw".into()));
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
