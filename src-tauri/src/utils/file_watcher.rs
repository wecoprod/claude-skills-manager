use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use tauri::{AppHandle, Emitter};

/// Démarre le file watcher pour surveiller les modifications des fichiers skills
pub fn start_watcher(
    paths: Vec<PathBuf>,
    app_handle: AppHandle,
) -> Result<RecommendedWatcher, notify::Error> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                // Filtrer uniquement les modifications et suppressions
                match event.kind {
                    EventKind::Modify(_) | EventKind::Remove(_) => {
                        let _ = tx.send(event);
                    }
                    _ => {}
                }
            }
        },
        notify::Config::default(),
    )?;

    // Watch tous les chemins fournis
    for path in paths {
        if path.exists() {
            watcher.watch(&path, RecursiveMode::NonRecursive)?;
        }
    }

    // Spawn un thread pour écouter les événements et les envoyer au frontend
    std::thread::spawn(move || {
        while let Ok(event) = rx.recv() {
            // Émettre l'événement vers le frontend
            let paths: Vec<String> = event
                .paths
                .iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();

            let _ = app_handle.emit("skill-file-changed", paths);
        }
    });

    Ok(watcher)
}
