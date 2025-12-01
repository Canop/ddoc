mod menu_config;
mod menu_link_config;

pub use {
    menu_config::*,
    menu_link_config::*,
};

use {
    crate::*,
    serde::{
        Deserialize,
        Serialize,
    },
    std::path::Path,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub title: String,
    pub description: Option<String>,
    pub repo_url: Option<String>,
    #[serde(alias = "pages")]
    pub menu: Menu,
    pub favicon: Option<String>,
    #[serde(default)]
    pub nav_links: NavLinksConfig,
    #[serde(default)]
    pub ui: UiOptions,
}

impl Config {
    pub fn at_root(path: &Path) -> DdResult<Self> {
        let config_path = path.join("ddoc.hjson");
        if !config_path.exists() {
            return Err(DdError::ConfigNotFound);
        }
        read_file(config_path)
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref().filter(|s| !s.is_empty())
    }
    pub fn favicon(&self) -> Option<&str> {
        self.favicon.as_deref().filter(|s| !s.is_empty())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UiOptions {
    /// If true, a hamburger checkbox is added to the UI for toggling
    /// the menu on small screens.
    ///
    /// When it's true (default), CSS becomes necessary
    #[serde(default = "bool_true")]
    hamburger_checkbox: bool,
}

impl Default for UiOptions {
    fn default() -> Self {
        Self {
            hamburger_checkbox: true,
        }
    }
}

pub fn bool_true() -> bool {
    true
}
