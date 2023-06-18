mod endpoint;

use bombay::client::{Client, Error, PaginationParameters, RequestParameters};
use std::error;
use uuid::uuid;

#[test]
fn about_grant() -> Result<(), Box<dyn error::Error>> {
    // We don't need authentication to fetch some information.
    let mc = Client::default();

    // Lets search for one of my favorite artists and bail if there are errors.
    let search_results = mc
        .artist()
        .get_all(Some(RequestParameters::from_search("Grant".to_owned())))
        .map_err(|_| Error::Message("Expected to find artists."))?;

    // I also expect some data in the response.
    let artists = search_results
        .data
        .ok_or(Error::Message("Expected to find artists."))?;

    // And not empty data!.
    let grant = artists
        .get(0)
        .ok_or(Error::Message("Expected to find at least one artist."))?;

    grant
        .id
        .eq(&uuid!("27063fd3-4fba-4119-9af0-5001e925b0d2"))
        .then_some(())
        .ok_or(Error::Message(
            "Expected to find artist Grant, but found another.",
        ))?;

    // Alright lets learn about Grant!
    let about_grant = grant.about.as_ref();
    let alt_about = r#"
    Grant's music makes any moment better.
    You'll find out how many emotions you can feel at once!
    "#
    .to_owned();
    println!("{}:\n{}\n", &grant.name, about_grant.unwrap_or(&alt_about));

    let active_years: Vec<String> = grant
        .active_years
        .as_ref()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|year| year.to_string())
        .collect();
    println!(
        "And he's been pretty busy all these years: {}\n",
        active_years.join(", ")
    );

    // Lets get three releases from Grant.
    let releases_result = mc
        .release()
        .get_by_artist_name_uri(
            &grant.uri,
            Some(RequestParameters::from_pagination(PaginationParameters {
                limit: 3,
                offset: 0,
            })),
        )
        .map_err(|_| Error::Message("Expected to find releases from Grant."))?;

    let releases = releases_result
        .data
        .ok_or(Error::NotFound("Grant's releases"))?;

    if releases.len() != 3 {
        return Err(Box::new(Error::Message(
            "Expected three releases from Grant.",
        )));
    }

    println!("Listen to his work in: ");
    for release in &releases {
        println!(
            "  {} by {}. A {} released on {}.",
            release.get_title(),
            release.get_artists(),
            release.get_type(),
            release.get_date()
        );
    }
    println!();

    println!("Follow Grant on: ");
    for link in grant.links.as_ref().unwrap_or(&Vec::new()) {
        println!("  {} - {}", link.platform, link.url);
    }
    println!();

    Ok(())
}
