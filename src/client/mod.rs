#![doc = include_str!("README.md")]

pub mod auth;
pub mod endpoints;
mod error;
mod request;
mod response;

use auth::*;
use const_format::formatcp;
use endpoints::*;
pub use error::*;
pub use request::*;
pub use response::*;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::{collections::HashMap, fmt::Display};
use ureq::{self, Request, Response};

const USER_AGENT: &str = formatcp!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
const URL_PLAYER_API: &str = "https://player.monstercat.app/api";
const URL_WWW_API: &str = "https://www.monstercat.com/";

/// Zero-size type to indicate the user signed-out state of a client.
#[derive(Debug)]
pub struct SignedOut;
/// Zero-size type to indicate the user signed-in state of a client.
#[derive(Debug)]
pub struct SignedIn;

/// Client for interacting with the Monstercat API.
///
/// Note that this struct uses zero-sized phantom data expose different
/// functionality based on whether or not the user is authenticated.
///
#[derive(Debug)]
pub struct Client<ClientAuthState = SignedOut> {
    pub agent: ureq::Agent,
    auth: Option<SavedAuthDetails>,
    url_player_api: String,
    url_www_api: String,
    user_agent: String,
    user_state: PhantomData<ClientAuthState>,
}

impl Default for Client<SignedOut> {
    fn default() -> Self {
        Client {
            user_state: PhantomData,
            url_player_api: URL_PLAYER_API.to_owned(),
            url_www_api: URL_WWW_API.to_owned(),
            user_agent: USER_AGENT.to_owned(),
            auth: None,
            agent: ureq::Agent::new(),
        }
    }
}

impl<ClientAuthState> Client<ClientAuthState> {
    /// Get endpoint for artist-related functions.
    pub fn artist(&self) -> EndpointArtist<ClientAuthState> {
        EndpointArtist { client: self }
    }

    /// Get endpoint for mood-related functions.
    pub fn mood(&self) -> EndpointMood<ClientAuthState> {
        EndpointMood { client: self }
    }

    /// Get endpoint for playlist-related functions.
    pub fn playlist(&self) -> EndpointPlaylist<ClientAuthState> {
        EndpointPlaylist { client: self }
    }

    /// Get endpoint for release-related functions.
    pub fn release(&self) -> EndpointRelease<ClientAuthState> {
        EndpointRelease { client: self }
    }

    /// Use the client to make a custom GET request to the API.
    pub fn get<RT: DeserializeOwned>(
        &self,
        api_type: TargetAPI,
        path: impl AsRef<str> + Display,
        queries: Option<impl Into<HashMap<String, String>>>,
    ) -> Result<RT, Error> {
        self.process_response::<RT>(
            self.build_get_request(api_type, path, queries.map(|q| q.into()))
                .call(),
        )
    }

    /// Use the client to make a custom GET request to the API and get a reader to the content.
    pub fn get_reader(
        &self,
        api_type: TargetAPI,
        path: impl AsRef<str> + Display,
        queries: Option<impl Into<HashMap<String, String>>>,
    ) -> Result<Box<dyn std::io::Read + Send + Sync>, Error> {
        let response = self
            .build_get_request(api_type, path, queries.map(|q| q.into()))
            .call();

        match response {
            Ok(res) => Ok(res.into_reader()),
            Err(err) => Err(Error::Request(Box::new(err))),
        }
    }

    /// Use the client to make a custom POST request to the API.
    pub fn post<RT: DeserializeOwned>(
        &self,
        api_type: TargetAPI,
        path: impl AsRef<str> + Display,
        queries: Option<impl Into<HashMap<String, String>>>,
        data: Option<impl serde::Serialize>,
    ) -> Result<RT, Error> {
        let request = self.build_post_request(api_type, path, queries.map(|q| q.into()));
        match data {
            Some(data) => self.process_response::<RT>(request.send_json(data)),
            None => self.process_response::<RT>(request.call()),
        }
    }

    /// Use the client to make a custom POST request to the API, expecting empty response.
    pub fn post_empty_response(
        &self,
        api_type: TargetAPI,
        path: impl AsRef<str> + Display,
        queries: Option<impl Into<HashMap<String, String>>>,
        data: Option<impl serde::Serialize>,
    ) -> Result<(), Error> {
        let request = self.build_post_request(api_type, path, queries.map(|q| q.into()));
        match data {
            Some(data) => self.process_empty_response(request.send_json(data)),
            None => self.process_empty_response(request.call()),
        }
    }

