use {
    crate::*,
    serde::{
        Deserialize,
        de,
    },
    std::fmt,
};

#[derive(Debug, Clone, Default)]
pub struct V2Composite {
    pub children: Vec<V2Element>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum DeserContent {
    Composite(V2Composite),
    Attributes(Attributes),
}

#[derive(Debug, Clone)]
pub enum V2ElementContent {
    Html {
        tag: String,
        children: Vec<V2Element>,
    },
    Link(NavLink),
    Menu(MenuInsert),
    Toc,
    Main,
}

#[derive(Debug, Clone)]
pub struct V2Element {
    pub classes: Vec<String>,
    pub content: V2ElementContent,
}

impl V2Element {
    pub fn is_html(&self) -> bool {
        matches!(self.content, V2ElementContent::Html { .. })
    }
    pub fn is_link(&self) -> bool {
        matches!(self.content, V2ElementContent::Link(_))
    }
    pub fn is_menu(&self) -> bool {
        matches!(self.content, V2ElementContent::Menu(_))
    }
    pub fn is_toc(&self) -> bool {
        matches!(self.content, V2ElementContent::Toc)
    }
    pub fn is_main(&self) -> bool {
        matches!(self.content, V2ElementContent::Main)
    }
    pub fn children(&self) -> Option<&Vec<V2Element>> {
        match &self.content {
            V2ElementContent::Html { children, .. } => Some(children),
            _ => None,
        }
    }
}

pub struct V2CompositeDeserializer {}
impl<'de> de::Visitor<'de> for V2CompositeDeserializer {
    type Value = V2Composite;

    fn expecting(
        &self,
        formatter: &mut fmt::Formatter,
    ) -> fmt::Result {
        formatter.write_str("a composite element")
    }
    fn visit_map<M>(
        self,
        mut access: M,
    ) -> Result<Self::Value, M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        let mut children = Vec::new();
        while let Some((key, value)) = access.next_entry::<ElementKey, DeserContent>()? {
            let ElementKey { etype, classes: _ } = key;
            let content = match (etype, value) {
                (ElementType::HtmlTag(tag), DeserContent::Composite(comp)) => {
                    V2ElementContent::Html {
                        tag,
                        children: comp.children,
                    }
                }
                (ElementType::Link, DeserContent::Attributes(attrs)) => {
                    let nav_link: NavLink = attrs.into();
                    V2ElementContent::Link(nav_link)
                }
                (ElementType::Menu, DeserContent::Attributes(attrs)) => {
                    let menu_insert: MenuInsert = attrs.into();
                    V2ElementContent::Menu(menu_insert)
                }
                (ElementType::Menu, _) => V2ElementContent::Menu(MenuInsert::default()),
                (ElementType::Toc, _) => V2ElementContent::Toc,
                (ElementType::Main, _) => V2ElementContent::Main,
                (etype, value) => {
                    eprintln!("etype: {:?}, value: {:?}", etype, value);
                    return Err(de::Error::custom(format!(
                        "invalid element type {:?} for value {:?}",
                        etype, value
                    )));
                }
            };
            children.push(V2Element {
                classes: key.classes,
                content,
            });
        }
        Ok(Self::Value { children })
    }
}
impl<'de> de::Deserialize<'de> for V2Composite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(V2CompositeDeserializer {})
    }
}

#[test]
fn test_v2element_deserialization() {
    let hjson = r#"
    {
        header: {
            div.before-menu: {
                ddoc-link: {
                    img: img/dystroy-rust-white.svg
                    href: https://dystroy.org
                    alt: dystroy.org homepage
                    class: external-nav-link
                }
                ddoc-link: {
                    url: /index.md
                    alt: ddoc homepage
                    label: ddoc
                    class: home-link
                }
            }
            ddoc-menu: {
                hamburger-checkbox: true
            }
            div.after-menu: {
                ddoc-link: {
                    img: img/ddoc-left-arrow.svg
                    href: --previous
                    class: previous-page-link
                    alt: Previous Page
                }
                ddoc-link: {
                    img: img/ddoc-search.svg
                    href: --search
                    class: search-opener
                    alt: Search
                }
                ddoc-link: {
                    img: img/ddoc-right-arrow.svg
                    href: --next
                    class: next-page-link
                    alt: Next Page
                }
                ddoc-link: {
                    img: img/github-mark-white.svg
                    class: external-nav-link
                    alt: GitHub
                    href: https://github.com/Canop/ddoc
                }
            }
        }
        article: {
            aside.page-nav: {
                ddoc-toc: {}
            }
            ddoc-main: {}
        }
        footer: {
            div.made-with-ddoc: {
                ddoc-link: {
                    label: made with
                }
                ddoc-link: {
                    label: ddoc
                    href: https://dystroy.org/ddoc
                    class: link-to-ddoc
                }
            }
        }
    }
    "#;
    let composite: V2Composite = deser_hjson::from_str(hjson).unwrap();
    assert_eq!(composite.children.len(), 3);
    let header = &composite.children[0];
    assert!(matches!(
        header.children().unwrap()[1].content,
        V2ElementContent::Menu(_)
    ));
}
