use crate::mc::release::ReleaseID;
use crate::mc::util::{CacheDetails, Link};
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Enumerated type to capture the possible artist types.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AnyArtist {
    Artist(Box<Artist>),
    AlbumArtist(Box<AlbumArtist>),
    ReleaseArtist(Box<ReleaseArtist>),
}

/// Most detailed artist object returned by the MC API.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Artist {
    pub about: Option<String>,
    pub active_years: Option<Vec<u16>>,
    #[serde(flatten)]
    pub cache_details: Option<CacheDetails>,
    pub details: ArtistDetails,
    pub featured_release_cover_file_id: Option<String>,
    pub featured_release_id: Option<String>,
    pub featured_video_url: Option<String>,
    pub id: Uuid,
    pub landscape_file_id: Option<String>,
    pub links: Option<Vec<Link>>,
    pub logo_file_id: Option<String>,
    pub name: String,
    pub portrait_file_id: Option<String>,
    pub profile_file_id: Option<Uuid>,
    pub public: bool,
    pub show_event: bool,
    pub square_file_id: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(alias = "URI")]
    pub uri: String,
}

/// Additional details regarding this artist.
///
/// Thanks to this
/// [thread](https://users.rust-lang.org/t/how-can-i-handle-duplicate-fields-when-specifying-multiple-aliases-using-serde/46426/7)
/// for a clever solution to conflicting key names after capitalization normalization.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtistDetails {
    #[serde(deserialize_with = "helper_artist_details_about", flatten)]
    pub about: Option<String>,
    #[serde(deserialize_with = "helper_artist_details_bookings", flatten)]
    pub bookings: Option<String>,
    #[serde(deserialize_with = "helper_artist_details_management", flatten)]
    pub management: Option<String>,
    #[serde(deserialize_with = "helper_artist_details_management_details", flatten)]
    pub management_details: Option<String>,
    #[serde(deserialize_with = "helper_artist_details_show_events", flatten)]
    pub show_events: Option<String>,
}

/// Facilitate extraction of inconsistently capitalized field.
#[derive(Deserialize)]
struct HelperArtistDetailsAbout {
    #[serde(rename = "About")]
    opt_0: Option<String>,
    #[serde(rename = "about")]
    opt_1: Option<String>,
}

/// Extract inconsistently capitalized field.
fn helper_artist_details_about<'d, D: Deserializer<'d>>(d: D) -> Result<Option<String>, D::Error> {
    let HelperArtistDetailsAbout { opt_0, opt_1 } = HelperArtistDetailsAbout::deserialize(d)?;
    Ok(opt_0.or(opt_1))
}

/// Facilitate extraction of inconsistently capitalized field.
#[derive(Deserialize)]
struct HelperArtistDetailsBookings {
    #[serde(rename = "Bookings")]
    opt_0: Option<String>,
    #[serde(rename = "bookings")]
    opt_1: Option<String>,
}

/// Extract inconsistently capitalized field.
fn helper_artist_details_bookings<'d, D: Deserializer<'d>>(
    d: D,
) -> Result<Option<String>, D::Error> {
    let HelperArtistDetailsBookings { opt_0, opt_1 } = HelperArtistDetailsBookings::deserialize(d)?;
    Ok(opt_0.or(opt_1))
}

/// Facilitate extraction of inconsistently capitalized field.
#[derive(Deserialize)]
struct HelperArtistDetailsManagement {
    #[serde(rename = "Management")]
    opt_0: Option<String>,
    #[serde(rename = "management")]
    opt_1: Option<String>,
}

/// Extract inconsistently capitalized field.
fn helper_artist_details_management<'d, D: Deserializer<'d>>(
    d: D,
) -> Result<Option<String>, D::Error> {
    let HelperArtistDetailsManagement { opt_0, opt_1 } =
        HelperArtistDetailsManagement::deserialize(d)?;
    Ok(opt_0.or(opt_1))
}

/// Facilitate extraction of inconsistently capitalized field.
#[derive(Deserialize)]
struct HelperArtistDetailsManagementDetails {
    #[serde(rename = "ManagementDetails")]
    opt_0: Option<String>,
    #[serde(rename = "managementDetails")]
    opt_1: Option<String>,
}

/// Extract inconsistently capitalized field.
fn helper_artist_details_management_details<'d, D: Deserializer<'d>>(
    d: D,
) -> Result<Option<String>, D::Error> {
    let HelperArtistDetailsManagementDetails { opt_0, opt_1 } =
        HelperArtistDetailsManagementDetails::deserialize(d)?;
    Ok(opt_0.or(opt_1))
}

/// Facilitate extraction of inconsistently capitalized field.
#[derive(Deserialize)]
struct HelperArtistDetailsShowEvents {
    #[serde(rename = "Management")]
    opt_0: Option<String>,
    #[serde(rename = "management")]
    opt_1: Option<String>,
}

/// Extract inconsistently capitalized field.
fn helper_artist_details_show_events<'d, D: Deserializer<'d>>(
    d: D,
) -> Result<Option<String>, D::Error> {
    let HelperArtistDetailsShowEvents { opt_0, opt_1 } =
        HelperArtistDetailsShowEvents::deserialize(d)?;
    Ok(opt_0.or(opt_1))
}

/// Artist object related to an album.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AlbumArtist {
    pub artist_id: Uuid,
    pub artist_number: usize,
    pub name: String,
    pub profile_file_id: Uuid,
    pub platform: String,
    pub public: bool,
    pub release_id: ReleaseID,
    pub role: String,
    pub square_file_id: Option<String>,
    #[serde(alias = "URI")]
    pub uri: String,
}

/// Artist object related to a release.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReleaseArtist {
    pub catalog_record_id: String,
    pub id: Uuid,
    pub name: String,
    pub profile_file_id: Uuid,
    pub public: bool,
    pub role: String,
    #[serde(alias = "URI")]
    pub uri: String,
}
