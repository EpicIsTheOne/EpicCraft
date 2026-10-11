//! A cached, repository-owned release index layered over upstream builds.

use std::collections::BTreeMap;

use artcraft_launcher_core::epiccraft::{INDEX_URL, parse_index};
use artcraft_launcher_core::release::Release;

use crate::http::Http;
use crate::paths::{Paths, write_atomic};

pub fn releases(http: &Http, paths: &Paths) -> BTreeMap<String, Release> {
    let cache = paths.cache.join("epiccraft-releases.json");
    match http.get_feed(INDEX_URL) {
        Ok(text) => {
            if let Some(index) = parse_index(&text) {
                if let Err(error) = write_atomic(&cache, text.as_bytes()) {
                    log::warn!("could not cache EpicCraft releases: {error}");
                }
                return index;
            }
            log::warn!("EpicCraft release index is malformed; using the last valid index");
        }
        Err(error) => log::warn!("EpicCraft release index: {}", error.why),
    }
    cached(paths)
}

pub fn cached(paths: &Paths) -> BTreeMap<String, Release> {
    std::fs::read_to_string(paths.cache.join("epiccraft-releases.json")).ok().and_then(|text| parse_index(&text)).unwrap_or_default()
}
