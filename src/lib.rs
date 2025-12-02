//!
//! **ddoc** is a markdown based static site generator.
//!
//! **Complete documentation at [https://dystroy.org/ddoc](https://dystroy.org/ddoc)**
//!
//! ## Usage overview
//!
//! Create a directory, then move to it
//!
//!     mkdir website & cd website
//!
//! Initialize the site:
//!
//!     ddoc --init
//!
//! This creates:
//!
//! - a `.gitignore` file, which eases inclusion of your site in a git managed project
//! - a `ddoc.hjson` file, holding the basic properties and navigation
//! - a `src` folder, for your markdown files, CSS style sheets and images
//!
//! `/src/css/site.css` is a default CSS file, a very simple one which you can remove, or keep as basis for your own incremental changes to get the layout and look you desire.
//!
//! To build your site, run
//!
//!     ddoc
//!
//! This updates a `site` directory, whose content can be sent to your server.
//!
//! If you want to test it locally, you may run
//!
//!     ddoc --serve
//!

mod cli;
mod config;
mod error;
mod files;
mod html;
mod init;
mod page;
mod page_path;
mod project;
mod server;
mod statics;
mod watcher;

pub use {
    cli::*,
    config::*,
    error::*,
    files::*,
    html::*,
    init::*,
    page::*,
    page_path::*,
    project::*,
    server::*,
    statics::*,
    watcher::*,
};

#[macro_use]
extern crate cli_log;
