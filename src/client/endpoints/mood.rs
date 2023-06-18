use crate::client::endpoints::TargetAPI;
use crate::client::response::{Paginated, Wrapped};
use crate::client::{EndpointMood, Error, RequestParameters};
use crate::mc::mood::Mood;
use std::collections::HashMap;
use std::fmt::Display;

impl<ClientAuthState> EndpointMood<'_, ClientAuthState> {
    /// Get all artists.
    ///
    /// Use the optional parameters to alter the pagination or search term.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let all_moods_res = mc.mood().get_all(None);
    ///
    /// if let Ok(moods_resp) = all_moods_res {
    ///   if let Some(moods) = moods_resp.data {
    ///     println!("Found all moods:");
    ///     for mood in &moods {
    ///       println!("  {} ({})", mood.name, mood.id)
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/moods>
    pub fn get_all(&self, parameters: Option<RequestParameters>) -> Result<Paginated<Mood>, Error> {
        self.client
            .get::<Wrapped<Paginated<Mood>>>(TargetAPI::Player, "/moods", parameters)?
            .remove("Moods")
            .ok_or(Error::NotFound("all moods"))
    }

    /// Get mood by name uri, which is a slight variation on the name depending on the characters involved.
    ///
    /// Example
    /// ```rust
    /// use bombay::client::Client;
    ///
    /// let mc = Client::default(); // Without authentication.
    /// let chill_res = mc.mood().get_by_name_uri("chill");
    ///
    /// if let Ok(chill) = chill_res {
    ///   println!("Found mood {}.", chill.name);
    /// }
    /// ```
    ///
    /// Example URL: <https://player.monstercat.app/api/mood/chill>
    pub fn get_by_name_uri(&self, mood_name_uri: impl AsRef<str> + Display) -> Result<Mood, Error> {
        self.client
            .get::<Wrapped<Mood>>(
                TargetAPI::Player,
                &format!("/mood/{mood_name_uri}"),
                None::<HashMap<String, String>>,
            )?
            .remove("Mood")
            .ok_or(Error::NotFound("mood"))
    }
}
