mod composite_element;
mod element;
mod menu_config;
mod menu_insert;
mod nav_component;
mod nav_link;

pub use {
    composite_element::*,
    element::*,
    menu_config::*,
    menu_insert::*,
    nav_component::*,
    nav_link::*,
};

use {
    crate::*,
    serde::{
        Deserialize,
        Serialize,
    },
    std::path::{
        Path,
        PathBuf,
    },
};

pub static CONFIG_FILE_NAME: &str = "ddoc.hjson";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub title: String,
    pub description: Option<String>,
    pub repo_url: Option<String>,
    #[serde(alias = "pages")]
    pub menu: Menu,
    pub favicon: Option<String>,
    #[serde(default)]
    pub header: NavDir,
    #[serde(default)]
    pub footer: NavDir,
    #[serde(default)]
    pub ui: UiOptions,
}

impl Config {
    /// Read the ddoc.hjson configuration file at the root of a ddoc project
    ///
    /// Return both the config and the path where it was found
    ///
    /// # Errors
    /// Return `DdError::ConfigNotFound` if no ddoc.hjson is found at the specified path
    /// or other `DdError` variants on read/parse errors
    pub fn at_root(path: &Path) -> DdResult<(Self, PathBuf)> {
        let config_path = path.join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            return Err(DdError::ConfigNotFound);
        }
        let config: Config = read_file(&config_path)?;
        Ok((config, config_path))
    }
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref().filter(|s| !s.is_empty())
    }
    pub fn favicon(&self) -> Option<&str> {
        self.favicon.as_deref().filter(|s| !s.is_empty())
    }
    pub fn needs_search_script(&self) -> bool {
        self.header.has_href("--search")
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
