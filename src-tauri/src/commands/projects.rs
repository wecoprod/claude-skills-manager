use crate::models::skill::Project;
use crate::models::config::Config;
use std::path::PathBuf;
use walkdir::WalkDir;

use super::{CLAUDE_DIR, SKILLS_DIR, MAX_SCAN_DEPTH};

/// Scanne les répertoires configurés pour trouver les projets avec .claude
#[tauri::command]
pub async fn scan_projects(base_dirs: Vec<String>) -> Result<Vec<Project>, String> {
    let mut projects = Vec::new();

    // Scanner chaque répertoire de base
    for base_dir in base_dirs {
        // Expanser le tilde (~) si présent
        let expanded_dir = Config::expand_tilde(&base_dir);
        let base_path = PathBuf::from(&expanded_dir);

        if !base_path.exists() {
            #[cfg(debug_assertions)]
            eprintln!("⚠️ Base path does not exist: {:?} (expanded from: {})", base_path, base_dir);
            continue;
        }

        #[cfg(debug_assertions)]
        eprintln!("🔍 Scanning for projects in: {:?}", base_path);

        // Parcourir les sous-répertoires
        for entry in WalkDir::new(&base_path)
            .max_depth(MAX_SCAN_DEPTH)
            .into_iter()
            .filter_map(|e| e.ok())
        {
        let path = entry.path();

        // Chercher les répertoires .claude
        if path.ends_with(CLAUDE_DIR) && path.is_dir() {
            if let Some(project_root) = path.parent() {
                let project_name = project_root
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown")
                    .to_string();

                let skills_path = path.join(SKILLS_DIR);

                // Compter le nombre de fichiers .md dans le dossier skills (s'il existe)
                let skill_count = if skills_path.exists() {
                    std::fs::read_dir(&skills_path)
                        .ok()
                        .map(|entries| {
                            entries
                                .filter_map(|e| e.ok())
                                .filter(|e| {
                                    e.path()
                                        .extension()
                                        .and_then(|s| s.to_str())
                                        == Some("md")
                                })
                                .count()
                        })
                        .unwrap_or(0)
                } else {
                    0
                };

                #[cfg(debug_assertions)]
                eprintln!("✅ Found project: {} with {} skills", project_name, skill_count);

                projects.push(Project {
                    name: project_name,
                    path: project_root.to_string_lossy().to_string(),
                    skills_path: skills_path.to_string_lossy().to_string(),
                    skill_count,
                });
            }
        }
        }
    }

    #[cfg(debug_assertions)]
    eprintln!("📊 Total projects found: {}", projects.len());

    Ok(projects)
}
