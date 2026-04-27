use {
    crate::*,
    lazy_regex::regex_remove,
    pulldown_cmark::{
        self as pcm,
        CowStr,
        Event,
        Parser,
        Tag,
        TagEnd,
        html::push_html,
    },
    rustc_hash::FxHashMap,
    std::fmt::Write,
    termimad::crossterm::style::Stylize,
};

pub struct PageWriter<'p> {
    page: &'p Page,
    project: &'p Project,
    /// What goes inside the `<ul class=toc-content>` tag
    toc: String,
    /// What goes inside the `<main>` tag
    main: String,
}

impl<'p> PageWriter<'p> {
    pub fn new(
        page: &'p Page,
        project: &'p Project,
        md: &str,
    ) -> DdResult<Self> {
        let mut id_counts = FxHashMap::default();
        let mut toc = String::new(); // stores the LIs of the nav.page-toc
        let mut main = String::new(); // stores the HTML of the <main> tag

        let mut events = Parser::new_ext(md, pcm::Options::all()).collect::<Vec<_>>();
        for i in 0..events.len() {
            match &mut events[i] {
                // Rewrite the image source
                Event::Start(Tag::Image { dest_url, .. }) => {
                    *dest_url = CowStr::from(project.img_url(dest_url, &page.page_path));
                }

                // rewrite internal links
                Event::Start(Tag::Link { dest_url, .. }) => {
                    if let Some(new_url) = project.rewrite_link_url(dest_url, &page.page_path) {
                        *dest_url = CowStr::from(new_url);
                    }
                }

                _ => {}
            }

            // Generate IDs for headings if missing and
            // generate the TOC's content
            if let Event::Start(Tag::Heading { level, id, .. }) = &events[i] {
                if id.is_none() {
                    // Generate an ID from the heading text
                    let mut heading_text = String::new();
                    let mut j = i + 1;
                    while j < events.len() {
                        match &events[j] {
                            Event::Code(code) => {
                                if !heading_text.is_empty() {
                                    heading_text.push(' ');
                                }
                                heading_text.push_str(code);
                            }
                            Event::Text(text) => {
                                if !heading_text.is_empty() {
                                    heading_text.push(' ');
                                }
                                heading_text.push_str(text);
                            }
                            Event::End(TagEnd::Heading(_)) => {
                                break;
                            }
                            _ => {}
                        }
                        j += 1;
                    }
                    let mut new_id = heading_text
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == ' ')
                        .map(|c| if c == ' ' { '-' } else { c })
                        .collect::<String>();
                    let count = id_counts.entry(new_id.clone()).or_insert(0);
                    *count += 1;
                    if *count > 1 {
                        new_id = format!("{}-{}", new_id, count);
                    }
                    writeln!(
                        toc,
                        "<li class=\"toc-item {level}\"><a href=#{new_id}>{heading_text}</a></li>"
                    )?;
                    if let Event::Start(Tag::Heading { id, .. }) = &mut events[i] {
                        *id = Some(CowStr::from(new_id));
                    } else {
                        unreachable!();
                    }
                }
            }
        }

