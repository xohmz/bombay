use bombay::client::{Client, SignedIn};
use lazy_static::lazy_static;
use totp_rs::{Algorithm, Secret, TOTP};

lazy_static! {
    static ref CLIENT: Client = Client::default();
}

lazy_static! {
    static ref MC_EMAIL: &'static str = option_env!("MC_EMAIL").unwrap();
}

lazy_static! {
    static ref MC_PASSWORD: &'static str =  option_env!("MC_PASSWORD").unwrap();
}

lazy_static! {
    static ref MC_TOTP_SECRET: Secret = Secret::Encoded( option_env!("MC_TOTP_SECRET").unwrap().to_owned());
}

lazy_static! {
    static ref MC_TOTP_GEN: TOTP = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        MC_TOTP_SECRET.to_bytes().unwrap(),
    )
    .unwrap();
}

lazy_static! {
    static ref AUTHED_CLIENT: Client<SignedIn> = Client::default().sign_in_2fa_totp(
            MC_EMAIL.to_owned(),
            MC_PASSWORD.to_owned(),
            MC_TOTP_GEN.generate_current().unwrap(),
        )
        .expect("Failed to sign in");
}

mod client;
