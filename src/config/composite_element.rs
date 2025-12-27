use {
    crate::*,
    indexmap::IndexMap,
    serde::{
        Deserialize,
        de,
    },
    std::fmt,
};

#[derive(Debug, Clone)]
pub struct CompositeElement {
    pub entries: Vec<CompositeElementEntry>,
}
#[derive(Debug, Clone)]
pub struct CompositeElementEntry {
    pub key: ElementKey,
    pub value: Element,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DeserElement {
    Composite(CompositeElement),
    Attributes(IndexMap<AttributeKey, AttributeValue>),
}

impl CompositeElement {
    pub fn get_by_key(
        &self,
        key: &str,
    ) -> Option<&Element> {
        for entry in &self.entries {
            if entry.key == key {
                return Some(&entry.value);
            }
        }
        None
    }
    pub fn get(
        &self,
        index: usize,
    ) -> Option<&CompositeElementEntry> {
        self.entries.get(index)
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

pub struct CompositeElementDeserializer {}
impl<'de> de::Visitor<'de> for CompositeElementDeserializer {
    type Value = CompositeElement;

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
        let mut entries = Vec::new();
        while let Some((key, value)) = access.next_entry::<ElementKey, Element>()? {
            entries.push(CompositeElementEntry { key, value });
        }
        Ok(Self::Value { entries })
    }
}
impl<'de> de::Deserialize<'de> for CompositeElement {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_map(CompositeElementDeserializer {})
    }
}

#[test]
fn test_element_deserialization() {
    let hjson = r#"
    {
        header: {
            div.before-menu: {
                link: {
                    img: img/dystroy-rust-white.svg
                    href: https://dystroy.org
                    alt: dystroy.org homepage
                    class: external-nav-link
                }
                link: {
                    url: /index.md
                    alt: ddoc homepage
                    label: ddoc
                    class: home-link
                }
            }
            menu: {
                hamburger-checkbox: true
            }
            div.after-menu: {
                link: {
                    img: img/ddoc-left-arrow.svg
                    href: --previous
                    class: previous-page-link
                    alt: Previous Page
                }
                link: {
                    img: img/ddoc-search.svg
                    href: --search
                    class: search-opener
                    alt: Search
                }
                link: {
                    img: img/ddoc-right-arrow.svg
                    href: --next
                    class: next-page-link
                    alt: Next Page
                }
                link: {
                    img: img/github-mark-white.svg
                    class: external-nav-link
                    alt: GitHub
                    href: https://github.com/Canop/ddoc
                }
            }
        }
    }
    "#;
    let element: Element = deser_hjson::from_str(hjson).unwrap();
    let header = element
        .as_composite()
        .unwrap()
        .get(0)
        .unwrap()
        .value
        .as_composite()
        .unwrap();
    assert_eq!(header.len(), 3);
    //assert_eq!(
    //    element.as_content().unwrap()
    //    .get("div.after-menu").unwrap()
    //    .as_content().unwrap()
    //    .get("link").unwrap()
    //    .as_attributes().unwrap()
    //    .get("href
}
