use crate::mc::{playlist::PlaylistItem, util::Codec};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

/// Object to set HTTP request query parameters.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RequestParameters {
    pub filters: Option<HashMap<String, String>>,
    pub codec: Option<Codec>,
    pub search: Option<String>,
    pub sort: Option<String>,
    pub creator_friendly: Option<bool>,
    pub no_gold: Option<bool>,
    pub pagination: Option<PaginationParameters>,
}

impl Default for RequestParameters {
    fn default() -> Self {
        RequestParameters {
            filters: None,
            codec: None,
            search: None,
            sort: None,
            creator_friendly: None,
            no_gold: None,
            pagination: Some(PaginationParameters::default()),
        }
    }
}

impl From<RequestParameters> for HashMap<String, String> {
    fn from(val: RequestParameters) -> Self {
        let mut queries = HashMap::new();

        if let Some(format) = val.codec {
            queries.insert("format".to_owned(), format.to_string());
        }

        if let Some(search) = val.search {
            queries.insert("search".to_owned(), search);
        }

        if let Some(sort) = val.sort {
            queries.insert("sort".to_owned(), sort);
        }

        if let Some(pagination) = val.pagination {
            let pagination_map: HashMap<String, String> = pagination.into();
            queries.extend(pagination_map)
        }

        queries
    }
}

impl RequestParameters {
    /// Create request parameters from pagination parameters.
    pub fn from_pagination(pagination: PaginationParameters) -> Self {
        RequestParameters {
            filters: None,
            codec: None,
            search: None,
            sort: None,
            creator_friendly: None,
            no_gold: None,
            pagination: Some(pagination),
        }
    }

    /// Create request parameters from search parameters.
    pub fn from_search(search_term: String) -> Self {
        RequestParameters {
            filters: None,
            codec: None,
            search: Some(search_term),
            sort: None,
            creator_friendly: None,
            no_gold: None,
            pagination: Some(PaginationParameters::default()),
        }
    }

    /// Create request parameters from audio codec.
    pub fn from_codec(codec: Codec) -> Self {
        RequestParameters {
            filters: None,
            codec: Some(codec),
            search: None,
            sort: None,
            creator_friendly: None,
            no_gold: None,
            pagination: None,
        }
    }

    /// Set request parameters pagination.
    pub fn set_pagination(mut self, pagination: PaginationParameters) -> Self {
        self.pagination = Some(pagination);
        self
    }

    /// Set request parameters search.
    pub fn set_search(mut self, search_term: String) -> Self {
        self.search = Some(search_term);
        self
    }
}

/// Type to set pagination for response.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaginationParameters {
    pub limit: usize,
    pub offset: usize,
}

impl Default for PaginationParameters {
    /// By default, pagination is limited to 3 items starting at offset 0.
    /// This is mostly arbitrary, but designed to not burden the MC server(s).
    fn default() -> Self {
        PaginationParameters {
            limit: 3,
            offset: 0,
        }
    }
}

impl From<PaginationParameters> for HashMap<String, String> {
    fn from(val: PaginationParameters) -> Self {
        let mut queries = HashMap::new();
        queries.insert("limit".to_owned(), val.limit.to_string());
        queries.insert("offset".to_owned(), val.offset.to_string());
        queries
    }
}

/// Valid operations for single playlist item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaylistItemOperations {
    Add,
    Remove,
    Up,
    Down,
    To,
}

impl Display for PlaylistItemOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaylistItemOperations::Add => write!(f, "add"),
            PlaylistItemOperations::Remove => write!(f, "remove"),
            PlaylistItemOperations::Up => write!(f, "up"),
            PlaylistItemOperations::Down => write!(f, "down"),
            PlaylistItemOperations::To => write!(f, "to"),
        }
    }
}

impl From<PlaylistItemOperations> for HashMap<String, String> {
    fn from(val: PlaylistItemOperations) -> Self {
        let mut queries = HashMap::new();
        queries.insert("type".to_owned(), val.to_string());
        queries
    }
}

/// Valid operations for multiple playlist items.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PlaylistItemsOperations {
    Add,
    Remove,
}

impl Display for PlaylistItemsOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaylistItemsOperations::Add => write!(f, "add"),
            PlaylistItemsOperations::Remove => write!(f, "remove"),
        }
    }
}

impl From<PlaylistItemsOperations> for HashMap<String, String> {
    fn from(val: PlaylistItemsOperations) -> Self {
        let mut queries = HashMap::new();
        queries.insert("type".to_owned(), val.to_string());
        queries
    }
}

/// Object to create playlist item modification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaylistItemMod {
    pub move_to: Option<u32>,
    pub record: PlaylistItem,
}

/// Object to create playlist items modification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaylistItemsMod {
    pub records: Vec<PlaylistItem>,
}
