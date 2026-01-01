use {
    crate::*,
    std::{
        fs,
        path::Path,
    },
};

/// Create and fill the `src/` directory in the given `dir`.
///
/// Break nothing, and don't write files if they don't look
/// necessary
pub fn init_src_in_dir(
    dir: &Path,
    init_values: &InitValues,
    _config: &Config,
) -> DdResult<()> {
    // src dir
    let src_dir = dir.join("src");
    if !src_dir.exists() {
        fs::create_dir_all(&src_dir)?;
    }

    // src/index.md
    let index_md_path = src_dir.join("index.md");
    if !index_md_path.exists() {
        if let Some(index_path) = &init_values.index {
            fs::copy(index_path, &index_md_path)?;
            eprintln!(
                "Created {} from {}",
                index_md_path.display(),
                index_path.display()
            );
        } else {
            // create a default index.md
            fs::write(
                &index_md_path,
                "# Welcome to your new ddoc documentation site!\n\n\
                This is the index page. You can edit this file to add your own content.\n",
            )?;
            eprintln!("Created {}", index_md_path.display());
        }
    }

    // src/css/
    let css_dir = src_dir.join("css");
    if !css_dir.exists() {
        fs::create_dir_all(&css_dir)?;
    }
    if !has_css(&css_dir)? {
        fs::write(
            css_dir.join("site.css"),
            include_bytes!("../../resources/src/css/site.css"),
        )?;
    }

    // src/img/
    let img_dir = src_dir.join("img");
    if !img_dir.exists() {
        fs::create_dir_all(&img_dir)?;
    }
    write_image_if_not_exists(
        &img_dir,
        "ddoc-search.svg",
        include_bytes!("../../resources/src/img/ddoc-search.svg"),
    )?;
    write_image_if_not_exists(
        &img_dir,
        "ddoc-left-arrow.svg",
        include_bytes!("../../resources/src/img/ddoc-left-arrow.svg"),
    )?;
    write_image_if_not_exists(
        &img_dir,
        "ddoc-right-arrow.svg",
        include_bytes!("../../resources/src/img/ddoc-right-arrow.svg"),
    )?;
    // the following image could be written only if the github navlink is used,
    // but we'd fail people willing to add it later, so we just add it now
    write_image_if_not_exists(
        &img_dir,
        "github-mark.svg",
        include_bytes!("../../resources/src/img/github-mark.svg"),
    )?;

    Ok(())
}

fn has_css(css_dir: &Path) -> DdResult<bool> {
    for entry in fs::read_dir(css_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("css") {
            return Ok(true);
        }
    }
    Ok(false)
}

fn write_image_if_not_exists(
    img_dir: &Path,
    filename: &str,
    data: &[u8],
) -> DdResult<()> {
    let img_path = img_dir.join(filename);
    if !img_path.exists() {
        fs::write(&img_path, data)?;
    }
    Ok(())
}
