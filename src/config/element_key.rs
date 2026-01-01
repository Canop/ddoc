use {
    crate::*,
    std::{
        fmt,
        str::FromStr,
    },
};

#[derive(Debug, Clone)]
pub struct ElementKey {
    pub etype: ElementType,
    pub classes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElementType {
    HtmlTag(String),
    Menu,
    Link,
    Toc,
    Main,
    PageTitle,
}

impl fmt::Display for ElementType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            ElementType::HtmlTag(tag) => write!(f, "{}", tag),
            ElementType::Menu => write!(f, "ddoc-menu"),
            ElementType::Link => write!(f, "ddoc-link"),
            ElementType::Toc => write!(f, "ddoc-toc"),
            ElementType::Main => write!(f, "ddoc-main"),
            ElementType::PageTitle => write!(f, "ddoc-page-title"),
        }
    }
}
impl fmt::Display for ElementKey {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}", &self.etype)?;
        for class in &self.classes {
            write!(f, ".{}", class)?;
        }
        Ok(())
    }
}

impl FromStr for ElementKey {
    type Err = DdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // FIXME check validity of tag/class names
        let parts: Vec<&str> = s.split('.').collect();
        let etype = match parts[0] {
            "ddoc-menu" => ElementType::Menu,
            "ddoc-link" => ElementType::Link,
            "ddoc-toc" => ElementType::Toc,
            "ddoc-main" => ElementType::Main,
            "ddoc-page-title" => ElementType::PageTitle,
            tag => ElementType::HtmlTag(tag.to_string()),
        };
        let classes = parts[1..].iter().map(|s| s.to_string()).collect();
        Ok(ElementKey { etype, classes })
    }
}

impl<'de> serde::Deserialize<'de> for ElementKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
