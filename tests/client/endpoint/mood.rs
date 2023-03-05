use crate::CLIENT;
use bombay::client::Error;

#[test]
fn find_and_fetch_all() -> Result<(), Error> {
    let all_moods_resp = CLIENT.mood().get_all(None)?;

    let all_moods = all_moods_resp
        .data
        .ok_or(Error::Message("Expected to find all moods."))?;

    println!("Found all moods:");
    for mood in &all_moods {
        println!("  {} ({})", mood.name, mood.id)
    }
    println!("");

    println!("Fetching all moods:");
    for mood in &all_moods {
        println!("  {}:", mood.name);
        let mood_with_params = CLIENT.mood().get_by_name_uri(&mood.uri)?;

        let params = mood_with_params
            .params
            .ok_or(Error::Message("Missing parameters in mood."))?;

        for param in &params {
            println!(
                "    {} is between {} and {}",
                param.param, param.min, param.max
            );
        }
    }
    println!("");

    Ok(())
}
