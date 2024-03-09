use std::collections::BTreeMap;
use serde_json::Value;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pagination(pub usize, pub usize);

impl Default for Pagination {
    fn default() -> Self {
        Self(5, 0)
    }
}

pub type VecOfMaps = Vec<BTreeMap<String, Value>>;