use {
    clap::{
        CommandFactory,
        Parser,
    },
    std::path::PathBuf,
};

static INTRO: &str = "
ddoc is a web documentation generator from markdown files.

";

/// Launch arguments
#[derive(Debug, Parser)]
#[command(
    author,
    about,
    version,
    disable_version_flag = true,
    disable_help_flag = true
)]
pub struct Args {
    /// Print help information
    #[arg(long)]
    pub help: bool,

    /// Print the version
    #[arg(long)]
    pub version: bool,

    /// Initialize a ddoc project in the specified directory
    #[arg(long)]
    pub init: bool,

    /// Serve files (for local development)
    #[arg(long)]
    pub serve: bool,

    /// Port to use when serving files (default: 8004)
    #[arg(long)]
    pub port: Option<u16>,

    pub path: Option<PathBuf>,
}

impl Args {
    pub fn print_help(&self) {
        let printer = clap_help::Printer::new(Args::command())
            .with("introduction", INTRO)
            .with("options", clap_help::TEMPLATE_OPTIONS_MERGED_VALUE)
            .without("author");
        printer.print_help();
    }
}
