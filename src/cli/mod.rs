mod args;

pub use args::*;

use {
    crate::*,
    clap::Parser,
    std::path::Path,
    termimad::crossterm::style::Stylize,
};

/// Run the ddoc command line application
///
/// # Errors
/// Return errors only on unexpected failures, not on invalid
/// data (those are printed to stderr)
pub fn run() -> DdResult<()> {
    init_cli_log!();
    let args: Args = Args::parse();
    info!("args: {:#?}", &args);

    if args.help {
        args.print_help();
        return Ok(());
    }

    if args.version {
        println!("cradoc {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let project_path = args.path.as_deref().unwrap_or(Path::new("."));

    if args.init {
        return match init_ddoc_project(project_path) {
            Err(DdError::InitNotPossible(reason)) => {
                eprintln!(
                    "{}\n{}",
                    "Cannot initialize ddoc project:".red().bold(),
                    reason,
                );
                Ok(())
            }
            res => res,
        };
    }

    let project_path = project_path.canonicalize().map_err(|error| DdError::Read {
        path: project_path.to_owned(),
        error,
    })?;

    let project = match Project::load(&project_path) {
        Err(DdError::ConfigNotFound) => {
            // A frequent error is to run ddoc in a super director
            // of a ddoc project, so we check for that
            if let Some(subdir) = project_subdirectory(&project_path) {
                let name = subdir
                    .file_name()
                    .map_or("that dir".to_string(), |s| s.to_string_lossy().to_string());
                eprintln!(
                    "{}\nBut {} looks like a ddoc project\nSo maybe do {} then run ddoc there?",
                    "No ddoc.hjson found in current directory".red().bold(),
                    subdir.to_string_lossy().yellow(),
                    format!("cd {}", name).green(),
                );
            } else {
                eprintln!(
                    "{}\nYou can initialize ddoc with {}",
                    "No ddoc.hjson found".red().bold(),
                    "ddoc --init".green().bold(),
                );
            }
            return Ok(());
        }
        res => res,
    }?;

    // On launch, we clean the build directory to avoid stale files
    // (and prevent users from thinking they should edit files there)
    project.clean_build_dir()?;
    project.build()?;
    eprintln!(
        "site built in {}",
        project.build_path.to_string_lossy().yellow()
    );

    if args.serve {
        let port = args.port.unwrap_or(8004);
        let server = Server::new(project.build_path.clone(), port)?;
        eprintln!(
            "Serving {} at {}",
            project.config.title.clone().yellow().bold(),
            server.base_url().green().bold(),
        );

        // we watch for changes and rebuild automatically on a background thread
        let _watcher = match rebuild_on_change(project, server.base_url().to_string()) {
            Ok(w) => {
                info!(
                    "Watching for changes in {}",
                    project_path.to_string_lossy().yellow()
                );
                Some(w)
            }
            Err(e) => {
                eprintln!(
                    "{} {}",
                    "Warning: could not start file watcher:".yellow().bold(),
                    e,
                );
                // we still serve even if the watcher could not be started
                None
            }
        };

        server.run()?;
    }

    Ok(())
}
