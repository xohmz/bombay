#![doc = include_str!("../README.md")]

//! ## Examples
//!
//! ### Create Clients
//!
//! Without authentication:
//!
//! ```rust
//! use bombay::client::Client;
//! let mc = Client::default();
//! ```
//!
//! With authentication:
//!
//! ```rust
//! use bombay::client::{auth::SignInOutcome, Error, Client};
//! use std::thread;
//! use std::time::Duration;
//! use totp_rs::{Algorithm, Secret, TOTP};
//!
//! let mut client_unauth = Client::default();
//!
//! let outcome = client_unauth
//!     .sign_in(
//!         env!("MC_EMAIL").to_owned(),
//!         env!("MC_PASSWORD").to_owned(),
//!     )
//!     .expect("Failed to sign in");
//!
//! let client = match outcome {
//!     // User doesn't have 2FA
//!     SignInOutcome::Authenticated(new_client) => Ok(new_client),
//!     // User has 2FA with email. Every 5 seconds, check if they've confirmed,
//!     // up to 300 times.
//!     SignInOutcome::Email(email_callback) => {
//!         let mut email_authed_client = Err(Error::SignIn(
//!             "Test failed, email confirmation took too long.",
//!         ));
//!         let mut attempts = 0;
//!         while attempts < 300 {
//!             attempts += 1;
//!             thread::sleep(Duration::from_secs(5));
//!             let client_res = email_callback(&mut client_unauth);
//!             if let Ok(client) = client_res {
//!                 email_authed_client = Ok(client);
//!                 break;
//!             }
//!         }
//!
//!         email_authed_client
//!     }
//!     // User has 2FA with authenticator app. Try code from String.
//!     SignInOutcome::TOTP(totp_callback) => {
//!         let totp_secret = Secret::Encoded(env!("MC_TOTP_SECRET").to_owned());
//!         let totp_gen = TOTP::new(
//!             Algorithm::SHA1,
//!             6,
//!             1,
//!             30,
//!             totp_secret.to_bytes().unwrap(),
//!         )
//!         .unwrap();
//!         totp_callback(&mut client_unauth, totp_gen.generate_current().unwrap())
//!     }
//! }
//! .expect("Failed to sign in");
//! ```
//!
//! ### Fetch Some Details
//!
//! ```rust
//! use bombay::client::{Error, Client, PaginationParameters, RequestParameters};
//! use std::error;
//! use uuid::uuid;
//!
//! fn main() -> Result<(), Box<dyn error::Error>> {
//!     // We don't need authentication to fetch some information.
//!     let mc = Client::default();
//!
//!     // Lets search for one of my favorite artists and bail if there are errors.
//!     let search_results = mc
//!         .artist()
//!         .get_all(Some(RequestParameters::from_search("Grant".to_owned())))?;
//!
//!     // I also expect some data in the response.
//!     let artists = search_results
//!         .data
//!         .ok_or(Error::Message("Oh no! Where did Grant go?!"))?;
//!
//!     // And not empty data!.
//!     let grant = artists
//!         .get(0)
//!         .ok_or(Error::Message("Oh no! Where did Grant go?!"))?;
//!
//!     // Lets make sure it's him.
//!     if grant.id != uuid!("27063fd3-4fba-4119-9af0-5001e925b0d2") {
//!         return Err(Box::new(Error::Message(
//!             "We found someone, but not Grant! Hmmm...",
//!         )));
//!     }
//!
//!     // Alright lets learn about Grant!
//!    let about_grant = grant.about.as_ref();
//!    let alt_about = r#"
//!     Grant's music makes any moment better.
//!     You'll find out how many emotions you can feel at once!
//!     "#
//!     .to_owned();
//!     println!("{}:\n{}\n", &grant.name, about_grant.unwrap_or(&alt_about));
//!
//!
//!     if let Some(active_years) = &grant.active_years {
//!         let years: Vec<String> = active_years
//!             .iter()
//!             .map(|year| year.to_string())
//!             .collect();
//!         println!(
//!             "And he's been pretty busy all these years: {}\n",
//!             years.join(", ")
//!         );
//!     }
//!
//!     // Lets get three releases from Grant.
//!     let releases_result = mc.release().get_by_artist_name_uri(
//!         &grant.uri,
//!         Some(RequestParameters::from_pagination(PaginationParameters {
//!             limit: 3,
//!             offset: 0,
//!         })),
//!     )?;
//!
//!     let releases = releases_result.data.ok_or(Error::Message(
//!         "Grant lost his releases, help!",
//!     ))?;
//!
//!     if releases.len() != 3 {
//!         return Err(Box::new(Error::Message(
//!             "We needed three releases from Grant. Hmmm..."
//!         )));
//!     }
//!
//!     println!("Listen to his work in: ");
//!     for release in &releases {
//!         println!(
//!             "  {} by {}. A {} released on {}.",
//!             release.get_title(),
//!             release.get_artists(),
//!             release.get_type(),
//!             release.get_date()
//!         );
//!     }
//!     println!();
//!
//!     if let Some(links) = &grant.links {
//!         println!("Follow Grant on: ");
//!         for link in links {
//!             println!("  {} - {}", link.platform.to_string(), link.url.to_string());
//!         }
//!         println!();
//!     }
//!
//!     Ok(())
//! }
//! ```

/// Module containing all components for the function of the API Client itself.
pub mod client;

/// Module containing types necessary to interact with the Monstercat
/// API, that are representative of some _thing_, like an artist or playlist.
pub mod mc;
