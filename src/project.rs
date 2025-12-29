use {
    crate::*,
    lazy_regex::regex_captures,
    rustc_hash::FxHashMap,
    std::{
        borrow::Cow,
        fs,
        io::Write,
        path::{
            Path,
            PathBuf,
        },
    },
    termimad::crossterm::style::Stylize,
};

/// A ddoc project, with its configuration, pages, and
/// location which allows building it.
pub struct Project {
    pub root: PathBuf,
    pub config: Config,
    pub config_path: PathBuf,
    pub pages: FxHashMap<PagePath, Page>,
    pub src_path: PathBuf,
    pub build_path: PathBuf,
}

impl Project {
    /// Given the path to a ddoc project root,
    /// load its configuration and pages into a `Project` struct.
    pub fn load(path: &Path) -> DdResult<Self> {
        let (mut config, config_path) = Config::at_root(path)?;
        config.fix_old();
        let src_path = path.join("src");
        let pages = FxHashMap::default();
        let build_path = path.join("site");
        let nav = config.site_map.clone();
        let mut project = Self {
            config,
            config_path,
            root: path.to_owned(),
            pages,
            src_path: src_path.clone(),
            build_path,
        };
        nav.add_pages(&mut project);
        Ok(project)
    }
    /// Fills the 'site' directory with the generated HTML files and static files
    ///
    /// Don't do any prealable cleaning, call `clean_build_dir` first if needed.
    pub fn build(&self) -> DdResult<()> {
        self.copy_static("img")?;
        self.copy_static("js")?;
        self.copy_static("css")?;
        if self.config.needs_search_script() {
            let search_js_path = self.build_path.join("js").join("ddoc-search.js");
            if !search_js_path.exists() {
                if let Some(parent) = search_js_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(
                    &search_js_path,
                    include_bytes!("../resources/site/js/ddoc-search.js"),
                )?;
            }
        }
        for page_path in self.pages.keys() {
            self.build_page(page_path)?;
        }
        Ok(())
    }
    /// Try to update the project. Return true when some real work was done.
    ///
    /// #Errors
    /// Doesn't return error on user/data problems. A missing file, or
    /// an invalid config file will only trigger printed messages, not errors.
    pub fn update(
        &mut self,
        change: FileChange,
        base_url: &str, // for informing the user on the link to look at
    ) -> DdResult<bool> {
        match change {
            FileChange::Other => {
                self.reload_and_rebuild(base_url)?;
                return Ok(true);
            }
            FileChange::Removal(touched_path) => {
                // we care only if it's a CSS or JS file (header may have changed)
                if let Ok(rel_path) = touched_path.strip_prefix(&self.src_path) {
                    if rel_path.starts_with("css/") || rel_path.starts_with("js/") {
                        self.reload_and_rebuild(base_url)?;
                        return Ok(true);
                    }
                }
            }
            FileChange::Write(touched_path) => {
                // partial update for /src/img/ files and /src/*.md files
                if let Ok(rel_path) = touched_path.strip_prefix(&self.src_path) {
                    let ext = rel_path.extension().and_then(|s| s.to_str());
                    if ext == Some("md") {
                        for (page_path, page) in &self.pages {
                            if page.md_file_path == touched_path {
                                info!("Modified page {:?}", page_path);
                                let url = page_path.to_absolute_url(base_url);
                                eprintln!("Modified {}", url.yellow());
                                self.build_page(page_path)?;
                                return Ok(true);
                            }
                        }
                        return Ok(false); // might be a readme, etc.
                    }
                    if let Ok(rel_img) = rel_path.strip_prefix("img/") {
                        info!("Deployed image {rel_img:?}");
                        eprintln!("Deployed image {}", rel_img.to_string_lossy().yellow());
                        let dst_path = self.build_path.join("img").join(rel_img);
                        if let Some(parent) = dst_path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::copy(&touched_path, &dst_path)?;
                        return Ok(true);
                    }
                }
                // If the change is related to the config file, a JS or CSS file,
                // then we have to do a full rebuild (all headers may have changed).
                // For JS & CSS we could do it only if the file is new or renamed,
                //  but for now we do a full rebuild).
                self.reload_and_rebuild(base_url)?;
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn reload_and_rebuild(
        &mut self,
        base_url: &str, // for informing the user on the link to look at
    ) -> DdResult<()> {
        info!("full rebuild");
        eprintln!("Full rebuild of {}", base_url.yellow());
        self.config = {
            let Ok(new_config) = read_file::<Config, _>(&self.config_path) else {
                eprintln!(
                    "{}: could not read updated config file at {:?}, keeping the old one.",
                    "warning".yellow().bold(),
                    &self.config_path
                );
                return Ok(());
            };
            new_config
        };
        self.pages.clear();
        let nav = self.config.site_map.clone();
        nav.add_pages(self);
        self.build()?;
        Ok(())
    }
    /// remove the 'build' directory and its content
    pub fn clean_build_dir(&self) -> DdResult<()> {
        if self.build_path.exists() {
            fs::remove_dir_all(&self.build_path)?;
        }
        Ok(())
    }
    pub fn load_and_build(path: &Path) -> DdResult<()> {
        let project = Self::load(path)?;
        project.build()?;
        Ok(())
    }
    /// If the provided path corresponds to a page in the project,
    /// return its `PagePath`, else return `None`.
    pub fn page_path_of(
        &self,
        path: &Path,
    ) -> Option<&PagePath> {
        for (page_path, page) in &self.pages {
            if page.md_file_path == path {
                return Some(page_path);
            }
        }
        None
    }
    pub fn list_js(&self) -> DdResult<Vec<StaticEntry>> {
        let static_src = self.src_path.join("js");
        StaticEntry::list_in(&static_src, Some(".js"))
    }
    pub fn list_css(&self) -> DdResult<Vec<StaticEntry>> {
        let static_src = self.src_path.join("css");
        StaticEntry::list_in(&static_src, Some(".css"))
    }

    pub fn copy_static(
        &self,
        dir: &str,
    ) -> DdResult<()> {
        let static_src = self.src_path.join(dir);
        if !static_src.exists() {
            return Ok(());
        }
        let static_dst = self.build_path.join(dir);
        copy_normal_recursive(&static_src, &static_dst)?;
        Ok(())
    }
    pub fn build_page(
        &self,
        page_path: &PagePath,
    ) -> DdResult<()> {
        let page = self
            .pages
            .get(page_path)
            .ok_or_else(|| DdError::internal(format!("Page not found: {:?}", page_path)))?;
        let mut html = String::new();
        page.write_html(&mut html, self)?;
        let html_path = page_path.html_path_buf(&self.build_path);
        if let Some(parent) = html_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        if html_path.exists() {
            fs::remove_file(&html_path)?;
        }
        let mut file = fs::File::create(&html_path)?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }
    pub fn check_img_path(
        &self,
        mut img_path: &str, // a relative path like "img/xyz.png",
        page_path: &PagePath,
    ) {
        // We tolerate leading ../ in img paths, as long as they don't go
        // beyond the project src root.
        // They're useless, but may seem natural to some users.
        for _ in 0..page_path.depth() {
            if img_path.starts_with("../") {
                img_path = &img_path[3..];
            }
        }
        let path = self.src_path.join(img_path);
        if !path.exists() {
            eprintln!(
                "{}: {} contains a broken img src: {}",
                "error".red().bold(),
                page_path.to_string().yellow(),
                img_path.to_string().red(),
            );
        }
    }
    pub fn maybe_rewrite_img_url(
        &self,
        src: &str,
        page_path: &PagePath,
    ) -> Option<String> {
        // filtering to change only relative links to /img files
        if let Some((_, before, path)) = regex_captures!(r"^(\.\./)*(img/.*)$", &src) {
            self.check_img_path(src, page_path);
            let depth = page_path.depth();
            if depth == 0 && before.is_empty() {
                return None; // no rewriting needed
            }
            let mut url = String::new();
            for _ in 0..depth {
                url.push_str("../");
            }
            url.push_str(path);
            return Some(url);
        }
        None
    }
    pub fn img_url<'s>(
        &self,
        src: &'s str,
        page_path: &PagePath,
    ) -> Cow<'s, str> {
        match self.maybe_rewrite_img_url(src, page_path) {
            Some(new_url) => Cow::Owned(new_url),
            None => Cow::Borrowed(src),
        }
    }
    /// Check if the given `PagePath` exists in the project,
    /// write an error if it does not.
    pub fn check_page_path(
        &self,
        page_path: &PagePath,
    ) {
        if !self.pages.contains_key(page_path) {
            eprintln!("Error: link to non-existing page: {:?}", page_path);
        }
    }
    /// Return a modified link URL if it needs to be rewritten,
    /// return `None` if no rewriting is needed.
    pub fn maybe_rewrite_link_url(
        &self,
        src: &str,
        page_path: &PagePath,
    ) -> Option<String> {
        // special expansions
        if src == "--previous" {
            return self
                .config
                .site_map
                .previous(page_path)
                .map(|dst_page_path| page_path.link_to(dst_page_path));
        }
        if src == "--next" {
            return self
                .config
                .site_map
                .next(page_path)
                .map(|dst_page_path| page_path.link_to(dst_page_path));
        }
        if src == "--search" {
            return Some("javascript:ddoc_search.open();".to_string());
        }
        // rewrite absolute internal links, making them relative to the current page
        if let Some((_, path, file, _ext, hash)) =
            regex_captures!(r"^/([\w\-/]+/)*([\w\-/]*?)(?:index)?(\.md)?/?(#.*)?$", &src,)
        {
            let depth = page_path.depth();
            let mut url = String::new();
            for _ in 0..depth {
                url.push_str("../");
            }
            url.push_str(path);
            url.push_str(file);
            url.push_str(hash);
            let dst_page_path = PagePath::from_path_file(path, file);
            if !self.pages.contains_key(&dst_page_path) {
                eprintln!("path: {}, file: {}", path, file);
                eprintln!("dst_page_path: {:?}", dst_page_path);
                eprintln!(
                    "{}: {} contains a broken link: {}",
                    "error".red(),
                    page_path.to_string().yellow(),
                    src.to_string().red(),
                );
            }
            return Some(url);
        }
        // rewrite relative internal links to .md files
        if let Some((_, path, file, _ext, hash)) =
            regex_captures!(r"^(\.\./|[\w\-/]+/)*([\w\-/]+?)(\.md)?/?(#.*)?$", &src,)
        {
            let dst_page_path = page_path.follow_relative_link(path, file);
            if !self.pages.contains_key(&dst_page_path) {
                eprintln!(
                    "{}: {} contains a broken relative link: {}",
                    "error".red().bold(),
                    page_path.to_string().yellow(),
                    src.to_string().red(),
                );
            }
            let file = if file == "index" { "" } else { file };
            let url = format!("{}{}{}", path, file, hash,);
            return Some(url);
        }
        None
    }
    /// Return a modified link URL if it needs to be rewritten.
    ///
    /// If the src is an expansion and cannot be resolved,
    /// return the src unchanged.
    pub fn link_url<'s>(
        &self,
        src: &'s str,
        page_path: &PagePath,
    ) -> Cow<'s, str> {
        match self.maybe_rewrite_link_url(src, page_path) {
            Some(new_url) => Cow::Owned(new_url),
            None => Cow::Borrowed(src),
        }
    }
    pub fn static_url(
        &self,
        dir: &str,
        filename: &str,
        page_path: &PagePath,
    ) -> String {
        let depth = page_path.depth();
        let mut url = String::new();
        for _ in 0..depth {
            url.push_str("../");
        }
        url.push_str(dir);
        url.push('/');
        url.push_str(filename);
        url
    }
}

/// Copy normal non hidden files from `src_dir` to `dst_dir` recursively
fn copy_normal_recursive(
    src_dir: &Path,
    dst_dir: &Path,
) -> DdResult<()> {
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }
    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let sub_src = entry.path();
            let sub_dst = dst_dir.join(entry.file_name());
            copy_normal_recursive(&sub_src, &sub_dst)?;
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        let file_name = entry.file_name();
        let Some(file_name) = file_name.to_str() else {
            continue;
        };
        if file_name.starts_with('.') {
            continue;
        }
        let dest_path = dst_dir.join(file_name);
        if dest_path.exists() {
            fs::remove_file(&dest_path)?; // to have it updated
        }
        fs::copy(entry.path(), dest_path)?;
    }
    Ok(())
}
