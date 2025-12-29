mod attribute;
mod composite_element;
mod element;
mod element_key;
mod menu;
mod nav_component;
mod nav_link;
mod page_list;

pub use {
    attribute::*,
    composite_element::*,
    element::*,
    element_key::*,
    menu::*,
    nav_component::*,
    nav_link::*,
    page_list::*,
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

pub type ClassName = String;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub title: String,
    pub description: Option<String>,
    #[serde(alias = "pages", alias = "menu")]
    pub site_map: PageList,
    pub favicon: Option<String>,
    /// for compatibility with ddoc (0.10-), this is loaded but only used
    /// through conversion to the new `body` field
    #[serde(flatten)]
    old: NavComponents,
    #[serde(default)]
    pub body: CompositeElement,
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
        self.body.has_href("--search")
    }
    /// For support of old ddoc versions (<= 0.10), convert old nav components
    /// if the new `body` field is empty
    pub fn fix_old(&mut self) {
        if self.body.children.is_empty() {
            self.body = self.old.into_body_composite();
        }
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
