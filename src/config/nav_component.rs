use {
    crate::*,
    indexmap::IndexMap,
    serde::{
        Deserialize,
        Serialize,
    },
};

/// Deprecated navigation components structure for compatibility with ddoc <= 0.10
#[derive(Debug, Clone, Deserialize)]
pub struct NavComponents {
    #[serde(default)]
    header: NavDir,
    #[serde(default)]
    footer: NavDir,
    #[serde(default)]
    ui: UiOptions,
}

/// Either the header or footer navigation configuration for ddoc <= 0.10
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(transparent)]
struct NavDir {
    components: IndexMap<ClassName, NavComponent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum NavComponent {
    Menu(Menu),
    NavLinks(Vec<NavLink>),
}

impl NavComponents {
    /// Build a CompositeElement from old-style NavComponents, adding the
    /// parts which were implicit in ddoc <= 0.10
    pub fn into_body_composite(&self) -> CompositeElement {
        let mut children = Vec::new();
        if !self.header.is_empty() {
            children.push(Element::new_composite(
                "header",
                self.header.to_children(&self.ui),
            ));
        }
        children.push(Element::new_composite(
            "article",
            vec![
                Element::new_composite("aside.page-nav", vec![ElementContent::Toc.into()]),
                ElementContent::Main.into(),
            ],
        ));
        if !self.footer.is_empty() {
            children.push(Element::new_composite(
                "footer",
                self.footer.to_children(&self.ui),
            ));
        }
        CompositeElement { children }
    }
}
impl NavDir {
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
    fn to_children(
        &self,
        ui: &UiOptions,
    ) -> Vec<Element> {
        let mut children = Vec::new();
        for (class, comp) in &self.components {
            match comp {
                NavComponent::Menu(_) => {
                    let menu_insert = Menu {
                        hamburger_checkbox: ui.hamburger_checkbox,
                    };
                    children.push(Element {
                        classes: vec![class.clone()],
                        content: ElementContent::Menu(menu_insert),
                    });
                }
                NavComponent::NavLinks(links) => {
                    let mut nav_children = Vec::new();
                    for link in links {
                        nav_children.push(ElementContent::Link(link.clone()).into());
                    }
                    let mut nav = Element::new_composite("nav", nav_children);
                    nav.classes.push(class.clone());
                    children.push(nav);
                }
            }
        }
        children
    }
}

#[test]
fn test_nav_component_deserialize() {
    #[derive(Debug, Clone, Deserialize, Serialize)]
    struct TestConfig {
        header: NavDir,
        footer: NavDir,
    }
    let hjson = r#"
        header: {
            before-menu: [
                {
                    img: img/dystroy-rust-white.svg
                    href: https://dystroy.org
                    alt: dystroy.org homepage
                    class: external-nav-link
                }
                {
                    url: /index.md
                    alt: ddoc homepage
                    label: ddoc
                    class: home-link
                }
            ]
            middle: menu
            after-menu: [
                {
                    img: img/ddoc-left-arrow.svg
                    href: --previous
                    class: previous-page-link
                    alt: Previous Page
                }
                {
                    img: img/ddoc-search.svg
                    href: --search
                    class: search-opener
                    alt: Search
                }
                {
                    img: img/ddoc-right-arrow.svg
                    url: --next
                    class: next-page-link
                    alt: Next Page
                }
                {
                    img: img/github-mark-white.svg
                    class: external-nav-link
                    alt: GitHub
                    href: https://github.com/Canop/ddoc
                }
            ]
        }
        footer: {
            right: [
                {
                    label: made with **ddoc**
                    href: https://dystroy.org/ddoc
                }
            ]
        }
    "#;
    let test_config: TestConfig = deser_hjson::from_str(hjson).unwrap();
    assert_eq!(test_config.header.components.len(), 3);
    assert_eq!(test_config.footer.components.len(), 1);
    assert!(matches!(
        test_config.header.components.get("middle").unwrap(),
        NavComponent::Menu(_)
    ));
    let NavComponent::NavLinks(links) = test_config.header.components.get("after-menu").unwrap()
    else {
        panic!("Expected NavLinks");
    };
    assert_eq!(links[2].href.as_deref(), Some("--next"));
}
