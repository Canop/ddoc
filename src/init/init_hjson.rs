use {
    crate::*,
    std::{
        borrow::Cow,
        fs,
        path::Path,
    },
};

static TEMPLATE_INIT_HJSON: &str = r#"
# This is a configuration file for the ddoc static site generator.
# For details and instruction, see https://dystroy.org/ddoc/

title: "<title>"
description: ""
favicon: img/favicon.ico

// All pages must be listed here
// One of them must be index.md
// You can have submenus, eg:
// pages: {
//     Home: index.md
//     Guide: {
//         "Getting Started": guide/getting_started.md
//         "Advanced Topics": guide/advanced_topics.md
//     }
// }
pages: {
    Home: index.md
}

// Nav links can have { img, url, class, label}, all optional
// URL starting with '/' are relative to the site's root
nav_links: {
    before_menu: [
        // this is a good place for a logo or a link to a wider site
    ]
    after_menu: [
        <github-navlink>
    ]
}

// UI options
ui: {
    // if true, the generated HTML includes a checkbox which
    // can be styled into a hamburger menu for small screens
    hamburger_checkbox: true
}

"#;
static TEMPLATE_GITHUB_NAVLINK: &str = r#"{
            img: img/github-mark-white.svg
            class: external-nav-link
            alt: GitHub
            url: <url>
        }"#;

/// Initialize a ddoc.hjson file in the specified directory
/// (do nothing if one already exists)
pub fn init_hjson_in_dir(
    dir: &Path,
    init_values: &InitValues,
) -> DdResult<()> {
    let path = dir.join("ddoc.hjson");
    if path.exists() {
        eprintln!(
            "{} already exists, keeping the existing file.",
            path.display()
        );
        return Ok(());
    }
    let mut hjson = TEMPLATE_INIT_HJSON.to_owned();
    let title = init_values.title.as_deref().unwrap_or("Unnamed Site");
    let github_navlink = if let Some(github_repo) = &init_values.github_repo {
        TEMPLATE_GITHUB_NAVLINK.replace("<url>", github_repo).into()
    } else {
        Cow::Borrowed("// links here will appear after the menu")
    };

    hjson = hjson
        .replace("<title>", title)
        .replace("<github-navlink>", &github_navlink);

    fs::write(&path, hjson)?;
    eprintln!("Created {}", path.display());
    Ok(())
}
