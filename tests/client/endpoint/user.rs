use crate::{AUTHED_CLIENT, MC_EMAIL, MC_PASSWORD, MC_TOTP_GEN};
use bombay::client::{auth::SignInOutcome, Client, Error};
use std::fs;
use std::thread;
use std::time::Duration;

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_user_info_any_login() -> Result<(), Error> {
    let mut client_unauth = Client::default();

    let outcome = client_unauth
        .sign_in(MC_EMAIL.to_owned(), MC_PASSWORD.to_owned())
        .expect("Failed to sign in");

    let client = match outcome {
        SignInOutcome::Authenticated(new_client) => Ok(new_client),
        SignInOutcome::Email(email_callback) => {
            let mut email_authed_client = Err(Error::Message(
                "Test failed, email confirmation took too long.",
            ));
            let mut attempts = 0;
            while attempts < 300 {
                attempts += 1;
                thread::sleep(Duration::from_secs(5));
                let client_res = email_callback(&mut client_unauth);
                if let Ok(client) = client_res {
                    email_authed_client = Ok(client);
                    break;
                }
            }

            email_authed_client
        }
        SignInOutcome::TOTP(totp_callback) => {
            let token = MC_TOTP_GEN.generate_current().unwrap();
            totp_callback(&mut client_unauth, token)
        }
    }?;

    let user_info = client.user().get_info()?;
    dbg!(user_info);

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_user_info_2fa_totp_login() -> Result<(), Error> {
    let mut client_unauth = Client::default();
    let client = client_unauth
        .sign_in_2fa_totp(
            MC_EMAIL.to_owned(),
            MC_PASSWORD.to_owned(),
            MC_TOTP_GEN.generate_current().unwrap(),
        )
        .expect("Failed to sign in");

    let user_info = client.user().get_info()?;
    dbg!(user_info);

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD)]
#[ignore]
#[test]
fn get_user_info_2fa_email_login() -> Result<(), Error> {
    let mut client_unauth = Client::default();

    let outcome = client_unauth
        .sign_in_2fa_email(MC_EMAIL.to_owned(), MC_PASSWORD.to_owned())
        .expect("Failed to sign in");

    thread::sleep(std::time::Duration::from_secs(45));

    let client = outcome(&mut client_unauth)?;
    let user_info = client.user().get_info()?;
    dbg!(user_info);

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_totp_qr_code_image() -> Result<(), Error> {
    let mut reader = AUTHED_CLIENT.user().get_totp_qr_code_image()?;

    let _dir = fs::create_dir_all("downloads").unwrap();
    let mut file_out = fs::File::create("downloads/qr_code.png").unwrap();

    match std::io::copy(&mut reader, &mut file_out) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IO(err)),
    }
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_licenses() -> Result<(), Error> {
    let licenses_resp = AUTHED_CLIENT.user().get_licenses(None)?;

    let licenses = licenses_resp
        .data
        .ok_or(Error::Message("Expected to find licenses."))?;

    let license = licenses
        .get(0)
        .ok_or(Error::Message("Expected to find at least one license."))?;

    dbg!(license);

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_shop_discount_code() -> Result<(), Error> {
    let shop_code = AUTHED_CLIENT.user().generate_shop_discount_code()?;

    println!(
        "Generated code '{}' for {} ({}).",
        shop_code.code, shop_code.value, shop_code.value_type
    );

    Ok(())
}

#[ignore]
#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn generate_player_code() -> Result<(), Error> {
    AUTHED_CLIENT.user().generate_player_code()
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_player_code() -> Result<(), Error> {
    let player_code = AUTHED_CLIENT.user().get_player_code()?;

    println!("Got player code '{}'.", player_code);

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[ignore]
#[test]
fn set_email() -> Result<(), Error> {
    AUTHED_CLIENT.user().set_email(MC_EMAIL.to_owned())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[ignore]
#[test]
fn set_password() -> Result<(), Error> {
    AUTHED_CLIENT
        .user()
        .set_password(MC_PASSWORD.to_owned(), MC_PASSWORD.to_owned())
}
