use crate::*;

#[derive(Debug, Clone)]
pub struct Element {
    pub classes: Vec<ClassName>,
    pub content: ElementContent,
}

#[derive(Debug, Clone)]
pub enum ElementContent {
    Html { tag: String, children: Vec<Element> },
    Link(NavLink),
    Menu(Menu),
    Toc,
    Main,
}

impl Element {
    /// The psudo-tag of this element, as used in configuration
    pub fn tag(&self) -> &str {
        match &self.content {
            ElementContent::Html { tag, .. } => tag,
            ElementContent::Link(_) => "ddoc-link",
            ElementContent::Menu(_) => "ddoc-menu",
            ElementContent::Toc => "ddoc-toc",
            ElementContent::Main => "ddoc-main",
        }
    }
    pub fn is_html(&self) -> bool {
        matches!(self.content, ElementContent::Html { .. })
    }
    pub fn is_link(&self) -> bool {
        matches!(self.content, ElementContent::Link(_))
    }
    pub fn is_menu(&self) -> bool {
        matches!(self.content, ElementContent::Menu(_))
    }
    pub fn is_toc(&self) -> bool {
        matches!(self.content, ElementContent::Toc)
    }
    pub fn is_main(&self) -> bool {
        matches!(self.content, ElementContent::Main)
    }
    pub fn children(&self) -> Option<&Vec<Element>> {
        match &self.content {
            ElementContent::Html { children, .. } => Some(children),
            _ => None,
        }
    }
    pub fn new_composite(
        key: &str, // tag.class1.class2
        children: Vec<Element>,
    ) -> Self {
        let mut tokens = key.split('.');
        let tag = tokens.next().unwrap_or("div").to_string();
        let classes = tokens.map(|s| s.to_string()).collect();
        Self {
            classes,
            content: ElementContent::Html { tag, children },
        }
    }
    pub fn visit<F>(
        &self,
        f: &mut F,
    ) where
        F: FnMut(&Element),
    {
        f(self);
        if let Some(children) = self.children() {
            for child in children {
                child.visit(f);
            }
        }
    }
}

impl From<ElementContent> for Element {
    fn from(content: ElementContent) -> Self {
        Self {
            classes: vec![],
            content,
        }
    }
}
