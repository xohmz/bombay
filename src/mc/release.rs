use crate::mc::artist::AnyArtist;
use crate::mc::label::Brand;
use crate::mc::util::{CacheDetails, Link};
use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;
use uuid::Uuid;

/// NewType for release identifier, wraps a UUID and adds type safety.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct ReleaseID(pub Uuid);

impl Deref for ReleaseID {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ReleaseID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// NewType for track identifier, wraps a UUID and adds type safety.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct TrackID(pub Uuid);

impl Deref for TrackID {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for TrackID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// NewType for release catalog identifier, wraps a UUID and adds type safety.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct CatalogID(pub String);

impl Deref for CatalogID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for CatalogID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Enumerated type to capture the possible release types.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnyRelease {
    Release(Release),
    Track(Track),
}

impl AnyRelease {
    /// Get any release's type.
    pub fn get_type(&self) -> &str {
        match self {
            AnyRelease::Release(a) => &a.kind,
            AnyRelease::Track(_) => "Track",
        }
    }

    /// Get any release's title.
    pub fn get_title(&self) -> &str {
        match self {
            AnyRelease::Release(a) => &a.title,
            AnyRelease::Track(t) => &t.title,
        }
    }

    /// Get any release's artists.
    pub fn get_artists(&self) -> &str {
        match self {
            AnyRelease::Release(a) => &a.artists_title,
            AnyRelease::Track(t) => &t.artists_title,
        }
    }

    /// Get any release's release date.
    pub fn get_date(&self) -> &Timestamp {
        match self {
            AnyRelease::Release(a) => &a.release_date,
            AnyRelease::Track(t) => &t.release.release_date,
        }
    }

    /// Get any release's release identifier.
    pub fn get_release_id(&self) -> &ReleaseID {
        match self {
            AnyRelease::Release(release) => &release.id,
            AnyRelease::Track(track) => &track.release.id,
        }
    }
}

/// Most detailed release object returned by the MC API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Release {
    pub album_notes: Option<String>,
    pub artists: Option<Vec<AnyArtist>>,
    pub artists_title: String,
    pub brand_id: Option<Brand>,
    pub brand_title: Option<String>,
    #[serde(flatten)]
    pub cache_details: Option<CacheDetails>,
    pub catalog_id: CatalogID,
    pub copyright_p_line: Option<String>,
    pub cover_file_id: Option<String>,
    pub description: String,
    pub downloadable: Option<bool>,
    pub featured_artists_title: String,
    #[serde(alias = "GRid")]
    pub grid: Option<String>,
    pub genre_primary: Option<String>,
    pub genre_secondary: Option<String>,
    pub id: ReleaseID,
    pub in_early_access: Option<bool>,
    pub links: Option<Vec<Link>>,
    pub prerelease_date: Option<Timestamp>,
    pub presave_date: Option<Timestamp>,
    pub release_date: Timestamp,
    pub release_date_timezone: String,
    pub spotify_id: Option<String>,
    pub streamable: Option<bool>,
    pub tags: Option<Vec<String>>,
    pub title: String,
    pub tracks: Option<Vec<Track>>,
    #[serde(alias = "Type")]
    pub kind: String,
    #[serde(alias = "UPC")]
    pub upc: Option<String>,
    pub version: String,
    #[serde(alias = "YouTubeUrl")]
    pub youtube_url: Option<String>,
}

/// Summarized release details.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReleaseSummary {
    pub artists_title: String,
    pub catalog_id: String,
    pub copyright_p_line: Option<String>,
    pub description: String,
    pub id: ReleaseID,
    pub release_date: Timestamp,
    pub release_date_timezone: String,
    pub tags: Option<Vec<String>>,
    pub title: String,
    #[serde(alias = "Type")]
    pub kind: String,
    #[serde(alias = "UPC")]
    pub upc: Option<String>,
    pub version: String,
}

/// Detailed release track information.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Track {
    pub artists: Option<Vec<AnyArtist>>,
    pub artists_title: String,
    #[serde(alias = "BPM")]
    pub bpm: usize,
    pub brand: String,
    pub brand_id: usize,
    pub creator_friendly: bool,
    pub debut_date: Option<Timestamp>,
    pub downloadable: bool,
    pub duration: usize,
    pub explicit: bool,
    pub genre_primary: String,
    pub genre_secondary: String,
    #[serde(alias = "ISRC")]
    pub isrc: String,
    pub id: TrackID,
    pub in_early_access: bool,
    pub lock_status: String,
    pub public: bool,
    pub playlist_sort: Option<u32>,
    pub release: ReleaseSummary,
    pub streamable: bool,
    pub tags: Option<Vec<String>>,
    pub title: String,
    pub track_number: usize,
    pub version: String,
}
