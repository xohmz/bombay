use std::collections::HashMap;

use crate::client::endpoints::TargetAPI;
use crate::client::{
    EndpointPlaylist, Error, PlaylistItemMod, PlaylistItemOperations, PlaylistItemsMod,
    PlaylistItemsOperations, Wrapped,
};
use crate::client::{Paginated, SignedIn};
use crate::mc::playlist::{Playlist, PlaylistID};
use crate::mc::release::AnyRelease;
use uuid::uuid;

const TOP_30: PlaylistID = PlaylistID(uuid!("991334fb-ca5e-48c6-bc73-cb83c364357d"));

impl<ClientAuthState> EndpointPlaylist<'_, ClientAuthState> {
    /// Get the public playlist of top 30 tracks.
    pub fn get_top_30_playlist_id(&self) -> PlaylistID {
        TOP_30
    }

    /// Get a playlist by id.
    pub fn by_id(&self, id: PlaylistID) -> Result<Playlist, Error> {
        self.client
            .get::<Wrapped<Playlist>>(
                TargetAPI::Player,
                &format!("/playlist/{id}"),
                None::<HashMap<String, String>>,
            )?
            .remove("Playlist")
            .ok_or(Error::NotFound("latest artists"))
    }

    /// Get the tracks of a playlist.
    pub fn get_tracks_by_playlist_id(
        &self,
        id: PlaylistID,
    ) -> Result<Paginated<AnyRelease>, Error> {
        self.client.get::<Paginated<AnyRelease>>(
            TargetAPI::Player,
            &format!("/playlist/{id}/catalog"),
            None::<HashMap<String, String>>,
        )
    }

    /// Get playlist tile image.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    /// use bombay::mc::playlist::PlaylistID;
    /// use uuid::uuid;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let mut reader = mc.playlist().get_tile_image(
    ///     PlaylistID(uuid!("991334fb-ca5e-48c6-bc73-cb83c364357d"))
    /// ).expect("Could not get tile.");
    ///
    /// let _dir = std::fs::create_dir_all("downloads").unwrap();
    /// let mut file_out = std::fs::File::create("downloads/top_30_tile.png").expect("Could not create file.");
    /// std::io::copy(&mut reader, &mut file_out).expect("Could not save tile.");
    /// ```
    pub fn get_tile_image(
        &self,
        playlist_id: PlaylistID,
    ) -> Result<Box<dyn std::io::Read + Send + Sync>, Error> {
        self.client.get_reader(
            TargetAPI::Player,
            format!("/playlist/{playlist_id}/tile"),
            None::<HashMap<String, String>>,
        )
    }

    /// Get playlist background image.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    /// use bombay::mc::playlist::PlaylistID;
    /// use uuid::uuid;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let mut reader = mc.playlist().get_background_image(
    ///     PlaylistID(uuid!("991334fb-ca5e-48c6-bc73-cb83c364357d"))
    /// ).expect("Could not get background.");
    ///
    /// let _dir = std::fs::create_dir_all("downloads").unwrap();
    /// let mut file_out = std::fs::File::create("downloads/top_30_background.png").expect("Could not create file.");
    /// std::io::copy(&mut reader, &mut file_out).expect("Could not save background.");
    /// ```
    pub fn get_background_image(
        &self,
        playlist_id: PlaylistID,
    ) -> Result<Box<dyn std::io::Read + Send + Sync>, Error> {
        self.client.get_reader(
            TargetAPI::Player,
            format!("/playlist/{playlist_id}/background"),
            None::<HashMap<String, String>>,
        )
    }
}

impl EndpointPlaylist<'_, SignedIn> {
    /// Get all of the user's playlist.
    pub fn get_all(&self) -> Result<Paginated<Playlist>, Error> {
        self.client
            .get::<Wrapped<Paginated<Playlist>>>(
                TargetAPI::Player,
                "/playlists",
                None::<HashMap<String, String>>,
            )?
            .remove("Playlists")
            .ok_or(Error::NotFound("Playlists not found."))
    }

    /// Create a playlist.
    pub fn create(&self, playlist: Playlist) -> Result<PlaylistID, Error> {
        self.client
            .post::<Wrapped<PlaylistID>>(
                TargetAPI::Player,
                "/playlist",
                None::<HashMap<String, String>>,
                Some(playlist),
            )?
            .remove("Id")
            .ok_or(Error::NotFound("Playlist not found."))
    }

    /// Edit a playlist.
    pub fn edit(&self, playlist: Playlist) -> Result<Playlist, Error> {
        self.client.post::<Playlist>(
            TargetAPI::Player,
            &format!("/playlist/{}", &playlist.id),
            None::<HashMap<String, String>>,
            Some(playlist),
        )
    }

    /// Modify a single playlist item.
    pub fn modify_item(
        &self,
        playlist_id: PlaylistID,
        operation: PlaylistItemOperations,
        item_mod: PlaylistItemMod,
    ) -> Result<(), Error> {
        if operation == PlaylistItemOperations::To && item_mod.move_to.is_none() {
            Err(Error::Message(
                "Playlist item move operation requires a move_to index.",
            ))
        } else {
            self.client.post_empty_response(
                TargetAPI::Player,
                format!("/playlist/{playlist_id}/modify-item"),
                Some(operation),
                Some(item_mod),
            )
        }
    }

    /// Modify multiple playlist items.
    pub fn modify_items(
        &self,
        playlist_id: PlaylistID,
        operation: PlaylistItemsOperations,
        items_mod: PlaylistItemsMod,
    ) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            format!("/playlist/{playlist_id}/modify-items"),
            Some(operation),
            Some(items_mod),
        )
    }

    /// Delete playlist.
    pub fn delete(&self, playlist_id: PlaylistID) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            format!("/playlist/{playlist_id}/delete"),
            None::<HashMap<String, String>>,
            None::<()>,
        )
    }
}
