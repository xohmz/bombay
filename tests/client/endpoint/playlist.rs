use crate::AUTHED_CLIENT;
use crate::CLIENT;
use bombay::client::PlaylistItemMod;
use bombay::client::PlaylistItemOperations;
use bombay::client::{Error, PlaylistItemsMod, PlaylistItemsOperations};
use bombay::mc::playlist::PlaylistID;
use bombay::mc::playlist::PlaylistItem;
use bombay::mc::release::AnyRelease;
use uuid::uuid;

#[test]
fn get_top_30() -> Result<(), Error> {
    let playlist_endpoint = CLIENT.playlist();
    let top_30_playlist = playlist_endpoint.by_id(playlist_endpoint.get_top_30_playlist_id())?;

    println!("Found:");
    println!("{}: {}", top_30_playlist.title, top_30_playlist.description);

    Ok(())
}

#[test]
fn get_top_30_tracks() -> Result<(), Error> {
    let playlist_endpoint = CLIENT.playlist();
    let top_30_playlist = playlist_endpoint.by_id(playlist_endpoint.get_top_30_playlist_id())?;

    let top_30_tracks = playlist_endpoint.get_tracks_by_playlist_id(top_30_playlist.id)?;
    let tracks = top_30_tracks
        .data
        .ok_or(Error::Message("Expected to find latest releases."))?;

    println!("Found:");
    println!(
        "Playlist {} has {} tracks.",
        top_30_playlist.title, top_30_tracks.total
    );

    let hottest_track = tracks
        .get(0)
        .ok_or(Error::Message("Expected to find latest releases."))?;

    println!("{} is really hot right now!", hottest_track.get_title());

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_user_playlists() -> Result<(), Error> {
    let playlists = AUTHED_CLIENT.playlist().get_all()?;

    dbg!(playlists);

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_test_playlist() -> Result<(), Error> {
    let playlist = AUTHED_CLIENT
        .playlist()
        .by_id(PlaylistID(uuid!("f6cbaba8-5a86-4fb8-bd4a-1cc3d6ad22e0")))?;

    dbg!(playlist);

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn get_test_playlist_items() -> Result<(), Error> {
    let tracks = AUTHED_CLIENT
        .playlist()
        .get_tracks_by_playlist_id(PlaylistID(uuid!("f6cbaba8-5a86-4fb8-bd4a-1cc3d6ad22e0")))?;

    dbg!(tracks);

    Ok(())
}

#[test_with::env(MC_EMAIL, MC_PASSWORD, MC_TOTP_SECRET)]
#[test]
fn many_playlist_tests() -> Result<(), Error> {
    let playlist_endpoint = AUTHED_CLIENT.playlist();
    let playlist =
        playlist_endpoint.by_id(PlaylistID(uuid!("f6cbaba8-5a86-4fb8-bd4a-1cc3d6ad22e0")))?;

    let tracks = playlist_endpoint
        .get_tracks_by_playlist_id(playlist.id)?
        .data
        .ok_or(Error::Message("Expected tracks in test playlist"))?;

    let playlist_items: Vec<PlaylistItem> = tracks
        .iter()
        .enumerate()
        .filter_map(|(index, any_rel)| match any_rel {
            AnyRelease::Track(track) => Some(PlaylistItem {
                playlist_id: playlist.id,
                sort: index,
                release_id: track.release.id,
                track_id: track.id,
            }),
            _ => None,
        })
        .collect();

    let mut new_playlist = playlist.clone();
    new_playlist.title = "TestDuplicate".to_owned();

    let new_playlist_id = playlist_endpoint.create(new_playlist)?;

    playlist_endpoint.modify_items(
        new_playlist_id,
        PlaylistItemsOperations::Add,
        PlaylistItemsMod {
            records: playlist_items.clone(),
        },
    )?;

    let _duplicate_tracks = playlist_endpoint
        .get_tracks_by_playlist_id(new_playlist_id)?
        .data
        .ok_or(Error::Message("Expected tracks in test playlist"))?;

    playlist_endpoint.modify_item(
        new_playlist_id,
        PlaylistItemOperations::Down,
        PlaylistItemMod {
            move_to: None,
            record: playlist_items[0].clone(),
        },
    )?;

    playlist_endpoint.delete(new_playlist_id)?;

    Ok(())
}
