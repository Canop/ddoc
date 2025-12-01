use {
    crate::*,
    std::path::{
        Path,
        PathBuf,
    },
};

#[derive(Debug, Default)]
pub struct InitValues {
    pub title: Option<String>,
    pub description: Option<String>,
    pub index: Option<PathBuf>,
    pub github_repo: Option<String>,
}

impl InitValues {
    pub fn guess(project_dir: &Path) -> DdResult<Self> {
        let Some(parent_dir) = project_dir.parent() else {
            return Ok(InitValues::default());
        };
        let mut init_values = Self::default();
        if let Some(cargo_toml) = CargoToml::in_dir(parent_dir) {
            if init_values.title.is_none() {
                init_values.title = Some(cargo_toml.project_name().to_owned());
            }
            if init_values.description.is_none() {
                if let Some(desc) = cargo_toml.project_description() {
                    init_values.description = Some(desc.to_owned());
                }
            }
            if init_values.github_repo.is_none() {
                init_values.github_repo = cargo_toml.github_repository();
            }
        }
        if let Some(readme_path) = find_file_ignore_case(parent_dir, "README.md") {
            if init_values.index.is_none() {
                init_values.index = Some(readme_path);
            }
        }
        Ok(init_values)
    }
}

pub fn find_file_ignore_case(
    dir: &Path,
    filename: &str,
) -> Option<PathBuf> {
    let filename_lower = filename.to_lowercase();
    let read_dir = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return None,
    };
    for entry in read_dir.flatten() {
        let file_name_os = entry.file_name();
        let file_name_str = match file_name_os.to_str() {
            Some(s) => s,
            None => continue,
        };
        if file_name_str.to_lowercase() == filename_lower {
            return Some(entry.path());
        }
    }
    None
}
