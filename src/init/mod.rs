mod cargo;
mod init_hjson;
mod init_src;
mod init_values;

pub use {
    cargo::*,
    init_hjson::*,
    init_src::*,
    init_values::*,
};

use {
    crate::*,
    std::{
        fs,
        path::PathBuf,
    },
    termimad::crossterm::style::Stylize,
};

pub fn init_ddoc_project(dir: PathBuf) -> DdResult<()> {
    // first check the directory is suitable
    if dir.exists() {
        if CargoToml::in_dir(&dir).is_some() {
            return Err(DdError::InitNotPossible(format!(
                "Directory {} looks busy, you should probably init ddoc in a subdirectory",
                dir.display()
            )));
        }
    } else {
        fs::create_dir_all(&dir).map_err(|e| {
            DdError::InitNotPossible(format!(
                "Failed to create directory {}: {}",
                dir.display(),
                e
            ))
        })?;
    }
    let dir = dir.canonicalize()?;

    eprintln!("Initializing ddoc project in {}", dir.display());

    let init_values = InitValues::guess(&dir)?;

    // ddoc.hjson
    init_hjson_in_dir(&dir, &init_values)?;

    // .gitignore
    let gitignore_path = dir.join(".gitignore");
    if !gitignore_path.exists() {
        fs::write(&gitignore_path, "/site\n")?;
        eprintln!("Created {}", gitignore_path.display());
    }

    // src/
    init_src_in_dir(&dir, &init_values)?;

    // print some instructions
    eprintln!(
        "Head to {} for the guide on site edition",
        "https://dystroy.org/ddoc/edit".yellow().bold()
    );
    eprintln!("Run {} to try your site", "ddoc --serve".green().bold());

    Ok(())
}
