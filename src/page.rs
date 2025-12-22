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
    std::{
        fmt::Write,
        fs,
        path::PathBuf,
    },
    termimad::crossterm::style::Stylize,
};

pub struct Page {
    pub title: String,
    pub page_path: PagePath,
    pub md_file_path: PathBuf,
}

impl Page {
    pub fn new(
        title: String,
        page_path: PagePath,
        md_file_path: PathBuf,
    ) -> Self {
        Self {
            title,
            page_path,
            md_file_path,
        }
    }
    /// Write the variable parts of the HTML `<head>`: the links to CSS, JS, meta tags, title, etc.
    ///
    /// # Errors
    /// Return `DdError` variants on write errors, not on project config/data errors
    pub fn write_html_head(
        &self,
        html: &mut String,
        project: &Project,
    ) -> DdResult<()> {
        html.push_str(HTML_START);
        let title = format!("{} - {}", &self.title, &project.config.title);
        writeln!(html, "<title>{}</title>", escape_text(&title))?;
        writeln!(
            html,
            "<meta name=\"og-title\" content=\"{}\">",
            escape_attr(&title)
        )?;
        if let Some(description) = project.config.description() {
            let description = escape_attr(description);
            writeln!(html, r#"<meta name="description" content="{description}">"#)?;
            writeln!(
                html,
                r#"<meta name="og-description" content="{description}">"#
            )?;
        }
        if let Some(url) = project.config.favicon() {
            let url = project.img_url(url, &self.page_path);
            writeln!(html, r#"<link rel="shortcut icon" href="{url}">"#)?;
        }
        for e in project.list_js()? {
            let url = project.static_url("js", &e.filename, &self.page_path);
            writeln!(html, r#"<script src="{}?m={}"></script>"#, url, e.mtime)?;
        }
        if project.config.needs_search_script() {
            let url = project.static_url("js", "ddoc-search.js", &self.page_path);
            writeln!(html, r#"<script src="{}" defer></script>"#, url)?;
        }
        for e in project.list_css()? {
            let url = project.static_url("css", &e.filename, &self.page_path);
            writeln!(
                html,
                r#"<link href="{}?m={}" rel=stylesheet>"#,
                url, e.mtime
            )?;
        }
        html.push_str("</head>\n");
        Ok(())
    }

    fn write_nav_links(
        &self,
        html: &mut String,
        nav_links: &[NavLink],
        project: &Project,
    ) -> DdResult<()> {
        for link in nav_links {
            let mut unexpanded = false;
            html.push_str("<a");
            if let Some(url) = &link.href {
                let url = project.link_url(url, &self.page_path);
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
            if unexpanded {
                html.push_str(" unexpanded");
            }
            html.push('\"');
            if let Some(target) = &link.target {
                write!(html, " target=\"{target}\"")?;
            }
            html.push('>');
            if let Some(img) = &link.img {
                let img_url = project.img_url(img, &self.page_path);
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
        }
        Ok(())
    }
    /// Write the HTML content for a navigation directory (header or footer)
    pub fn write_nav_dir(
        &self,
        html: &mut String,
        tag: &str,
        nav_dir: &NavDir,
        project: &Project,
    ) -> DdResult<()> {
        if !nav_dir.is_empty() {
            writeln!(html, "<{tag}>")?;
            for (class_name, component) in &nav_dir.components {
                match component {
                    NavComponent::NavLinks(links) => {
                        writeln!(html, "<nav class=\"{class_name}\">")?;
                        self.write_nav_links(html, links, project)?;
                        html.push_str("</nav>\n");
                    }
                    NavComponent::Menu(_) => {
                        project
                            .config
                            .menu
                            .push_nav(html, project, class_name, &self.page_path)?;
                    }
                }
            }
            writeln!(html, "</{tag}>")?;
        }
        Ok(())
    }

    /// Write the part of the HTML generated from the page's Markdown content
    ///
    /// The resulting html contains,
    /// - at top level: an `<article>` element, which contains:
    ///   - `<nav class=page-toc>` : the table of contents for the page
    ///   - `<main>` : the main content of the page
    pub fn write_html_article(
        &self,
        html: &mut String,
        md: &str,
        project: &Project,
    ) -> DdResult<()> {
        let mut toc = String::new(); // stores the LIs of the nav.page-toc

        let mut events = Parser::new_ext(md, pcm::Options::all()).collect::<Vec<_>>();
        for i in 0..events.len() {
            match &mut events[i] {
                // Rewrite the image source
                Event::Start(Tag::Image { dest_url, .. }) => {
                    if let Some(new_url) = project.maybe_rewrite_img_url(dest_url, &self.page_path)
                    {
                        *dest_url = CowStr::from(new_url);
                    }
                }

                // rewrite internal links
                Event::Start(Tag::Link { dest_url, .. }) => {
                    if let Some(new_url) = project.maybe_rewrite_link_url(dest_url, &self.page_path)
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

        html.push_str("<article>\n");

        // push the nav.page-toc
        html.push_str("<aside class=page-nav>\n");
        html.push_str("<nav class=page-toc>\n");
        html.push_str("<a class=toc-title href=\"#top\">");
        html.push_str(&self.title); // TODO escape HTML
        html.push_str("</a>\n");
        if !toc.is_empty() {
            html.push_str("<ul class=toc-content>");
            html.push_str(&toc);
            html.push_str("</ul>");
        }
        html.push_str("</nav>\n");
        html.push_str("</aside>\n");
        // push the HTML matching the file's Markdown content
        html.push_str("<main>\n");
        push_html(html, events.into_iter());
        html.push_str("</main>\n");

        html.push_str("</article>\n");
        Ok(())
    }

    /// Write the full HTML for this page into the given `html` String
    ///
    /// # Errors
    /// Return `DdError` variants on write errors, not on project config/data errors
    pub fn write_html(
        &self,
        html: &mut String,
        project: &Project,
    ) -> DdResult<()> {
        // first check that the page's md file exists
        let Ok(md) = fs::read_to_string(&self.md_file_path) else {
            eprintln!(
                "{} {} could not be read, skipping.",
                "ERROR:".red().bold(),
                self.md_file_path.to_string_lossy().yellow()
            );
            return Ok(());
        };
        self.write_html_head(html, project)?;
        writeln!(html, "<body class=\"page-{}\">\n", &self.page_path.stem)?;
        self.write_nav_dir(html, "header", &project.config.header, project)?;
        self.write_html_article(html, &md, project)?; // page-to & article
        self.write_nav_dir(html, "footer", &project.config.footer, project)?;
        html.push_str("</html>\n");
        Ok(())
    }
}
