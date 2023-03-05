use crate::{AUTHED_CLIENT, CLIENT};
use bombay::client::Error;
use bombay::mc::release::{AnyRelease, CatalogID, ReleaseID, TrackID};
use bombay::mc::util::Codec;
use std::fs;
use uuid::uuid;

#[test]
fn find_latest() -> Result<(), Error> {
    let releases_resp = CLIENT.release().get_latest(None)?;

    let releases = releases_resp
        .data
        .ok_or(Error::Message("Expected to find latest releases."))?;

    let release = releases.get(0).ok_or(Error::Message(
        "Expected to find at least one release in latest releases",
    ))?;

    println!(
        "Listen to {} from {}!",
        release.get_title(),
        release.get_artists()
    );

    Ok(())
}

#[test]
fn find_latest_from_rogue() -> Result<(), Error> {
    let releases_resp = CLIENT.release().get_by_artist_name_uri("rogue", None)?;

    let releases = releases_resp.data.ok_or(Error::Message(
        "Expected to find latest releases from Rogue.",
    ))?;

    let release = releases.get(0).ok_or(Error::Message(
        "Expected to find at least one release in latest releases from Rogue",
    ))?;

    println!(
        "{} released {} on {}!",
        release.get_artists(),
        release.get_title(),
        release.get_date()
    );

    Ok(())
}

#[test]
fn get_souvenir_details() -> Result<(), Error> {
    let release_tracks = CLIENT
        .release()
        .get_by_catalog_id(&CatalogID("MCS1186".to_owned()))?;

    let (any_release, tracks) = release_tracks;

    assert_eq!(any_release.get_title(), "Souvenir");
    assert_eq!(any_release.get_artists(), "Whales feat. Dutch Melrose");

    match &any_release {
        AnyRelease::Release(release) => {
            assert_eq!(release.grid, Some("A10443ZXECUUYVB2CO".to_owned()))
        }
        _ => panic!("This release should be of Release type!"),
    }

    assert_eq!(tracks.len(), 1);
    assert_eq!(tracks[0].isrc, "CA6D22100326");

    Ok(())
}

#[test]
fn get_related_to_oxygen() -> Result<(), Error> {
    let releases_resp = CLIENT.release().get_related_by_id(
        &ReleaseID(uuid!("6a58b6d2-bbec-4847-8dcf-45023a930968")),
        None,
    )?;

    let releases = releases_resp.data.ok_or(Error::Message(
        "Expected to find release Oxygen (6a58b6d2-bbec-4847-8dcf-45023a930968).",
    ))?;

    let release = releases.get(0).ok_or(Error::Message(
        "Expected to find at least one release related to Oxygen (6a58b6d2-bbec-4847-8dcf-45023a930968).",
    ))?;

    println!("Release similar to Oxygen:\n{:#?}", release);

    Ok(())
}

#[test]
fn find_latest_related() -> Result<(), Error> {
    let latest_releases_resp = CLIENT.release().get_latest(None)?;

    let latest_releases = latest_releases_resp
        .data
        .ok_or(Error::Message("Expected to find latest releases."))?;

    let latest_release = latest_releases.get(0).ok_or(Error::Message(
        "Expected to find at least one release in latest releases",
    ))?;

    let related_releases_resp = CLIENT
        .release()
        .get_related_by_id(latest_release.get_release_id(), None)?;

    let related_releases = related_releases_resp
        .data
        .ok_or(Error::Message("Expected to find related releases."))?;

    let related_release_opt = related_releases.get(0);

    match related_release_opt {
        Some(related_release) => println!(
            "{} is similar to latest release {}.",
            related_release.get_title(),
            latest_release.get_title()
        ),

        None => println!("No similar releases found for latest release."),
    }

    Ok(())
}

#[test]
fn stream_no_service() -> Result<(), Error> {
    let mut reader = CLIENT.release().stream_by_ids(
        &ReleaseID(uuid!("3efe8e1e-8ec9-440b-8c00-d825f777c83e")),
        &TrackID(uuid!("6b5401bc-06d0-41e8-ab16-7742f2aa40bf")),
    )?;

    let _dir = fs::create_dir_all("downloads").unwrap();
    let mut file_out = fs::File::create("downloads/far_out.mp3").unwrap();

    match std::io::copy(&mut reader, &mut file_out) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IO(err)),
    }
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn download_every_little_thing() -> Result<(), Error> {
    let mut reader = AUTHED_CLIENT.release().download_by_ids(
        &ReleaseID(uuid!("e7c6a280-6af3-4101-af9f-5c809afb6541")),
        &TrackID(uuid!("2399321a-b7ba-406d-976f-0c30054ab938")),
        Some(Codec::FLAC),
    )?;

    let _dir = fs::create_dir_all("downloads").unwrap();
    let mut file_out = fs::File::create("downloads/everything_little_thing_flac.flac").unwrap();

    match std::io::copy(&mut reader, &mut file_out) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IO(err)),
    }
}

#[test]
fn download_feelings_cover_art() -> Result<(), Error> {
    let mut reader = CLIENT
        .release()
        .get_cover_art(&CatalogID("742779546913".to_owned()))?;

    let _dir = fs::create_dir_all("downloads").unwrap();
    let mut file_out = fs::File::create("downloads/feelings_cover_art.jpeg").unwrap();

    match std::io::copy(&mut reader, &mut file_out) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IO(err)),
    }
}
