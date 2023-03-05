use std::collections::HashMap;

use crate::client::endpoints::TargetAPI;
use crate::client::{EndpointUser, Error, RequestParameters, Wrapped};
use crate::client::{Paginated, SignedIn};
use crate::mc::user::{
    EditableSettings, EditableUserInfo, NewEmail, NewPassword, NotificationInterests, PlayerCode,
    Settings, ShopCode, User,
};
use crate::mc::util::{ClaimVideoId, License, LicenseID};
use serde_json::Value;

impl EndpointUser<'_, SignedIn> {
    /// Get user information and settings.
    pub fn get_info(&self) -> Result<(Settings, User), Error> {
        let mut user_info_wrapper = self.client.get::<Wrapped<Value>>(
            TargetAPI::Player,
            "/me",
            None::<HashMap<String, String>>,
        )?;

        let settings_val = user_info_wrapper
            .remove("Settings")
            .ok_or(Error::NotFound("user settings"))?;

        let release_obj = serde_json::from_value::<Settings>(settings_val)
            .map_err(|err| Error::Deserialization(err))?;

        let user_val = user_info_wrapper
            .remove("User")
            .ok_or(Error::NotFound("user information"))?;

        let tracks_obj =
            serde_json::from_value::<User>(user_val).map_err(|err| Error::Deserialization(err))?;

        Ok((release_obj, tracks_obj))
    }

    /// Set some editable user information.
    pub fn set_info(&self, user_info: EditableUserInfo) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me",
            None::<HashMap<String, String>>,
            Some(user_info),
        )
    }

    /// Set some editable user settings.
    pub fn set_settings(&self, user_info: EditableSettings) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/settings",
            None::<HashMap<String, String>>,
            Some(user_info),
        )
    }

    /// Get streaming widget player code.
    pub fn get_player_code(&self) -> Result<String, Error> {
        let resp = self.client.get::<PlayerCode>(
            TargetAPI::Player,
            "/me/player-code",
            None::<HashMap<String, String>>,
        )?;

        Ok(resp.player_code)
    }

    /// Generate streaming widget player code.
    pub fn generate_player_code(&self) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/player-code",
            None::<HashMap<String, String>>,
            None::<()>,
        )
    }

    /// Set a account and login new email.
    pub fn set_email(&self, new_email: String) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/email",
            None::<HashMap<String, String>>,
            Some(NewEmail { new_email }),
        )
    }

    /// Set a new password.
    pub fn set_password(&self, old_password: String, new_password: String) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/password",
            None::<HashMap<String, String>>,
            Some(NewPassword {
                old_password,
                new_password,
            }),
        )
    }

    /// Enable 2FA with TOTP
    pub fn enable_2fa_totp(&self) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/two-factor/enable-totp",
            None::<HashMap<String, String>>,
            None::<()>,
        )
    }

    /// Disable 2FA with TOTP
    pub fn disable_2fa_totp(&self) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/two-factor/disable-totp",
            None::<HashMap<String, String>>,
            None::<()>,
        )
    }

    /// Enable 2FA with email confirmation link.
    pub fn enable_2fa_email(&self) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/two-factor/enable-email",
            None::<HashMap<String, String>>,
            None::<()>,
        )
    }

    /// Disable 2FA with email confirmation link.
    pub fn disable_2fa_email(&self) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/two-factor/disable-email",
            None::<HashMap<String, String>>,
            None::<()>,
        )
    }

    /// Get TOTP QR code PNG image.
    pub fn get_totp_qr_code_image(&self) -> Result<Box<dyn std::io::Read + Send + Sync>, Error> {
        self.client.get_reader(
            TargetAPI::Player,
            "/me/two-factor/totp-qr",
            None::<HashMap<String, String>>,
        )
    }

    /// Set email notification preferences.
    pub fn set_notification_interests(
        &self,
        interests: Vec<NotificationInterests>,
    ) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            "/me/notifications",
            None::<HashMap<String, String>>,
            Some(interests),
        )
    }

    /// Get creator licenses registered with your account.
    pub fn get_licenses(
        &self,
        parameters: Option<RequestParameters>,
    ) -> Result<Paginated<License>, Error> {
        self.client
            .get::<Wrapped<Paginated<License>>>(TargetAPI::Player, "/self/licenses", parameters)?
            .remove("Licenses")
            .ok_or(Error::NotFound("licenses"))
    }

    /// Delete creator license registered with your account.
    pub fn remove_license(&self, license_id: LicenseID) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            &format!("/self/license/{license_id}/delete"),
            None::<HashMap<String, String>>,
            None::<()>,
        )
    }

    /// Delete creator license registered with your account.
    pub fn remove_video_claim(&self, video_id: String) -> Result<(), Error> {
        self.client.post_empty_response(
            TargetAPI::Player,
            &format!("/me/remove-claims"),
            None::<HashMap<String, String>>,
            Some(ClaimVideoId { video_id: video_id }),
        )
    }

    /// Generate gold member shop discount code.
    ///
    /// These are supposed to be used for 30 days. Try to reuse instead of generating on demand.
    pub fn generate_shop_discount_code(&self) -> Result<ShopCode, Error> {
        self.client
            .post::<Wrapped<ShopCode>>(
                TargetAPI::Player,
                "/me/benefits/shop-code",
                None::<HashMap<String, String>>,
                None::<()>,
            )?
            .remove("ShopCode")
            .ok_or(Error::NotFound("shop code"))
    }
}
