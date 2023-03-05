use crate::mc::release::{ReleaseID, TrackID};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::ops::Deref;
use uuid::Uuid;

/// NewType for playlist identifier, wraps a UUID and adds type safety.
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct PlaylistID(pub Uuid);

impl Deref for PlaylistID {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for PlaylistID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A saved playlist.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Playlist {
    pub archived: bool,
    pub background_file_id: Option<Uuid>,
    pub created_at: String,
    pub description: String,
    pub id: PlaylistID,
    pub is_public: bool,
    pub items: Option<Vec<PlaylistItem>>,
    pub my_library: bool,
    pub num_records: usize,
    pub tile_file_id: Option<Uuid>,
    pub title: String,
    pub updated_at: String,
    pub user_id: Option<Uuid>,
}

/// Track present in a playlist.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaylistItem {
    /// ID of playlist this track is found in.
    pub playlist_id: PlaylistID,
    /// ID of release this track is from.
    pub release_id: ReleaseID,
    /// This track's index within the playlist.
    pub sort: usize,
    /// ID of this track.
    pub track_id: TrackID,
}
