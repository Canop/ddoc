use {
    crate::*,
    notify::{
        EventKind,
        RecommendedWatcher,
        RecursiveMode,
        Watcher,
        event::{
            AccessKind,
            AccessMode,
            DataChange,
            ModifyKind,
        },
    },
    std::path::PathBuf,
    termimad::crossterm::style::Stylize,
};

/// watch for file changes and triggers a rebuild.
///
/// Caller should keep the returned watcher alive (e.g., by storing it in a variable)
/// as watching stops when the watcher is dropped.
pub fn rebuild_on_change(project_path: PathBuf) -> Result<RecommendedWatcher, notify::Error> {
    let config_path = project_path.join("ddoc.hjson");
    let src_path = project_path.join("src");
    let mut watcher =
        notify::recommended_watcher(move |res: notify::Result<notify::Event>| match res {
            Ok(we) => {
                match we.kind {
                    EventKind::Modify(ModifyKind::Metadata(_)) => {
                        //debug!("ignoring metadata change");
                        return; // useless event
                    }
                    EventKind::Modify(ModifyKind::Data(DataChange::Any)) => {
                        //debug!("ignoring 'any' data change");
                        return; // probably useless event with no real change
                    }
                    EventKind::Access(AccessKind::Close(AccessMode::Write)) => {
                        debug!("close write event: {we:?}");
                    }
                    EventKind::Access(_) => {
                        //debug!("ignoring access event: {we:?}");
                        return; // probably useless event
                    }
                    _ => {
                        info!("notify event: {we:?}");
                    }
                }
                eprintln!("Changes detected...");
                match Project::load_and_build(&project_path) {
                    Ok(_) => eprintln!("... {}", "site rebuilt successfully".green().bold()),
                    Err(e) => eprintln!("{}{}", "Error rebuilding site: ".red().bold(), e),
                }
            }
            Err(e) => warn!("watch error: {e:?}"),
        })?;
    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;
    eprintln!("src path: {}", src_path.display());
    watcher.watch(&src_path, RecursiveMode::Recursive)?;
    Ok(watcher)
}
