use crate::mc::user::UserID;
use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::ops::Deref;
use std::{fmt::Display, str::FromStr};
use url::Url;
use uuid::Uuid;

/// Values related to some sort of cache.
/// These are found (flattened) in various objects.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CacheDetails {
    pub cache_time: Timestamp,
    pub cache_status: String,
    pub cache_status_detail: String,
}

/// Supported audio codecs for downloading songs.
#[derive(Clone, Debug, Default, PartialEq, DeserializeFromStr, SerializeDisplay)]
pub enum Codec {
    #[default]
    MP3,
    FLAC,
    WAV,
}

impl Display for Codec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Codec::MP3 => "mp3_320",
                Codec::FLAC => "flac",
                Codec::WAV => "wav",
            }
        )
    }
}

impl FromStr for Codec {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut norm = s.to_lowercase();
        norm.retain(|c| !c.is_whitespace());
        Ok(match norm.as_str() {
            "mp3_320" => Codec::MP3,
            "flac" => Codec::FLAC,
            "wav" => Codec::WAV,
            _ => Codec::MP3,
        })
    }
}

/// Represents a link to a particular platform resource.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Link {
    /// Name of the platform.
    pub platform: Platform,
    /// HTTP(S) URL.
    pub url: Url,
}

/// Variants of platforms.
#[derive(Clone, Debug, PartialEq, DeserializeFromStr, SerializeDisplay)]
#[serde_with()]
pub enum Platform {
    Amazon,
    AppleMusic,
    Audiomack,
    Audius,
    Bandcamp,
    Deezer,
    Facebook,
    GooglePlay,
    Instagram,
    Other(String),
    Patreon,
    SoundCloud,
    Spotify,
    Tidal,
    TikTok,
    Twitch,
    Twitter,
    Website,
    YouTube,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Platform::Amazon => "Amazon",
                Platform::AppleMusic => "Apple Music",
                Platform::Audiomack => "Audiomack",
                Platform::Audius => "Audius",
                Platform::Bandcamp => "Bandcamp",
                Platform::Deezer => "Deezer",
                Platform::Facebook => "Facebook",
                Platform::GooglePlay => "Google Play",
                Platform::Instagram => "Instagram",
                Platform::Other(unk) => unk,
                Platform::Patreon => "Patreon",
                Platform::SoundCloud => "SoundCloud",
                Platform::Spotify => "Spotify",
                Platform::Tidal => "Tidal",
                Platform::TikTok => "TikTok",
                Platform::Twitch => "Twitch",
                Platform::Twitter => "Twitter",
                Platform::Website => "Website",
                Platform::YouTube => "YouTube",
            }
        )
    }
}

impl FromStr for Platform {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut norm = s.to_lowercase();
        norm.retain(|c| !c.is_whitespace());
        Ok(match norm.as_str() {
            "amazon" => Platform::Amazon,
            "applemusic" => Platform::AppleMusic,
            "audiomack" => Platform::Audiomack,
            "audius" => Platform::Audius,
            "bandcamp" => Platform::Bandcamp,
            "deezer" => Platform::Deezer,
            "facebook" => Platform::Facebook,
            "googleplay" => Platform::GooglePlay,
            "ig" => Platform::Instagram,
            "instagram" => Platform::Instagram,
            "patreon" => Platform::Patreon,
            "soundcloud" => Platform::SoundCloud,
            "spotify" => Platform::Spotify,
            "tidal" => Platform::Tidal,
            "tiktok" => Platform::TikTok,
            "twitch" => Platform::Twitch,
            "twitter" => Platform::Twitter,
            "website" => Platform::Website,
            "youtube" => Platform::YouTube,
            _ => Platform::Other(norm),
        })
    }
}

/// NewType for license identifier, wraps a UUID and adds type safety.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct LicenseID(pub Uuid);

impl Deref for LicenseID {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for LicenseID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// License allowing user/creator to use MC songs in public, published content.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct License {
    active_times: Vec<LicenseActiveTimes>,
    allow_listed: Option<Value>,
    archived: bool,
    created_at: Timestamp,
    free: bool,
    free_at: Option<Timestamp>,
    free_reason: String,
    has_active_period: bool,
    id: LicenseID,
    identity: String,
    invalid: bool,
    last_sync: Timestamp,
    notes: Option<String>,
    #[serde(rename = "OAuthId")]
    oauth_id: Uuid,
    sanitized: Option<bool>,
    scheduled_sync: Option<Timestamp>,
    state: String,
    sync_failures: Option<u32>,
    sync_state: Option<String>,
    updated_at: Timestamp,
    user_archived: bool,
    user_email: String,
    user_id: UserID,
    vendor: Platform,
    whitelisted: Option<bool>,
    #[serde(rename = "YouTubeStatsDate")]
    youtube_stats_date: Option<Timestamp>,
    #[serde(rename = "YouTubeSubscribers")]
    youtube_subscribers: usize,
    #[serde(rename = "YouTubeTitle")]
    youtube_title: String,
    #[serde(rename = "YouTubeUrl")]
    youtube_url: String,
    #[serde(rename = "YouTubeViews")]
    youtube_views: usize,
}

/// Times during which license is active.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LicenseActiveTimes {
    created_at: Timestamp,
    finish: Timestamp,
    gold_time_range_id: Uuid,
    id: Uuid,
    license_id: Uuid,
    source: String,
    start: Timestamp,
}

/// Simple wrapper for call to remove copyright claim on a video.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClaimVideoId {
    pub video_id: String,
}
