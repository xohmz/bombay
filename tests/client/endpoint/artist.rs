use crate::CLIENT;
use bombay::client::{Error, PaginationParameters, RequestParameters};
use std::fs;

#[test]
fn find_bishu() -> Result<(), Error> {
    let bishu = CLIENT.artist().get_by_name_uri("bishu")?;

    println!("Found {}!", bishu.name);

    Ok(())
}

#[test]
fn find_latest() -> Result<(), Error> {
    let artists_resp = CLIENT.artist().get_latest(None)?;

    let artists = artists_resp
        .data
        .ok_or(Error::Message("Expected to find latest artists."))?;

    let latest_artist = artists.get(0).ok_or(Error::Message(
        "Expected to find at least one artist in latest artists.",
    ))?;

    println!("Welcome {}!", latest_artist.name);

    Ok(())
}

#[ignore]
#[test]
fn get_100() -> Result<(), Error> {
    let artists_resp = CLIENT
        .artist()
        .get_all(Some(RequestParameters::from_pagination(
            PaginationParameters {
                limit: 100,
                offset: 0,
            },
        )))?;

    println!(
        "There are {} Monstercat artists, fetched {}.",
        artists_resp.total,
        artists_resp.data.map_or(0, |d| d.len())
    );

    Ok(())
}

#[test]
fn count_all() -> Result<(), Error> {
    let artists_resp = CLIENT.artist().get_all(None)?;

    println!("There are {} Monstercat artists.", artists_resp.total);

    Ok(())
}

#[test]
fn search_latest() -> Result<(), Error> {
    let search_term = "are";

    let paginated_search = RequestParameters::from_pagination(PaginationParameters {
        limit: 10,
        offset: 0,
    })
    .set_search(search_term.to_owned());

    let artists_resp = CLIENT
        .artist()
        .get_latest(Some(paginated_search))
        .map_err(|_| Error::Message("Expected to find latest artists."))?;

    let artists = artists_resp
        .data
        .ok_or(Error::Message("Expected to find latest artists."))?;

    println!(
        "From latest, found these artists with search '{}':",
        search_term
    );

    for artist in artists {
        println!("  {}", artist.name);
    }

    Ok(())
}

#[test]
fn search_all() -> Result<(), Error> {
    let search_term = "and";

    let paginated_search = RequestParameters::from_pagination(PaginationParameters {
        limit: 10,
        offset: 0,
    })
    .set_search("and".to_owned());

    let artists_resp = CLIENT
        .artist()
        .get_all(Some(paginated_search))
        .map_err(|_| Error::Message("Expected to find all artists."))?;

    let artists = artists_resp
        .data
        .ok_or(Error::Message("Expected to find all artists."))?;

    println!(
        "From all, found these artists with search '{}':",
        search_term
    );

    for artist in artists {
        println!("  {}", artist.name);
    }

    Ok(())
}

#[test]
fn get_lani_daye_photo() -> Result<(), Error> {
    let mut reader = CLIENT.artist().get_photo("lanidaye")?;

    fs::create_dir_all("downloads").unwrap();
    let mut file_out = fs::File::create("downloads/lanidaye.jpeg").unwrap();
    match std::io::copy(&mut reader, &mut file_out) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::IO(err)),
    }
}
