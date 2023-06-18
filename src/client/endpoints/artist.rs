use crate::client::endpoints::TargetAPI;
use crate::client::response::{Paginated, Wrapped};
use crate::client::{EndpointArtist, Error, RequestParameters};
use crate::mc::artist::Artist;
use std::collections::HashMap;
use std::fmt::Display;

impl<ClientAuthState> EndpointArtist<'_, ClientAuthState> {
    /// Get all artists.
    ///
    /// Use the optional parameters to alter the pagination or search term.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let all_artists_res = mc.artist().get_all(None);
    ///
    /// if let Ok(all_artists) = all_artists_res {
    ///   println!("There are {} Monstercat artists.", all_artists.total);
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/artists>
    pub fn get_all(
        &self,
        parameters: Option<RequestParameters>,
    ) -> Result<Paginated<Artist>, Error> {
        self.client
            .get::<Wrapped<Paginated<Artist>>>(TargetAPI::Player, "/artists", parameters)?
            .remove("Artists")
            .ok_or(Error::NotFound("all artists"))
    }

    /// Get artist by name uri, which is a slight variation on the name depending on the characters involved.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let ace_aura_res = mc.artist().get_by_name_uri("ace-aura");
    ///
    /// if let Ok(ace_aura) = ace_aura_res {
    ///   println!("Found {}.", ace_aura.name);
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/artist/ace-aura>
    pub fn get_by_name_uri(
        &self,
        artist_name_uri: impl AsRef<str> + Display,
    ) -> Result<Artist, Error> {
        self.client.get::<Artist>(
            TargetAPI::Player,
            &format!("/artist/{artist_name_uri}"),
            None::<HashMap<String, String>>,
        )
    }

    /// Get latest artists.
    ///
    /// Use the optional parameters to alter the pagination or search term.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let latest_artists_res = mc.artist().get_latest(None);
    ///
    /// if let Ok(artists_resp) = latest_artists_res {
    ///   if let Some(latest_artists) = artists_resp.data {
    ///     if let Some(latest_artist) = latest_artists.get(0) {
    ///       println!("Welcome {}!", latest_artist.name);
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/latest-artists>
    pub fn get_latest(
        &self,
        parameters: Option<RequestParameters>,
    ) -> Result<Paginated<Artist>, Error> {
        self.client
            .get::<Wrapped<Paginated<Artist>>>(TargetAPI::Player, "/latest-artists", parameters)?
            .remove("LatestArtists")
            .ok_or(Error::NotFound("latest artists"))
    }

    /// Get artist's profile photo.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let mut reader = mc.artist().get_photo("lanidaye").expect("Could not get photo.");
    ///
    /// let _dir = std::fs::create_dir_all("downloads").unwrap();
    /// let mut file_out = std::fs::File::create("downloads/lanidaye.jpeg").expect("Could not create file.");
    /// std::io::copy(&mut reader, &mut file_out).expect("Could not save photo.");
    /// ```
    pub fn get_photo(
        &self,
        artist_name_uri: impl AsRef<str> + Display,
    ) -> Result<Box<dyn std::io::Read + Send + Sync>, Error> {
        self.client.get_reader(
            TargetAPI::WWW,
            format!("artist/{artist_name_uri}/photo"),
            None::<HashMap<String, String>>,
        )
    }
}
