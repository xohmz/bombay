use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pagination information and a vector of some MC type.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Paginated<T> {
    pub data: Option<Vec<T>>,
    pub not_found: Option<bool>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

/// Some MC type wrapped in a HashMap to facilitate dynamic parent key.
pub type Wrapped<T> = HashMap<String, T>;
