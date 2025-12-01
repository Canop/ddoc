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
};

pub struct Project {
    pub root: PathBuf,
    pub config: Config,
    pub pages: FxHashMap<PagePath, Page>,
    pub src_path: PathBuf,
    pub build_path: PathBuf,
}

impl Project {
    pub fn load(path: &Path) -> DdResult<Self> {
        let config = Config::at_root(path)?;
        let src_path = path.join("src");
        let pages = FxHashMap::default();
        let build_path = path.join("site");
        let nav = config.menu.clone();
        let mut project = Self {
            config,
            root: path.to_owned(),
            pages,
            src_path: src_path.clone(),
            build_path,
        };
        nav.add_pages(&mut project);
        Ok(project)
    }
    pub fn build(&self) -> DdResult<()> {
        self.copy_static("img")?;
        self.copy_static("js")?;
        self.copy_static("css")?;
        for page_path in self.pages.keys() {
            self.build_page(page_path)?;
        }
        Ok(())
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

    pub fn maybe_rewrite_img_url(
        &self,
        src: &str,
        page_path: &PagePath,
    ) -> Option<String> {
        // filtering to change only relative links to /img files
        if let Some((_, before, path)) = regex_captures!(r"^(\.\./)*(img/.*)$", &src,) {
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
    /// Return a modified link URL if it needs to be rewritten,
    /// return None if no rewriting is needed.
    pub fn maybe_rewrite_link_url(
        &self,
        src: &str,
        page_path: &PagePath,
    ) -> Option<String> {
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
            return Some(url);
        }
        // rewrite relative internal links to .md files
        if let Some((_, path, file, _ext, hash)) =
            regex_captures!(r"^(\.\./|[\w\-/]+/)*([\w\-/]+)(\.md)?/?(#.*)?$", &src,)
        {
            return Some(format!("{}{}{}", path, file, hash,));
        }
        None
    }
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

fn copy_normal_recursive(
    src_dir: &Path,
    dst_dir: &Path,
) -> DdResult<()> {
    if !dst_dir.exists() {
        fs::create_dir_all(&dst_dir)?;
    }
    for entry in fs::read_dir(&src_dir)? {
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
        if file_name.starts_with(".") {
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
