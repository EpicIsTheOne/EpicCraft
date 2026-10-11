//! EpicCraft releases overlay the official catalog only for apps we have published.

use std::collections::BTreeMap;

use crate::catalog::{CraftApp, LAUNCHER_ID, SUITE};
use crate::release::{Release, parse_github_release};

pub const REPO: &str = "EpicIsTheOne/EpicCraft";
pub const INDEX_URL: &str = "https://raw.githubusercontent.com/EpicIsTheOne/EpicCraft/main/epiccraft/releases.json";

/// The index contains GitHub release API objects keyed by app id. Unknown apps,
/// drafts, malformed versions and releases without matching assets are ignored.
pub fn parse_index(text: &str) -> Option<BTreeMap<String, Release>> {
    let entries: BTreeMap<String, serde_json::Value> = serde_json::from_str(text).ok()?;
    let mut releases = BTreeMap::new();
    for (id, value) in entries {
        let app = if id == LAUNCHER_ID {
            &crate::catalog::LAUNCHER
        } else if let Some(app) = SUITE.iter().find(|app| app.id == id) {
            app
        } else {
            continue;
        };
        if value.get("draft").and_then(serde_json::Value::as_bool) != Some(false) {
            continue;
        }
        let Ok(mut release) = parse_github_release(&value.to_string(), REPO) else { continue };
        release.version = release.version.or_else(|| release.tag.strip_prefix(&format!("{id}-v")).and_then(crate::version::parse));
        if release.version.is_none() {
            continue;
        }
        release.assets.retain(|asset| (matches_app(app, &asset.name) && asset.sha256.is_some()) || asset.name == "SHA256SUMS.txt");
        if !release.assets.iter().any(|asset| matches_app(app, &asset.name)) {
            continue;
        }
        releases.insert(id, release);
    }
    Some(releases)
}

fn matches_app(app: &CraftApp, name: &str) -> bool {
    app.stems().any(|stem| name.starts_with(&format!("{stem}-")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn index(url: &str, name: &str) -> String {
        serde_json::json!({"filmcraft": {"draft": false, "tag_name": "filmcraft-v0.99.0",
            "assets": [{"name": name, "browser_download_url": url, "size": 100,
                "digest": format!("sha256:{}", "a".repeat(64))}]}})
        .to_string()
    }

    #[test]
    fn only_our_repo_and_matching_app_assets_are_accepted() {
        let url = "https://github.com/EpicIsTheOne/EpicCraft/releases/download/filmcraft-v0.99.0/filmcraft-0.99.0-windows-x64.msi";
        assert!(parse_index(&index(url, "filmcraft-0.99.0-windows-x64.msi")).unwrap().contains_key("filmcraft"));
        assert!(parse_index(&index(&url.replace("EpicIsTheOne/EpicCraft", "someone/other"), "filmcraft-0.99.0-windows-x64.msi")).unwrap().is_empty());
        assert!(parse_index(&index(url, "photocraft-0.99.0-windows-x64.msi")).unwrap().is_empty());
    }

    #[test]
    fn malformed_index_and_drafts_do_not_replace_official_releases() {
        assert!(parse_index("broken").is_none());
        assert!(parse_index("{}").unwrap().is_empty());
        let draft = index("https://github.com/EpicIsTheOne/EpicCraft/releases/download/v1/filmcraft-1.msi", "filmcraft-1.msi")
            .replace("\"draft\":false", "\"draft\":true");
        assert!(parse_index(&draft).unwrap().is_empty());
    }
}
