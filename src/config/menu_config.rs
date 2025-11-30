use {
    crate::*,
    indexmap::IndexMap,
    serde::{
        Deserialize,
        Serialize,
    },
    std::fmt::Write,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum MenuItem {
    Page(PagePath),
    SubMenu(Menu),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Menu {
    pub items: IndexMap<String, MenuItem>,
}

impl Menu {
    pub fn first_page_path(&self) -> Option<PagePath> {
        for item in self.items.values() {
            match item {
                MenuItem::Page(path) => {
                    return Some(path.clone());
                }
                MenuItem::SubMenu(submenu) => {
                    if let Some(path) = submenu.first_page_path() {
                        return Some(path);
                    }
                }
            }
        }
        None
    }
    pub fn add_pages(
        &self,
        project: &mut Project,
    ) {
        for (title, item) in &self.items {
            match item {
                MenuItem::Page(path) => {
                    if !project.pages.contains_key(path) {
                        let md_file_path = path.md_path_buf(&project.src_path);
                        let page = Page::new(title.clone(), path.clone(), md_file_path);
                        project.pages.insert(path.clone(), page);
                    }
                }
                MenuItem::SubMenu(submenu) => {
                    submenu.add_pages(project);
                }
            }
        }
    }
    pub fn push_nav(
        &self,
        html: &mut String,
        project: &Project,
        hosting_page_path: &PagePath,
    ) {
        html.push_str("<nav class=site-nav>\n");
        if project.config.ui.hamburger_checkbox {
            html.push_str(
                "<input type=checkbox id=nav-toggle class=nav-toggle>\n\
                 <label for=nav-toggle class=nav-toggle-label>☰</label>\n",
            );
        }
        self.push_nav_item_html(html, project, hosting_page_path);
        html.push_str("</nav>\n");
    }
    /// Generate the HTML for a menu or submenu hosted on a page.
    #[allow(clippy::only_used_in_recursion)]
    fn push_nav_item_html(
        &self,
        html: &mut String,
        project: &Project,
        hosting_page_path: &PagePath,
    ) {
        html.push_str("<ul class=nav-menu>\n");
        for (title, item) in &self.items {
            let (link, selected) = match item {
                MenuItem::Page(path) => {
                    (hosting_page_path.link_to(path), path == hosting_page_path)
                }
                MenuItem::SubMenu(submenu) => {
                    let first_page_path = submenu.first_page_path();
                    let link = first_page_path
                        .as_ref()
                        .map(|p| hosting_page_path.link_to(p))
                        .unwrap_or("#".to_string());
                    (link, false)
                }
            };
            let selected_class = if selected { "selected" } else { "not-selected" };
            let _ = writeln!(
                html,
                "<li class=\"nav-item {}\"><a href=\"{}\">{}</a>",
                selected_class, link, title,
            );
            if let MenuItem::SubMenu(submenu) = item {
                submenu.push_nav_item_html(html, project, hosting_page_path);
            }
            html.push_str("</li>\n");
        }
        html.push_str("</ul>\n");
    }
}

impl MenuItem {
    pub fn first_page_path(&self) -> Option<PagePath> {
        match self {
            MenuItem::Page(path) => Some(path.clone()),
            MenuItem::SubMenu(submenu) => submenu.first_page_path(),
        }
    }
}
