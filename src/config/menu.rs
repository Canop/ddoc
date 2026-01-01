use {
    crate::*,
    std::{
        fmt,
        str::FromStr,
    },
};

/// A placeholder to insert the menu at this position in the nav bar
#[derive(Debug, Clone, Copy)]
pub struct Menu {
    pub hamburger_checkbox: bool,
}

impl FromStr for Menu {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s != "menu" {
            return Err("Menu must be 'menu'");
        }
        Ok(Self {
            hamburger_checkbox: true,
        })
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            hamburger_checkbox: true,
        }
    }
}

impl From<Attributes> for Menu {
    fn from(map: Attributes) -> Self {
        let mut menu_insert = Menu::default();
        if let Some(v) = map
            .get("hamburger_checkbox")
            .or_else(|| map.get("hamburger-checkbox"))
        {
            if let Some(b) = v.as_bool() {
                menu_insert.hamburger_checkbox = b;
            }
        }
        menu_insert
    }
}

impl fmt::Display for Menu {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "menu")
    }
}
impl serde::Serialize for Menu {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for Menu {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
