use {
    crate::*,
    lazy_regex::regex_captures,
    std::{
        fmt,
        path::{
            Path,
            PathBuf,
        },
        str::FromStr,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PagePath {
    pub dir: Vec<String>,
    pub stem: String,
}

impl PagePath {
    pub fn from_path_file(
        path: &str,
        stem: &str,
    ) -> Self {
        let mut dir: Vec<String> = path
            .split('/')
            .filter(|part| !part.is_empty())
            .map(|s| s.to_owned())
            .collect();
        let stem = if stem.is_empty() {
            if let Some(s) = dir.pop() {
                s
            } else {
                "index".to_owned()
            }
        } else {
            stem.to_owned()
        };
        Self { dir, stem }
    }
    /// Given a relative link (path and stem), return the new `PagePath`
    /// obtained by following that link from this `PagePath`.
    ///
    /// If there are too many ".." parts in the path, Some of those will be ignored
    /// (we don't go above the root).
    pub fn follow_relative_link(
        &self,
        path: &str,
        mut stem: &str,
    ) -> Self {
        let mut dir = self.dir.clone();
        if self.stem != "index" {
            dir.push(self.stem.clone());
        }
        let t_dir = path.split('/').filter(|part| !part.is_empty());
        for token in t_dir {
            if token == ".." {
                if dir.pop().is_none() {
                    warn!("relative link goes above root: ignoring extra ..");
                }
            } else if token != "." {
                dir.push(token.to_owned());
            }
        }
        if stem.is_empty() {
            stem = "index";
        }
        Self {
            dir,
            stem: stem.to_owned(),
        }
    }
    pub fn depth(&self) -> usize {
        if self.is_root_index() {
            0
        } else {
            1 + self.dir.len()
        }
    }
    pub fn to_path_buf(
        &self,
        parent: &Path,
        ext: Option<&str>,
    ) -> PathBuf {
        let mut path = parent.to_owned();
        for d in &self.dir {
            path.push(d);
        }
        let file_name = match ext {
            Some(e) => format!("{}.{}", self.stem, e),
            None => self.stem.clone(),
        };
        path.push(file_name);
        path
    }
    pub fn md_path_buf(
        &self,
        parent: &Path,
    ) -> PathBuf {
        let mut path = parent.to_owned();
        for d in &self.dir {
            path.push(d);
        }
        path.push(format!("{}.md", self.stem));
        path
    }
    pub fn html_path_buf(
        &self,
        parent: &Path,
    ) -> PathBuf {
        let mut path = parent.to_owned();
        if !self.is_root_index() {
            for d in &self.dir {
                path.push(d);
            }
            path.push(&self.stem);
        }
        path.push("index.html");
        path
    }
    pub fn is_root_index(&self) -> bool {
        self.dir.is_empty() && self.stem == "index"
    }
    pub fn to_absolute_url(
        &self,
        base_url: &str,
    ) -> String {
        let mut url = base_url.to_owned();
        if !url.ends_with('/') {
            url.push('/');
        }
        for d in &self.dir {
            url.push_str(d);
            url.push('/');
        }
        if self.stem != "index" {
            url.push_str(&self.stem);
            url.push('/');
        }
        url
    }
    /// Returns a relative link from this `PagePath` to another `PagePath`
    pub fn link_to(
        &self,
        other: &PagePath,
    ) -> String {
        let mut path = String::new();
        for _ in 0..self.depth() {
            path.push_str("../");
        }
        if !other.is_root_index() {
            for d in &other.dir {
                path.push_str(d);
                path.push('/');
            }
            path.push_str(&other.stem);
            path.push('/');
        }
        path
    }
}

impl FromStr for PagePath {
    type Err = DdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((_, dir, stem)) = regex_captures!(r"^([\w-]+/)*([\w-]+)(?:\.md)?$", s,) else {
            return Err(DdError::InvalidPagePath { path: s.to_owned() });
        };
        let dir = dir
            .trim_end_matches('/')
            .split('/')
            .filter(|part| !part.is_empty())
            .map(|s| s.to_owned())
            .collect();
        let stem = stem.to_owned();
        Ok(Self { dir, stem })
    }
}

impl fmt::Display for PagePath {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        for d in &self.dir {
            write!(f, "{}/", d)?;
        }
        write!(f, "{}", self.stem)
    }
}

impl serde::Serialize for PagePath {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> serde::Deserialize<'de> for PagePath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
