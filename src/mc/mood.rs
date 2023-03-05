use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Display;
use uuid::Uuid;

/// Mood object used for categorizing songs.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mood {
    pub id: Uuid,
    pub name: String,
    pub uri: String,
    pub description: String,
    pub omitted_genres: Option<Value>,
    pub start_date: Option<Timestamp>,
    pub timezone: String,
    pub params: Option<Vec<MoodParamConfig>>,
    pub omitted_songs: Option<Value>,
    pub tile_file_id: Uuid,
    pub background_file_id: Uuid,
}

/// Configuration of mood parameter.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MoodParamConfig {
    pub mood_id: Uuid,
    pub param: MoodParam,
    pub min: f32,
    pub max: f32,
}

/// Variants of mood parameters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MoodParam {
    Acousticness,
    Danceability,
    Energy,
    Instrumentalness,
    Liveness,
    Loudness,
    Speechiness,
    Valence,
}

impl Display for MoodParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoodParam::Acousticness => write!(f, "acousticness"),
            MoodParam::Danceability => write!(f, "danceability"),
            MoodParam::Energy => write!(f, "energy"),
            MoodParam::Instrumentalness => write!(f, "instrumentalness"),
            MoodParam::Liveness => write!(f, "liveness"),
            MoodParam::Loudness => write!(f, "loudness"),
            MoodParam::Speechiness => write!(f, "speechniess"),
            MoodParam::Valence => write!(f, "valance"),
        }
    }
}
