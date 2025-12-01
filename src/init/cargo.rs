use {
    lazy_regex::regex_is_match,
    std::path::Path,
};

#[derive(Debug, serde::Deserialize)]
pub struct CargoToml {
    package: CargoTomlPackage,
}

#[derive(Debug, serde::Deserialize)]
pub struct CargoTomlPackage {
    name: String,
    description: Option<String>,
    repository: Option<String>,
}

impl CargoToml {
    pub fn in_dir(dir: &Path) -> Option<Self> {
        let cargo_toml_path = dir.join("Cargo.toml");
        if !cargo_toml_path.exists() {
            return None;
        }
        let s = match std::fs::read_to_string(&cargo_toml_path) {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to read {}: {}", cargo_toml_path.display(), e);
                return None;
            }
        };
        let cargo_toml = match toml::from_str(&s) {
            Ok(cargo_toml) => cargo_toml,
            Err(e) => {
                warn!("Failed to parse {}: {}", cargo_toml_path.display(), e);
                return None;
            }
        };
        Some(cargo_toml)
    }

    pub fn project_name(&self) -> &str {
        &self.package.name
    }
    pub fn project_description(&self) -> Option<&str> {
        self.package.description.as_deref()
    }
    pub fn github_repository(&self) -> Option<String> {
        self.package
            .repository
            .as_deref()
            .filter(|repo| regex_is_match!(r"^https://github\.com/.+$", repo))
            .map(|s| s.to_owned())
    }
}
