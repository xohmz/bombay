use crate::mc::util::Codec;
use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use std::{fmt::Display, ops::Deref};
use uuid::Uuid;

/// NewType for user identifier, wraps a UUID and adds type safety.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct UserID(pub Uuid);

impl Deref for UserID {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for UserID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Type for user settings and information.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub archived: Option<bool>,
    pub auto_say_song: bool,
    pub attributes: Attributes,
    pub birthday: Option<String>,
    pub city: Option<String>,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub created_at: String,
    pub email: String,
    pub email_verification_status: Option<String>,
    pub features: Option<Vec<Value>>,
    pub first_name: String,
    pub free_gold: bool,
    pub free_gold_at: Option<String>,
    pub free_gold_reason: String,
    pub given_download_access: bool,
    pub google_maps_place_id: String,
    pub has_download: bool,
    pub has_gold: bool,
    pub has_password: bool,
    pub id: UserID,
    pub last_name: Option<String>,
    pub last_seen: Option<Timestamp>,
    pub last_update_benefits_gold: Option<bool>,
    pub location_lat: f64,
    pub location_lng: f64,
    pub max_licenses: u64,
    pub my_library: String,
    pub place_name: String,
    pub place_name_full: String,
    #[serde(alias = "PlayerUUID")]
    pub player_uuid: String,
    pub pronouns: Option<String>,
    pub prov_st: Option<String>,
    pub province_state: Option<String>,
    pub say_song: bool,
    pub score: Option<Value>,
    pub settings: Settings,
    pub two_factor_id: Option<String>,
    pub two_factor_pending_id: Option<String>,
    pub updated_at: Timestamp,
    pub username: String,
}

/// User information that can be set using an API POST.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EditableUserInfo {
    pub birthday: Option<Timestamp>,
    pub google_maps_place_id: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub pronouns: Option<String>,
}

/// User attributes, most indicate notification email preferences.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub events: bool,
    pub gold_perks: bool,
    pub merch: Option<bool>,
    pub news: Option<bool>,
    pub relics: Option<bool>,
}

/// User settings.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    pub auto_enable_streamer_mode: Option<bool>,
    pub block_unlicensable_tracks: Option<bool>,
    pub hide_unlicensable_tracks: Option<bool>,
    pub streamer_mode: Option<bool>,
    pub playlist_public_default: bool,
    pub preferred_format: String,
    pub say_song: Option<bool>,
    pub auto_say_song: Option<bool>,
}

/// User settings that can be changed using an API POST.
///
/// auto_say_song requires say_song. Both require a connected Twitch account.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EditableSettings {
    pub playlist_public_default: Option<bool>,
    pub preferred_format: Option<Codec>,
    pub say_song: Option<bool>,
    pub auto_say_song: Option<bool>,
}

/// Simple type to capture the streaming width (player code) response.
///
/// These sorts of simple wrappers are made to maintain the call patterns
/// and to leave room for future expansion, such as additional fields or
/// letter case changes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PlayerCode {
    pub player_code: String,
}

/// NewType for shop code identifier, wraps a UUID and adds type safety.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct ShopCodeID(pub Uuid);

impl Deref for ShopCodeID {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ShopCodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Shop code discount object.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShopCode {
    pub id: ShopCodeID,
    pub code: String,
    pub create_date: Timestamp,
    pub expire_date: Timestamp,
    pub reward_description: String,
    pub updated_at: Timestamp,
    pub user_id: UserID,
    pub value: String,
    pub value_type: String,
}

/// Simple type to capture the new email request.
///
/// These sorts of simple wrappers are made to maintain the call patterns
/// and to leave room for future expansion, such as additional fields or
/// letter case changes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct NewEmail {
    pub new_email: String,
}

/// Simple type to capture the new password request.
///
/// These sorts of simple wrappers are made to maintain the call patterns
/// and to leave room for future expansion, such as additional fields or
/// letter case changes.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct NewPassword {
    pub old_password: String,
    pub new_password: String,
}

/// Variants of platforms.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationInterests {
    News,
    Events,
    Merch,
    GoldPerks,
    Relics,
}
