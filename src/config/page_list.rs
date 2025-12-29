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
pub enum ListItem {
    Page(PagePath),
    List(PageList),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PageList {
    pub items: IndexMap<String, ListItem>,
}

impl PageList {
    pub fn first_page_path(&self) -> Option<PagePath> {
        for item in self.items.values() {
            match item {
                ListItem::Page(path) => {
                    return Some(path.clone());
                }
                ListItem::List(submenu) => {
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
                ListItem::Page(path) => {
                    if !project.pages.contains_key(path) {
                        let md_file_path = path.md_path_buf(&project.src_path);
                        let page = Page::new(title.clone(), path.clone(), md_file_path);
                        project.pages.insert(path.clone(), page);
                    }
                }
                ListItem::List(submenu) => {
                    submenu.add_pages(project);
                }
            }
        }
    }
    pub fn add_page_paths<'m>(
        &'m self,
        list: &mut Vec<&'m PagePath>,
    ) {
        for item in self.items.values() {
            match item {
                ListItem::Page(path) => {
                    if !list.contains(&path) {
                        list.push(path);
                    }
                }
                ListItem::List(submenu) => {
                    submenu.add_page_paths(list);
                }
            }
        }
    }
    pub fn previous(
        &self,
        current_page: &PagePath,
    ) -> Option<&PagePath> {
        let mut page_paths = Vec::new();
        self.add_page_paths(&mut page_paths);
        for (i, path) in page_paths.iter().enumerate() {
            if path == &current_page {
                if i > 0 {
                    return Some(page_paths[i - 1]);
                } else {
                    return None;
                }
            }
        }
        None
    }
    pub fn next(
        &self,
        current_page: &PagePath,
    ) -> Option<&PagePath> {
        let mut page_paths = Vec::new();
        self.add_page_paths(&mut page_paths);
        for (i, path) in page_paths.iter().enumerate() {
            if path == &current_page {
                if i + 1 < page_paths.len() {
                    return Some(page_paths[i + 1]);
                } else {
                    return None;
                }
            }
        }
        None
    }
    pub fn push_nav(
        &self,
        html: &mut String,
        classes: &[ClassName],
        menu_insert: &Menu,
        hosting_page_path: &PagePath,
    ) -> DdResult<()> {
        writeln!(html, "<nav class=\"site-nav")?;
        for class in classes {
            write!(html, " {}", class.as_str())?;
        }
        writeln!(html, "\">")?;
        if menu_insert.hamburger_checkbox {
            html.push_str(
                "<input type=checkbox id=nav-toggle class=nav-toggle>\n\
                 <label for=nav-toggle class=nav-toggle-label>☰</label>\n",
            );
        }
        self.push_nav_item_html(html, hosting_page_path);
        html.push_str("</nav>\n");
        Ok(())
    }
    /// Generate the HTML for a menu or submenu hosted on a page.
    #[allow(clippy::only_used_in_recursion)]
    fn push_nav_item_html(
        &self,
        html: &mut String,
        hosting_page_path: &PagePath,
    ) {
        html.push_str("<ul class=\"nav-menu\">\n");
        for (title, item) in &self.items {
            let (link, selected) = match item {
                ListItem::Page(path) => {
                    (hosting_page_path.link_to(path), path == hosting_page_path)
                }
                ListItem::List(submenu) => {
                    let first_page_path = submenu.first_page_path();
                    let link = first_page_path
                        .as_ref()
                        .map(|p| hosting_page_path.link_to(p))
                        .unwrap_or_else(|| "#".to_string());
                    (link, false)
                }
            };
            let selected_class = if selected { "selected" } else { "not-selected" };
            let _ = writeln!(
                html,
                "<li class=\"nav-item {}\"><a href=\"{}\">{}</a>",
                selected_class, link, title,
            );
            if let ListItem::List(submenu) = item {
                submenu.push_nav_item_html(html, hosting_page_path);
            }
            html.push_str("</li>\n");
        }
        html.push_str("</ul>\n");
    }
}

impl ListItem {
    pub fn first_page_path(&self) -> Option<PagePath> {
        match self {
            ListItem::Page(path) => Some(path.clone()),
            ListItem::List(submenu) => submenu.first_page_path(),
        }
    }
}
