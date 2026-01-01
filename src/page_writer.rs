use {
    crate::*,
    pulldown_cmark::{
        self as pcm,
        CowStr,
        Event,
        Parser,
        Tag,
        TagEnd,
        html::push_html,
    },
    std::fmt::Write,
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
        let mut toc = String::new(); // stores the LIs of the nav.page-toc
        let mut main = String::new(); // stores the HTML of the <main> tag

        let mut events = Parser::new_ext(md, pcm::Options::all()).collect::<Vec<_>>();
        for i in 0..events.len() {
            match &mut events[i] {
                // Rewrite the image source
                Event::Start(Tag::Image { dest_url, .. }) => {
                    if let Some(new_url) = project.maybe_rewrite_img_url(dest_url, &page.page_path)
                    {
                        *dest_url = CowStr::from(new_url);
                    }
                }

                // rewrite internal links
                Event::Start(Tag::Link { dest_url, .. }) => {
                    if let Some(new_url) = project.maybe_rewrite_link_url(dest_url, &page.page_path)
                    {
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
                    let new_id = heading_text
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == ' ')
                        .map(|c| if c == ' ' { '-' } else { c })
                        .collect::<String>();
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
        writeln!(html, "<body class=\"page-{}\">\n", &self.page_path().stem)?;
        self.write_html_element_list(html, &self.config().body)?;
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
        let title = format!("{} - {}", &self.page.title, &self.config().title);
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
            let url = self.project.static_url("js", &e.filename, self.page_path());
            writeln!(html, r#"<script src="{}?m={}"></script>"#, url, e.mtime)?;
        }
        if self.config().needs_search_script() {
            let url = self
                .project
                .static_url("js", "ddoc-search.js", self.page_path());
            writeln!(html, r#"<script src="{}" defer></script>"#, url)?;
        }
        for e in self.project.list_css()? {
            let url = self
                .project
                .static_url("css", &e.filename, self.page_path());
            writeln!(
                html,
                r#"<link href="{}?m={}" rel=stylesheet>"#,
                url, e.mtime
            )?;
        }
        html.push_str("</head>\n");
        Ok(())
    }

    /// Write the HTML for the content of a `ElementList`
    fn write_html_element_list(
        &self,
        html: &mut String,
        composite: &ElementList,
    ) -> DdResult<()> {
        for element in &composite.children {
            self.write_element(html, element)?;
        }
        Ok(())
    }

    fn write_element(
        &self,
        html: &mut String,
        element: &Element,
    ) -> DdResult<()> {
        match &element.content {
            ElementContent::Html { tag, children } => {
                writeln!(html, "<{}", tag)?;
                if !element.classes.is_empty() {
                    html.push_str(" class=\"");
                    for (i, class) in element.classes.iter().enumerate() {
                        if i > 0 {
                            html.push(' ');
                        }
                        html.push_str(class);
                    }
                    html.push('"');
                }
                html.push('>');
                for child in children {
                    self.write_element(html, child)?;
                }
                writeln!(html, "</{}>", tag)?;
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
            ElementContent::Toc => {
                html.push_str("<nav class=page-toc>\n");
                html.push_str("<a class=toc-title href=\"#top\">");
                html.push_str(&escape_text(&self.page.title));
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
        html: &mut String,
        classes: &[String],
        link: &NavLink,
    ) -> DdResult<()> {
        let mut unexpanded = false;
        html.push_str("<a");
        if let Some(url) = &link.href {
            let url = self.project.link_url(url, self.page_path());
            if url.starts_with("--") {
                // failed expansion (eg --previous on first page)
                unexpanded = true;
            } else {
                write!(html, " href=\"{url}\"")?;
            }
        }
        html.push_str(" class=\"nav-link ");
        if let Some(class) = &link.class {
            html.push_str(class);
        }
        for class in classes {
            html.push(' ');
            html.push_str(class);
        }
        if unexpanded {
            html.push_str(" unexpanded");
        }
        html.push('\"');
        if let Some(target) = &link.target {
            write!(html, " target=\"{target}\"")?;
        }
        html.push('>');
        if let Some(img) = &link.img {
            let img_url = self.project.img_url(img, self.page_path());
            write!(html, "<img src=\"{img_url}\"")?;
            if let Some(alt) = &link.alt {
                let alt = escape_attr(alt);
                write!(html, " alt=\"{alt}\"")?;
                write!(html, " title=\"{alt}\"")?;
            }
            html.push('>');
        }
        if let Some(label) = &link.label {
            let label = escape_text(label);
            write!(html, "<span>{label}</span>")?;
        }
        html.push_str("</a>\n");
        Ok(())
    }
}