        push_html(&mut main, events.into_iter());
        Ok(Self {
            page,
            project,
            toc,
            main,
        })
    }

    pub fn page_path(&self) -> &PagePath {
        &self.page.page_path
    }
    pub fn config(&self) -> &Config {
        &self.project.config
    }

    /// Write the full HTML for this page into the given `html` String
    ///
    /// # Errors
    /// Return `DdError` variants on write errors, not on project config/data errors
    pub fn write_html(
        &self,
        html: &mut String,
    ) -> DdResult<()> {
        self.write_html_head(html)?;
        write!(html, "<body class=\"page-{}", &self.page_path().stem)?;
        for name in self.project.plugin_names() {
            write!(html, " plugin-{name}")?;
        }
        writeln!(html, "\">\n")?;
        self.write_element(html, &self.config().body)?;
        html.push_str("</html>\n");
        Ok(())
    }

    /// Write the variable parts of the HTML `<head>`: the links to CSS, JS, meta tags, title, etc.
    ///
    /// # Errors
    /// Return `DdError` variants on write errors, not on project config/data errors
    pub fn write_html_head(
        &self,
        html: &mut String,
    ) -> DdResult<()> {
        html.push_str(HTML_START);
        let title = format!("{} - {}", &self.page.title, &self.config().title());
        writeln!(html, "<title>{}</title>", escape_text(&title))?;
        writeln!(
            html,
            "<meta name=\"og-title\" content=\"{}\">",
            escape_attr(&title)
        )?;
        if let Some(description) = self.config().description() {
            let description = escape_attr(description);
            writeln!(html, r#"<meta name="description" content="{description}">"#)?;
            writeln!(
                html,
                r#"<meta name="og-description" content="{description}">"#
            )?;
        }
        if let Some(url) = self.config().favicon() {
            let url = self.project.img_url(url, self.page_path());
            writeln!(html, r#"<link rel="shortcut icon" href="{url}">"#)?;
        }
        for e in self.project.list_js()? {
            let url = self.project.static_url(&e.served_path, self.page_path());
            writeln!(html, r#"<script src="{}?m={}"></script>"#, url, e.mtime)?;
        }
        before_0_16::write_special_js_headers_if_needed(
            self.page_path(),
            self.config(),
            self.project,
            html,
        )?;
        for e in self.project.list_css()?.into_iter().rev() {
            let url = self.project.static_url(&e.served_path, self.page_path());
            writeln!(
                html,
                r#"<link href="{}?m={}" rel=stylesheet>"#,
                url, e.mtime
            )?;
        }
        html.push_str("</head>\n");
        Ok(())
    }

    fn write_page_title(
        &self,
        html: &mut String,
        page: &Page,
    ) {
        html.push_str(&escape_text(&page.title));
    }

    /// Write the text for a `Text` item, which can be a string or a dynamic
    /// value like the current page title
    ///
    /// Return `false` in case of missing expansion (the container should probably
    /// not be rendered)
    fn write_text(
        &self,
        html: &mut String,
        text: &Text,
    ) -> bool {
        match text {
            Text::String(s) => html.push_str(&escape_text(s)),
            Text::PreviousPageTitle => {
                if let Some(prev_page) = self.project.previous_page(self.page_path()) {
                    self.write_page_title(html, prev_page)
                }
            }
            Text::CurrentPageTitle => self.write_page_title(html, self.page),
            Text::NextPageTitle => {
                if let Some(next_page) = self.project.next_page(self.page_path()) {
                    self.write_page_title(html, next_page)
                }
            }
            Text::Var(var_name) => {
                let Some(var_value) = self.config().var(var_name) else {
                    return false;
                };
                html.push_str(var_value.as_str());
            }
        }
        true
    }

    fn write_opening_tag(
        &self,
        html: &mut String,
        tag: &str,
        classes: &[String],
    ) {
        html.push('<');
        html.push_str(tag);
        if !classes.is_empty() {
            html.push_str(" class=\"");
            for (i, class) in classes.iter().enumerate() {
                if i > 0 {
                    html.push(' ');
                }
                html.push_str(class);
            }
            html.push('"');
        }
    }
    fn write_closing_tag(
        &self,
        html: &mut String,
        tag: &str,
    ) {
        html.push_str("</");
        html.push_str(tag);
        html.push_str(">\n");
    }

    fn write_element(
        &self,
        html: &mut String,
        element: &Element,
    ) -> DdResult<()> {
        match &element.content {
            ElementContent::DomLeaf {
                tag,
                text,
                raw_html,
                attributes,
            } => {
                self.write_opening_tag(html, tag, &element.classes);
                for (name, value) in attributes {
                    if let Some(value) = value.as_str() {
                        let value = escape_attr(value);
                        write!(html, r#" {name}="{value}""#)?;
                    }
                }
                html.push_str(">\n");
                if let Some(text) = text {
                    self.write_text(html, text);
                }
                if let Some(raw_html) = raw_html {
                    html.push_str(raw_html);
                }
                self.write_closing_tag(html, tag);
            }
            ElementContent::DomTree { tag, children } => {
                self.write_opening_tag(html, tag, &element.classes);
                html.push_str(">\n");
                for child in children {
                    self.write_element(html, child)?;
                }
                self.write_closing_tag(html, tag);
            }
            ElementContent::Link(link) => {
                self.write_nav_link(html, &element.classes, link)?;
            }
            ElementContent::Menu(menu_insert) => {
                self.config().site_map.push_nav(
                    html,
                    &element.classes,
                    menu_insert,
                    self.page_path(),
                )?;
            }
            ElementContent::Toc(toc) => {
                html.push_str("<nav class=\"page-toc");
                if self.toc.is_empty() {
                    html.push_str(" empty");
                }
                html.push_str("\">\n");
                html.push_str("<a class=toc-title href=\"#top\">");
                let title = toc.title.as_ref().unwrap_or(&self.page.title);
                let title = escape_text(title);
                html.push_str(&title);
                html.push_str("</a>\n");
                if !self.toc.is_empty() {
                    html.push_str("<ul class=toc-content>");
                    html.push_str(&self.toc);
                    html.push_str("</ul>");
                }
                html.push_str("</nav>\n");
            }
            ElementContent::PageTitle => {
                // Here we don't wrap in any tag, just output the title text
                // This has the downside of ignoring any classes specified on the element
                // Maybe wrap in a div when classes are specified?
                // Or allow a tag as atribute to PageTitle element?
                html.push_str(&escape_text(&self.page.title));
            }
            ElementContent::Main => {
                html.push_str("<main>\n"); // fixme add classes?
                html.push_str(&self.main);
                html.push_str("\n</main>\n");
            }
        }
        Ok(())
    }

    fn write_nav_link(
        &self,
        dest_html: &mut String,
        classes: &[String],
        link: &NavLink,
    ) -> DdResult<bool> {
        let mut html = String::new();
        html.push_str("<a");
        if let Some(url) = &link.href {
            let url = self.project.link_url(url, self.page_path());
            if url.starts_with("--") {
                // failed expansion (eg --previous on first page)
                return Ok(false);
            }
            write!(html, " href=\"{url}\"")?;
        }
        html.push_str(" class=\"nav-link ");
        for class in classes {
            html.push(' ');
            html.push_str(class);
        }
        html.push('\"');
        if let Some(target) = &link.target {
            write!(html, " target=\"{target}\"")?;
        }
        html.push_str(">\n");
        for part in &link.content {
            match part {
                NavLinkPart::Img { src, alt } => {
                    let img_url = self.project.img_url(src, self.page_path());
                    if img_url.starts_with("--") {
                        // failed expansion (eg --logo with no logo configured)
                        return Ok(false);
                    }
                    write!(html, "<img src=\"{img_url}\"")?;
                    if let Some(alt) = alt {
                        let alt = escape_attr(alt);
                        write!(html, " alt=\"{alt}\"")?;
                        write!(html, " title=\"{alt}\"")?;
                    }
                    html.push_str(">\n");
                }
                NavLinkPart::InlineImg { src } => {
                    match self.project.load_file(src)? {
                        Some(content) => {
                            // we clean the content from xml or doctype declarations
                            let content = regex_remove!(r"<\?xml[^>]*>\s*"i, &content);
                            let content = regex_remove!(r"<!DOCTYPE[^>]*>\s*", &content);
                            html.push_str(&content);
                        }
                        None => {
                            eprintln!(
                                "{}: file not found in ddoc-link configuration: {}",
                                "error".red().bold(),
                                src.clone().red(),
                            );
                        }
                    }
                }
                NavLinkPart::Label(label) => {
                    html.push_str("<span>");
                    if !self.write_text(&mut html, label) {
                        return Ok(false);
                    }
                    html.push_str("</span>");
                }
            }
        }
        html.push_str("</a>\n");
        dest_html.push_str(&html);
        Ok(true)
    }
}
