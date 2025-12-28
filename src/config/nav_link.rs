use {
    crate::*,
    indexmap::IndexMap,
    serde::{
        Deserialize,
        Serialize,
    },
};

/// A single link in the navigation bar
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NavLink {
    pub img: Option<String>,
    pub label: Option<String>,
    pub alt: Option<String>,
    #[serde(alias = "url")]
    pub href: Option<String>,
    pub class: Option<String>,
    pub target: Option<String>,
}

impl From<Attributes> for NavLink {
    fn from(map: IndexMap<AttributeKey, AttributeValue>) -> Self {
        let img = map
            .get("img")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let label = map
            .get("label")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let alt = map
            .get("alt")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let href = map
            .get("href")
            .or(map.get("link_target"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let class = map
            .get("class")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let target = map
            .get("target")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        Self {
            img,
            label,
            alt,
            href,
            class,
            target,
        }
    }
}
