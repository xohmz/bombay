#![doc = include_str!("README.md")]

mod artist;
mod mood;
mod playlist;
mod release;
mod user;

use crate::client::{Client, SignedIn};

pub use artist::*;
pub use mood::*;
pub use playlist::*;
pub use release::*;
pub use user::*;

/// Type enumerating the two base endpoints for the Monstercat API.
#[derive(Default)]
pub enum TargetAPI {
    #[default]
    Player,
    WWW,
}

/// Trait for things that provide access to some part of the Monstercat API.
pub trait Endpoint: private::Sealed {}

mod private {
    use super::EndpointArtist;

    pub trait Sealed {}

    impl<ClientAuthState> Sealed for EndpointArtist<'_, ClientAuthState> {}
}

/// Endpoint to retrieve one or more artists.
pub struct EndpointArtist<'a, ClientAuthState> {
    pub client: &'a Client<ClientAuthState>,
}

/// Endpoint to retrieve one or more moods.
pub struct EndpointMood<'a, ClientAuthState> {
    pub client: &'a Client<ClientAuthState>,
}

/// Endpoint to retrieve one or more Users.
pub struct EndpointPlaylist<'a, ClientAuthState> {
    pub client: &'a Client<ClientAuthState>,
}

/// Endpoint to retrieve one or more releases.
pub struct EndpointRelease<'a, ClientAuthState> {
    pub client: &'a Client<ClientAuthState>,
}

/// Endpoint to retrieve and manage user account information.
pub struct EndpointUser<'a, ClientAuthState = SignedIn> {
    pub client: &'a Client<ClientAuthState>,
}
