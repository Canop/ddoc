use {
    crate::*,
    std::{
        fs,
        path::Path,
    },
};

pub fn init_src_in_dir(
    dir: &Path,
    init_values: &InitValues,
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
        fs::write(
            css_dir.join("site.css"),
            include_bytes!("../../resources/src/css/site.css"),
        )?;
    }

    // src/img/
    let img_dir = src_dir.join("img");
    if !img_dir.exists() {
        fs::create_dir_all(&img_dir)?;
        fs::write(
            img_dir.join("github-mark.svg"),
            include_bytes!("../../resources/src/img/github-mark.svg"),
        )?;
        fs::write(
            img_dir.join("github-mark-white.svg"),
            include_bytes!("../../resources/src/img/github-mark-white.svg"),
        )?;
    }

    Ok(())
}