    /// Construct get request for targeted API, including any query parameters.
    fn build_get_request(
        &self,
        api: TargetAPI,
        path: impl AsRef<str> + Display,
        queries: Option<HashMap<String, String>>,
    ) -> Request {
        let request = match api {
            TargetAPI::Player => self.agent.get(&format!("{}{}", self.url_player_api, path)),
            TargetAPI::WWW => self.agent.get(&format!("{}{}", self.url_www_api, path)),
        }
        .set("User-Agent", &self.user_agent)
        .set("Accept", "application/json");

        self.add_request_queries(request, queries)
    }

    /// Construct post request for targeted API.
    fn build_post_request(
        &self,
        api: TargetAPI,
        path: impl AsRef<str> + Display,
        queries: Option<HashMap<String, String>>,
    ) -> Request {
        let request = match api {
            TargetAPI::Player => self.agent.post(&format!("{}{}", self.url_player_api, path)),
            TargetAPI::WWW => self.agent.post(&format!("{}{}", self.url_www_api, path)),
        }
        .set("User-Agent", &self.user_agent)
        .set("Accept", "application/json");

        self.add_request_queries(request, queries)
    }

    fn add_request_queries(
        &self,
        mut req: Request,
        queries: Option<HashMap<String, String>>,
    ) -> Request {
        if let Some(parameters) = queries {
            for (parameter, value) in parameters {
                req = req.query(&parameter, &value);
            }
        }

        req
    }

    /// If successful, return serialized object. Otherwise, return wrapped error from request or response.
    fn process_response<RT: DeserializeOwned>(
        &self,
        result: Result<Response, ureq::Error>,
    ) -> Result<RT, Error> {
        match result {
            Ok(response) => response.into_json::<RT>().map_err(Error::IO),
            Err(err) => Err(Error::Request(Box::new(err))),
        }
    }

    /// If successful, ignore response and return Ok(()). Otherwise, return wrapped error.
    fn process_empty_response(&self, result: Result<Response, ureq::Error>) -> Result<(), Error> {
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Request(Box::new(err))),
        }
    }
}

impl Client<SignedOut> {
    /// Create a new signed-out client.
    pub fn new(player_api: String, www_api: String) -> Client<SignedOut> {
        Client {
            user_state: PhantomData,
            url_player_api: player_api,
            url_www_api: www_api,
            user_agent: USER_AGENT.to_owned(),
            auth: None,
            agent: ureq::Agent::new(),
        }
    }

    /// Sign in and get a sign-in outcomes, depending on 2FA settings.
    pub fn sign_in(&mut self, email: String, password: String) -> Result<SignInOutcome, Error> {
        let signin_parameters = SigninParameters {
            auth: None,
            email,
            password,
        };

        let signin_res = self.post::<AuthReply>(
            TargetAPI::Player,
            "/sign-in",
            None::<HashMap<String, String>>,
            Some(signin_parameters.clone()),
        );

        // If we get a valid response and need 2FA, handle that.
        // Do nothing with an error. For some reason, a valid login
        // with no 2FA will produce a status 400 response.
        if let Ok(resp) = signin_res {
            if resp.needs_2fa {
                let second_factor = resp
                    .default_auth_type
                    .ok_or(Error::SignIn("Bad sign-in response, missing 2FA method."))?;

                let auth_data = resp
                    .auth_data
                    .ok_or(Error::SignIn("Bad sign-in response, missing auth data."))?;

                let mut auth = SavedAuthDetails {
                    email: signin_parameters.email.clone(),
                    email_id: None,
                    password: signin_parameters.password,
                };

                match second_factor {
                    Auth2FAMethod::Email => {
                        auth.email_id = Some(
                            auth_data
                                .email
                                .ok_or(Error::SignIn(
                                    "Bad sign-in response, missing email auth data.",
                                ))?
                                .id
                                .ok_or(Error::SignIn("Bad sign-in response, missing email id."))?,
                        );
                        self.auth = Some(auth);

                        return Ok(SignInOutcome::Email(Self::mfa_callback_email));
                    }
                    Auth2FAMethod::Totp => match auth_data.totp {
                        Some(_) => {
                            self.auth = Some(auth);
                            return Ok(SignInOutcome::TOTP(Self::mfa_callback_totp));
                        }
                        None => {
                            return Err(Error::SignIn("Bad sign-in response, missing TOTP."));
                        }
                    },
                }
            }
        }

        Ok(SignInOutcome::Authenticated(self.verify_signin_cookie()?))
    }

