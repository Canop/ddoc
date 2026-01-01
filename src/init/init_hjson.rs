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

title: <title>
description: <description>
favicon: null // eg "img/favicon.ico"

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
site-map: {
    Home: index.md
}

// This describes the content of the <body> element of each page.
// Elements starting with 'ddoc-' will be replaced by special items:
// links, page TOC, global menu, main HTML translated from markdown, etc.
//
// Links (ddoc-link) can have { img, href, label, alt, target}.
// All these fields are optional.
// Hrefs starting with '/' are relative to the site's root (eg '/guide/help.md')
body: {
    header: {
        nav.before-menu: {
            // this is a good place for a logo or a link to a wider site
        }
        ddoc-menu: {
            // if true, the generated HTML includes a checkbox which
            // can be styled into a hamburger menu for small screens
            hamburger-checkbox: true
        }
        nav.after-menu: {
            ddoc-link.previous-page-link: {
                inline: img/ddoc-left-arrow.svg
                href: --previous
                alt: Previous Page
            }
            ddoc-link.search-opener: {
                inline: img/ddoc-search.svg
                href: --search
                alt: Search
            }
            ddoc-link.next-page-link: {
                inline: img/ddoc-right-arrow.svg
                href: --next
                alt: Next Page
            }
            <github-navlink>
        }
    }
    article: {
        aside.page-nav: {
            ddoc-toc: {}
        }
        ddoc-main: {}
    }
    footer: {
    }
}

"#;
static TEMPLATE_GITHUB_NAVLINK: &str = r#"ddoc-link.external-nav-link: {
                inline: img/github-mark.svg
                alt: GitHub
                href: <url>
            }"#;

/// Initialize a ddoc.hjson file in the specified directory
/// (do nothing if one already exists)
///
/// # Errors
/// Return `DdError::InvalidConfig` if an existing ddoc.hjson
/// cannot be read, or other less likely `DdError` variants on
/// write errors when creating a new ddoc.hjson
pub fn init_hjson_in_dir(
    dir: &Path,
    init_values: &InitValues,
) -> DdResult<Config> {
    let path = dir.join("ddoc.hjson");
    if path.exists() {
        read_file(&path).map_err(|e| {
            error!("Error reading {}: {}", path.display(), e);
            // Return a specific error so that the caller can
            // issue a proper message to the user
            DdError::InvalidConfig
        })
    } else {
        // FIXME we should handle the case of this generation failing
        // to build a parsable hjson (then probably give up with the
        // init values)
        let mut hjson = TEMPLATE_INIT_HJSON.to_owned();
        let title = init_values.title.as_deref().unwrap_or("Unnamed Site");
        let description = init_values.description.as_deref().unwrap_or("");
        let github_navlink = if let Some(github_repo) = &init_values.github_repo {
            TEMPLATE_GITHUB_NAVLINK.replace("<url>", github_repo).into()
        } else {
            Cow::Borrowed("// links here will appear after the menu")
        };

        hjson = hjson
            .replace("<title>", &escape_hjson_string(title))
            .replace("<description>", &escape_hjson_string(description))
            .replace("<github-navlink>", &github_navlink);

        fs::write(&path, hjson)?;
        eprintln!("Created {}", path.display());
        read_file(&path)
    }
}

pub fn escape_hjson_string(s: &str) -> String {
    format!("{:?}", s)
}
