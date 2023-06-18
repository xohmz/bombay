use crate::client::{Client, Error, SignedIn, SignedOut};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Sign-in outcome variants.
///
/// There are three ways authentication proceeds after a successful sign-in
/// first-step. Without 2FA, you get a signed in client! With email 2FA,
/// you get a function to call back to check on the email confirmation. With
/// TOTP 2FA, you get a function to call back with your code. Both of these
/// callback functions can return:
///
/// 1. `Ok(Client<SignedIn>)` - You get a signed in client!
/// 3. `Err(Error)` - Something has gone wrong.
///
pub enum SignInOutcome {
    Authenticated(Client<SignedIn>),
    Email(EmailCallback),
    TOTP(TOTPCallback),
}

/// Type for callback function provided to check on email 2FA.
pub type EmailCallback = fn(&mut Client<SignedOut>) -> Result<Client<SignedIn>, Error>;

/// Type for callback function provided to try code for TOTP 2FA.
pub type TOTPCallback = fn(&mut Client<SignedOut>, String) -> Result<Client<SignedIn>, Error>;

/// User sign-in parameters.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SigninParameters {
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthParameters>,
}

/// 2-factor authentication parameters.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct AuthParameters {
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "TOTP", skip_serializing_if = "Option::is_none")]
    pub totp: Option<String>,
}

/// Sign-in response.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AuthReply {
    pub auth_data: Option<AuthData>,
    pub default_auth_type: Option<Auth2FAMethod>,
    #[serde(alias = "Needs2FA")]
    pub needs_2fa: bool,
}

/// Authentication data contained in sign-in response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct AuthData {
    #[serde(rename = "Email")]
    pub email: Option<AuthDataEmail>,
    #[serde(rename = "TOTP")]
    pub totp: Option<Value>,
}

/// Response from resending 2-factor authentication email.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AuthDataEmail {
    pub id: Option<String>,
    pub email: Option<String>,
}

/// Variants of 2-factor authentication.
#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub(crate) enum Auth2FAMethod {
    #[serde(rename = "Email")]
    Email,
    #[serde(rename = "TOTP")]
    Totp,
}

/// Saved authentication credentials for callback use.
#[derive(Debug)]
pub(crate) struct SavedAuthDetails {
    pub email: String,
    pub email_id: Option<String>,
    pub password: String,
}
