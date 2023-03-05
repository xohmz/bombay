use crate::client::endpoints::TargetAPI;
use crate::client::request::RequestParameters;
use crate::client::{EndpointRelease, Error, Paginated, SignedIn, Wrapped};
use crate::mc::release::{AnyRelease, CatalogID, ReleaseID, Track, TrackID};
use crate::mc::util::Codec;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Display;

impl<ClientAuthState> EndpointRelease<'_, ClientAuthState> {
    /// Get all releases.
    ///
    /// Use the optional parameters to alter the pagination or search term.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let all_releases_res = mc.release().get_all(None);
    ///
    /// if let Ok(all_releases) = all_releases_res {
    ///   println!("There are {} Monstercat releases.", all_releases.total);
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/releases>
    pub fn get_all(
        &self,
        parameters: Option<RequestParameters>,
    ) -> Result<Paginated<AnyRelease>, Error> {
        self.client
            .get::<Wrapped<Paginated<AnyRelease>>>(TargetAPI::Player, "/releases", parameters)?
            .remove("Releases")
            .ok_or(Error::NotFound("all releases"))
    }

    /// Get latest releases.
    ///
    /// Use the optional parameters to alter the pagination or search term.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let latest_releases_res = mc.release().get_latest(None);
    ///
    /// if let Ok(latest_releases) = latest_releases_res {
    ///   if let Some(releases) = latest_releases.data {
    ///     if let Some(latest_release) = releases.get(0) {
    ///       println!("Welcome {}!", latest_release.get_title());
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/catalog/latest-releases>
    pub fn get_latest(
        &self,
        parameters: Option<RequestParameters>,
    ) -> Result<Paginated<AnyRelease>, Error> {
        self.client.get::<Paginated<AnyRelease>>(
            TargetAPI::Player,
            "/catalog/latest-releases",
            Some(parameters.unwrap_or_default()),
        )
    }

    /// Get artist's latest releases by their name uri, which is a slight
    /// variation on the name depending on the characters involved.
    ///
    /// Use the optional parameters to alter the pagination or search term.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::{Client, PaginationParameters, RequestParameters};
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let related_releases_res = mc.release().get_by_artist_name_uri(
    ///   "rogue",
    ///   Some(RequestParameters::from_pagination(PaginationParameters { limit: 5, offset: 0 }))
    /// );
    ///
    /// if let Ok(related_releases) = related_releases_res {
    ///   if let Some(releases) = related_releases.data {
    ///     if let Some(latest_related_release) = releases.get(0){
    ///       println!(
    ///         "{} released {} on {}!",
    ///         latest_related_release.get_artists(),
    ///         latest_related_release.get_title(),
    ///         latest_related_release.get_date()
    ///       );
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/artist/rogue/releases?limit=5>
    pub fn get_by_artist_name_uri(
        &self,
        artist_name_uri: impl AsRef<str> + Display,
        parameters: Option<RequestParameters>,
    ) -> Result<Paginated<AnyRelease>, Error> {
        self.client
            .get::<Wrapped<Paginated<AnyRelease>>>(
                TargetAPI::Player,
                &format!("/artist/{artist_name_uri}/releases"),
                parameters,
            )?
            .remove("Releases")
            .ok_or(Error::NotFound("artist releases"))
    }

    /// Get a release by its catalog ID.
    ///
    /// Use the optional parameters to alter the pagination or search term.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    /// use bombay::mc::release::CatalogID;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let release_and_tracks_res = mc.release().get_by_catalog_id(&CatalogID("MCS1186".to_owned()));
    ///
    /// if let Ok(release_and_tracks) = release_and_tracks_res {
    ///   let (any_release, tracks) = release_and_tracks;
    ///   println!(
    ///     "{} released {} on {} with {} tracks!",
    ///     any_release.get_artists(),
    ///     any_release.get_title(),
    ///     any_release.get_date(),
    ///     tracks.len()
    ///   );
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/catalog/release/MCS1186>
    pub fn get_by_catalog_id(
        &self,
        catalog_id: &CatalogID,
    ) -> Result<(AnyRelease, Vec<Track>), Error> {
        let mut related_wrapper = self.client.get::<Wrapped<Value>>(
            TargetAPI::Player,
            &format!("/catalog/release/{catalog_id}?idType=catalogId"),
            None::<HashMap<String, String>>,
        )?;

        let release_val = related_wrapper
            .remove("Release")
            .ok_or(Error::NotFound("release"))?;

        let release_obj = serde_json::from_value::<AnyRelease>(release_val)
            .map_err(|err| Error::Deserialization(err))?;

        let tracks_val = related_wrapper
            .remove("Tracks")
            .ok_or(Error::NotFound("release tracks"))?;

        let tracks_obj = serde_json::from_value::<Vec<Track>>(tracks_val)
            .map_err(|err| Error::Deserialization(err))?;

        Ok((release_obj, tracks_obj))
    }

    /// Get Release cover art.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    /// use bombay::mc::release::CatalogID;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let mut reader = mc.release()
    ///     .get_cover_art(&CatalogID("742779546913".to_owned()))
    ///     .expect("Could not find release cover art.");
    ///
    /// let _dir = std::fs::create_dir_all("downloads").unwrap();
    /// let mut file_out = std::fs::File::create("downloads/feelings_cover_art.jpeg").expect("Could not create file.");
    /// std::io::copy(&mut reader, &mut file_out).expect("Could not save cover art.");
    /// ```
    pub fn get_cover_art(
        &self,
        catalog_id: &CatalogID,
    ) -> Result<Box<dyn std::io::Read + Send + Sync>, Error> {
        self.client.get_reader(
            TargetAPI::WWW,
            &format!("release/{catalog_id}/cover"),
            None::<HashMap<String, String>>,
        )
    }

    /// Get releases related to another by the release id.
    ///
    /// Use the optional parameters to alter the pagination or search term.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    /// use bombay::mc::release::ReleaseID;
    /// use uuid::uuid;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let related_releases_res = mc.release().get_related_by_id(
    ///   &ReleaseID(uuid!("6a58b6d2-bbec-4847-8dcf-45023a930968")),
    ///   None
    /// );
    ///
    /// if let Ok(related_releases) = related_releases_res {
    ///   if let Some(releases) = related_releases.data {
    ///     if let Some(latest_related_release) = releases.get(0) {
    ///       println!(
    ///         "Related to 'Chasing Shadows': {} released {} on {}!",
    ///         latest_related_release.get_artists(),
    ///         latest_related_release.get_title(),
    ///         latest_related_release.get_date()
    ///       );
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/related-releases/6a58b6d2-bbec-4847-8dcf-45023a930968>
    pub fn get_related_by_id(
        &self,
        id: &ReleaseID,
        parameters: Option<RequestParameters>,
    ) -> Result<Paginated<AnyRelease>, Error> {
        self.client.get::<Paginated<AnyRelease>>(
            TargetAPI::Player,
            &format!("/related-releases/{id}"),
            parameters,
        )
    }

    /// Stream track using release id and track id.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    /// use bombay::mc::release::{ReleaseID, TrackID};
    /// use uuid::uuid;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let reader = mc.release().stream_by_ids(
    ///   &ReleaseID(uuid!("4c6b9486-7644-4f3f-b9ae-0fa4d27a4259")),
    ///   &TrackID(uuid!("00164f5c-3a1e-44ad-8b73-bfdde22b8b6e"))
    /// );
    ///
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/release/4c6b9486-7644-4f3f-b9ae-0fa4d27a4259/track-stream/00164f5c-3a1e-44ad-8b73-bfdde22b8b6e>
    pub fn stream_by_ids(
        &self,
        release_id: &ReleaseID,
        track_id: &TrackID,
    ) -> Result<Box<dyn std::io::Read + Send + Sync>, Error> {
        self.client.get_reader(
            TargetAPI::Player,
            &format!("/release/{release_id}/track-stream/{track_id}"),
            None::<HashMap<String, String>>,
        )
    }
}

impl EndpointRelease<'_, SignedIn> {
    /// Download track using release id and track id.
    pub fn download_by_ids(
        &self,
        release_id: &ReleaseID,
        track_id: &TrackID,
        codec: Option<Codec>,
    ) -> Result<Box<dyn std::io::Read + Send + Sync>, Error> {
        self.client.get_reader(
            TargetAPI::Player,
            &format!("/release/{release_id}/track-download/{track_id}"),
            Some(RequestParameters::from_codec(codec.unwrap_or_default())),
        )
    }
}
