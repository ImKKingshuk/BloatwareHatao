use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageCategory {
    // OEM-specific categories
    /// Samsung apps
    Samsung,
    /// Google apps and services
    Google,
    /// Xiaomi/MIUI apps
    Xiaomi,
    /// Huawei apps
    Huawei,
    /// Honor apps
    Honor,
    /// OPPO apps
    Oppo,
    /// Vivo apps
    Vivo,
    /// OnePlus apps
    Oneplus,
    /// Realme apps
    Realme,
    /// Motorola apps
    Motorola,
    /// ASUS apps
    Asus,
    /// Sony apps
    Sony,
    /// LG apps
    Lg,
    /// Nokia apps
    Nokia,
    /// Nothing apps
    Nothing,
    /// Meizu apps
    Meizu,
    /// Infinix apps
    Infinix,
    /// Tecno apps
    Tecno,
    /// Itel apps
    Itel,
    /// Amazon apps
    Amazon,
    /// Meta/Facebook apps
    Meta,
    /// Microsoft apps
    Microsoft,
    
    // Functional categories
    /// Generic OEM apps
    Generic,
    /// AOSP/System core
    Aosp,
    /// System services
    System,
    /// Chipset (Qualcomm, MediaTek)
    Chipset,
    /// Carrier-installed apps
    Carrier,
    /// Advertising and analytics
    Ads,
    /// Social media apps
    Social,
    /// Productivity apps
    Productivity,
    /// Entertainment apps
    Entertainment,
    /// Security and antivirus
    Security,
    /// Finance and payment
    Finance,
    /// Health and fitness
    Health,
    /// Gaming
    Gaming,
    /// Shopping
    Shopping,
    /// News and reading
    News,
    /// Education
    Education,
    /// Miscellaneous
    Misc,
    /// User installed apps
    UserInstalled,
    /// Other/unknown
    Other,
}

use serde::de::{self, Visitor, Deserializer};
use std::fmt;

impl<'de> Deserialize<'de> for PackageCategory {
    fn deserialize<D>(deserializer: D) -> Result<PackageCategory, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CategoryVisitor;

        impl<'de> Visitor<'de> for CategoryVisitor {
            type Value = PackageCategory;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a package category string")
            }

            fn visit_str<E>(self, value: &str) -> Result<PackageCategory, E>
            where
                E: de::Error,
            {
                match value.to_lowercase().as_str() {
                    "samsung" => Ok(PackageCategory::Samsung),
                    "google" => Ok(PackageCategory::Google),
                    "xiaomi" => Ok(PackageCategory::Xiaomi),
                    "huawei" => Ok(PackageCategory::Huawei),
                    "honor" => Ok(PackageCategory::Honor),
                    "oppo" => Ok(PackageCategory::Oppo),
                    "vivo" => Ok(PackageCategory::Vivo),
                    "oneplus" => Ok(PackageCategory::Oneplus),
                    "realme" => Ok(PackageCategory::Realme),
                    "motorola" => Ok(PackageCategory::Motorola),
                    "asus" => Ok(PackageCategory::Asus),
                    "sony" => Ok(PackageCategory::Sony),
                    "lg" => Ok(PackageCategory::Lg),
                    "nokia" => Ok(PackageCategory::Nokia),
                    "nothing" => Ok(PackageCategory::Nothing),
                    "meizu" => Ok(PackageCategory::Meizu),
                    "infinix" => Ok(PackageCategory::Infinix),
                    "tecno" => Ok(PackageCategory::Tecno),
                    "itel" => Ok(PackageCategory::Itel),
                    "amazon" => Ok(PackageCategory::Amazon),
                    "meta" => Ok(PackageCategory::Meta),
                    "microsoft" => Ok(PackageCategory::Microsoft),
                    "oem" | "generic" => Ok(PackageCategory::Generic),
                    "aosp" => Ok(PackageCategory::Aosp),
                    "system" => Ok(PackageCategory::System),
                    "chipset" => Ok(PackageCategory::Chipset),
                    "carrier" => Ok(PackageCategory::Carrier),
                    "ads" => Ok(PackageCategory::Ads),
                    "social" => Ok(PackageCategory::Social),
                    "productivity" => Ok(PackageCategory::Productivity),
                    "entertainment" => Ok(PackageCategory::Entertainment),
                    "security" => Ok(PackageCategory::Security),
                    "finance" => Ok(PackageCategory::Finance),
                    "health" => Ok(PackageCategory::Health),
                    "gaming" => Ok(PackageCategory::Gaming),
                    "shopping" => Ok(PackageCategory::Shopping),
                    "news" => Ok(PackageCategory::News),
                    "education" => Ok(PackageCategory::Education),
                    "misc" => Ok(PackageCategory::Misc),
                    "user_installed" => Ok(PackageCategory::UserInstalled),
                    "other" => Ok(PackageCategory::Other),
                    _ => Ok(PackageCategory::Other), // Fallback to Other instead of error for robustness?
                    // Or keep error to find issues? User explicitly complained about "Other" for everything.
                    // If I fallback to Other, existing Unknown logic handles it.
                    // But the Error 'unknown variant' prevents DB load ENTIRELY.
                    // So fallback to Other is SAFER to ensure DB loads at least.
                }
            }
        }

        deserializer.deserialize_str(CategoryVisitor)
    }
}

impl PackageCategory {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            // OEMs
            Self::Samsung => "Samsung",
            Self::Google => "Google",
            Self::Xiaomi => "Xiaomi",
            Self::Huawei => "Huawei",
            Self::Honor => "Honor",
            Self::Oppo => "OPPO",
            Self::Vivo => "Vivo",
            Self::Oneplus => "OnePlus",
            Self::Realme => "Realme",
            Self::Motorola => "Motorola",
            Self::Asus => "ASUS",
            Self::Sony => "Sony",
            Self::Lg => "LG",
            Self::Nokia => "Nokia",
            Self::Nothing => "Nothing",
            Self::Meizu => "Meizu",
            Self::Infinix => "Infinix",
            Self::Tecno => "Tecno",
            Self::Itel => "Itel",
            Self::Amazon => "Amazon",
            Self::Meta => "Meta",
            Self::Microsoft => "Microsoft",
            // Functional
            Self::Generic => "OEM",
            Self::Aosp => "AOSP",
            Self::System => "System",
            Self::Chipset => "Chipset",
            Self::Carrier => "Carrier",
            Self::Ads => "Ads",
            Self::Social => "Social",
            Self::Productivity => "Productivity",
            Self::Entertainment => "Entertainment",
            Self::Security => "Security",
            Self::Finance => "Finance",
            Self::Health => "Health",
            Self::Gaming => "Gaming",
            Self::Shopping => "Shopping",
            Self::News => "News",
            Self::Education => "Education",
            Self::Misc => "Misc",
            Self::UserInstalled => "User Installed",
            Self::Other => "Other",
        }
    }
}
