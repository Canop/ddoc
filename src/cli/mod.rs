mod args;

pub use args::*;

use {
    crate::*,
    clap::Parser,
    std::path::Path,
    termimad::crossterm::style::Stylize,
};

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
        return match init_ddoc_project(project_path.to_path_buf()) {
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
            eprintln!(
                "{}\nYou can initialize ddoc with {}",
                "No ddoc.hjson found".red().bold(),
                "ddoc --init".green().bold(),
            );
            return Ok(());
        }
        res => res,
    }?;
    project.build()?;

    if args.serve {
        let port = args.port.unwrap_or(8004);
        serve_project(&project, port)?;
    }

    Ok(())
}
