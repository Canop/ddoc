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
            RenameMode,
        },
    },
    termimad::crossterm::style::Stylize,
};

#[derive(Debug)]
pub enum FileChange {
    /// Creation, move inside, edition, etc.
    Write(std::path::PathBuf),
    /// Removal, move outside, etc. but also first part of a move inside
    Removal(std::path::PathBuf),
    /// Anything else looking relevant (e.g. multiple files written)
    /// A full rebuild is probably needed
    Other,
}

/// watch for file changes to keep a project up to date
///
/// Caller should keep the returned watcher alive (e.g., by storing it in a variable)
/// as watching stops when the watcher is dropped.
pub fn rebuild_on_change(
    mut project: Project,
    base_url: String, // to display the modified page URL
) -> Result<RecommendedWatcher, notify::Error> {
    let config_path = project.config_path.clone();
    let src_path = project.src_path.clone();
    let mut watcher =
        notify::recommended_watcher(move |res: notify::Result<notify::Event>| match res {
            Ok(we) => {
                // Filter to get events which are relevant for a rebuild
                // (not a cleaning) of the project:
                // - file being modified
                // - file being created
                // - file being renamed
                // - file being removed (matters when the file is linked from the head)
                let mut is_removal = false;
                match we.kind {
                    EventKind::Modify(ModifyKind::Metadata(_)) => {
                        return; // useless event
                    }
                    EventKind::Modify(ModifyKind::Data(DataChange::Any)) => {
                        return; // probably useless event with no real change
                    }
                    EventKind::Modify(ModifyKind::Name(RenameMode::From)) => {
                        debug!("rename from event: {we:?}");
                        is_removal = true;
                    }
                    EventKind::Modify(ModifyKind::Name(RenameMode::To)) => {
                        debug!("rename to event: {we:?}");
                    }
                    EventKind::Access(AccessKind::Close(AccessMode::Write)) => {
                        // file was created or modified
                        debug!("close write event: {we:?}");
                    }
                    EventKind::Access(_) => {
                        return; // probably useless event
                    }
                    _ => {
                        // probably useless, log just in case a user has missing rebuilds
                        debug!("skipped notify event: {we:?}");
                        return;
                    }
                }
                let path = if we.paths.len() == 1 {
                    Some(we.paths[0].clone())
                } else {
                    None // several paths changed
                };
                let change = match (path, is_removal) {
                    (Some(p), true) => FileChange::Removal(p),
                    (Some(p), false) => FileChange::Write(p),
                    (None, _) => FileChange::Other,
                };
                info!("rebuilding site due to {change:?}");
                let start = std::time::Instant::now();
                match project.update(change, &base_url) {
                    Ok(true) => eprintln!("Site rebuilt in {}", duration_since(start)),
                    Ok(false) => debug!("No rebuild needed"),
                    Err(e) => eprintln!("{}{}", "Error rebuilding site: ".red().bold(), e),
                }
            }
            Err(e) => warn!("watch error: {e:?}"),
        })?;
    watcher.watch(&config_path, RecursiveMode::NonRecursive)?;
    watcher.watch(&src_path, RecursiveMode::Recursive)?;
    Ok(watcher)
}

pub fn duration_since(start: std::time::Instant) -> String {
    let millis = start.elapsed().as_secs_f32() / 1000.0;
    format!("{:.3}ms", millis)
}