    /// Try to sign in using one of the saved MFA authentication parameters and handle response.
    fn try_mfa_signin(
        &mut self,
        signin_param: SigninParameters,
    ) -> Result<Client<SignedIn>, Error> {
        match self.post::<AuthReply>(
            TargetAPI::Player,
            "/sign-in",
            None::<HashMap<String, String>>,
            Some(signin_param),
        ) {
            Ok(_) => self.verify_signin_cookie(),
            Err(Error::Request(boxed_err)) => match *boxed_err {
                ureq::Error::Status(200, _) => self.verify_signin_cookie(),
                _ => Err(Error::Request(boxed_err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Signing by trying to force the use of email 2FA.
    pub fn sign_in_2fa_email(
        &mut self,
        email: String,
        password: String,
    ) -> Result<EmailCallback, Error> {
        let signin_parameters = SigninParameters {
            auth: None,
            email,
            password,
        };

        let signin_res = self.post::<AuthDataEmail>(
            TargetAPI::Player,
            "/me/two-factor/resend-email",
            None::<HashMap<String, String>>,
            Some(signin_parameters.clone()),
        );

        if let Ok(email_auth_data) = signin_res {
            let id = email_auth_data
                .id
                .ok_or(Error::SignIn("Bad sign-in response, missing email id."))?;

            self.auth = Some(SavedAuthDetails {
                email: signin_parameters.email.clone(),
                email_id: Some(id),
                password: signin_parameters.password,
            });

            return Ok(Self::mfa_callback_email);
        }

        Err(Error::SignIn("Bad sign-in response, missing email id."))
    }

    /// Immediately try to sign in with 2FA TOTP code.
    pub fn sign_in_2fa_totp(
        &mut self,
        email: String,
        password: String,
        code: String,
    ) -> Result<Client<SignedIn>, Error> {
        let signin_parameters = SigninParameters {
            auth: None,
            email,
            password,
        };

        let signin_res = self.post::<AuthReply>(
            TargetAPI::Player,
            "/sign-in",
            None::<HashMap<String, String>>,
            Some(signin_parameters.clone()),
        );

        // If we get a valid response and need 2FA, handle that.
        // Do nothing with an error. For some reason, a valid login
        // with no 2FA will produce a status 400 response.
        if let Ok(resp) = signin_res {
            if resp.needs_2fa {
                let second_factor = resp
                    .default_auth_type
                    .ok_or(Error::SignIn("Bad sign-in response, missing 2FA method."))?;

                self.auth = Some(SavedAuthDetails {
                    email: signin_parameters.email.clone(),
                    email_id: None,
                    password: signin_parameters.password,
                });

                if let Auth2FAMethod::Totp = second_factor {
                    return self.mfa_callback_totp(code);
                }
            }
        }

        self.verify_signin_cookie()
    }

    /// Function to try login with email confirmation after username and password was already provided.
    fn mfa_callback_email(&mut self) -> Result<Client<SignedIn>, Error> {
        let auth = self
            .auth
            .as_ref()
            .ok_or(Error::SignIn("Missing 2FA data, needed for email 2FA."))?;

        self.try_mfa_signin(SigninParameters {
            email: auth.email.clone(),
            password: auth.password.clone(),
            auth: Some(AuthParameters {
                email: auth.email_id.clone(),
                totp: None,
            }),
        })
    }

    /// Function to try login with TOTP code after username and password was already provided.
    fn mfa_callback_totp(&mut self, code: String) -> Result<Client<SignedIn>, Error> {
        let auth = self
            .auth
            .as_ref()
            .ok_or(Error::SignIn("Missing 2FA data, needed for TOTP 2FA."))?;

        self.try_mfa_signin(SigninParameters {
            email: auth.email.clone(),
            password: auth.password.clone(),
            auth: Some(AuthParameters {
                email: None,
                totp: Some(code),
            }),
        })
    }

    /// After a login strategy (may have) worked, confirm there is a login cookie.
    fn verify_signin_cookie(&mut self) -> Result<Client<SignedIn>, Error> {
        // Ensure saved auth details are removed.
        self.auth = None;

        match self
            .agent
            .cookie_store()
            .get("player.monstercat.app", "/", "cid")
        {
            Some(_) => Ok(Client {
                agent: self.agent.clone(),
                auth: None,
                url_player_api: self.url_player_api.clone(),
                url_www_api: self.url_www_api.clone(),
                user_agent: self.user_agent.clone(),
                user_state: PhantomData,
            }),
            None => Err(Error::SignIn(
                "Sign-in verification failed, missing cookie.",
            )),
        }
    }
}

impl Client<SignedIn> {
    /// Get endpoint for user-related functions.
    pub fn user(&self) -> EndpointUser<SignedIn> {
        EndpointUser { client: self }
    }
}
