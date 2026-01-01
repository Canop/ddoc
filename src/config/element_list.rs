use {
    crate::*,
    serde::{
        Deserialize,
        de,
    },
    std::fmt,
    termimad::crossterm::style::Stylize,
};

/// A collection of elements
#[derive(Debug, Clone, Default)]
pub struct ElementList {
    pub children: Vec<Element>,
}

impl ElementList {
    pub fn child(
        &self,
        index: usize,
    ) -> Option<&Element> {
        self.children.get(index)
    }
    pub fn len(&self) -> usize {
        self.children.len()
    }
    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
    pub fn visit<F>(
        &self,
        mut f: F,
    ) where
        F: FnMut(&Element),
    {
        for child in &self.children {
            child.visit(&mut f);
        }
    }
    pub fn has_href(
        &self,
        href: &str,
    ) -> bool {
        let mut found = false;
        self.visit(|element: &Element| {
            if let ElementContent::Link(link) = &element.content {
                if link.href.as_deref() == Some(href) {
                    found = true;
                }
            }
        });
        found
    }
}

pub struct ElementListDeserializer {}
impl<'de> de::Visitor<'de> for ElementListDeserializer {
    type Value = ElementList;

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
        #[derive(Debug, Clone, Deserialize)]
        #[serde(untagged)]
        enum DeserContent {
            Composite(ElementList),
            Attributes(Attributes),
        }
        let mut children = Vec::new();
        while let Some((key, value)) = access.next_entry::<ElementKey, DeserContent>()? {
            let ElementKey { etype, classes: _ } = key;
            let content = match (etype, value) {
                (ElementType::HtmlTag(tag), DeserContent::Composite(comp)) => {
                    ElementContent::Html {
                        tag,
                        children: comp.children,
                    }
                }
                (ElementType::Link, DeserContent::Attributes(attrs)) => {
                    let nav_link: NavLink = attrs.into();
                    ElementContent::Link(nav_link)
                }
                (ElementType::Menu, DeserContent::Attributes(attrs)) => {
                    let menu_insert: Menu = attrs.into();
                    ElementContent::Menu(menu_insert)
                }
                (ElementType::Menu, _) => ElementContent::Menu(Menu::default()),
                (ElementType::Toc, _) => ElementContent::Toc,
                (ElementType::Main, _) => ElementContent::Main,
                (ElementType::PageTitle, _) => ElementContent::PageTitle,
                (etype, value) => {
                    eprintln!(
                        "{}: invalid element type {} for value {:?}",
                        "error".red(),
                        etype.to_string().yellow(),
                        value,
                    );
                    return Err(de::Error::custom(format!(
                        "invalid element type {:?} for value {:?}",
                        etype, value
                    )));
                }
            };
            children.push(Element {
                classes: key.classes,
                content,
            });
        }
        Ok(Self::Value { children })
    }
}
impl<'de> de::Deserialize<'de> for ElementList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(ElementListDeserializer {})
    }
}

#[test]
fn test_composite_element_deserialization() {
    let hjson = r#"
    {
        header: {
            nav.before-menu: {
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
            nav.after-menu: {
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
            nav.made-with-ddoc: {
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
    let composite: ElementList = deser_hjson::from_str(hjson).unwrap();
    assert_eq!(composite.children.len(), 3);
    let header = &composite.children[0];
    assert!(matches!(
        header.children().unwrap()[1].content,
        ElementContent::Menu(_)
    ));
    let article = &composite.children[1];
    let toc = &article.children().unwrap()[0].children().unwrap()[0];
    assert!(toc.is_toc());
    let footer = &composite.children[2];
    assert_eq!(footer.tag(), "footer");
    assert!(composite.has_href("--next"));
    assert!(!composite.has_href("--top"));
}
