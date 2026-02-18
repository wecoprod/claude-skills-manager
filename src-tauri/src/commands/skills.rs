use crate::models::skill::{Skill, SkillSource};
use crate::models::config::Config;
use crate::utils::markdown_parser::parse_frontmatter;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::{CLAUDE_DIR, COMMANDS_DIR, SKILLS_DIR};

/// Liste tous les skills (globaux + projets)
#[tauri::command]
pub async fn list_skills(project_paths: Vec<String>) -> Result<Vec<Skill>, String> {
    use std::collections::HashMap;

    let mut skills = Vec::new();
    let mut symlink_map: HashMap<String, Vec<String>> = HashMap::new();

    // 1. Scanner les skills globaux (depuis la config)
    let config = Config::load().map_err(|e| format!("Erreur chargement config: {}", e))?;
    let global_path = PathBuf::from(config.expanded_global_skills_path());

    if let Ok(global_skills) = scan_skills_directory(&global_path, SkillSource::Global, None) {
        skills.extend(global_skills);
    }

    // 2. Scanner les skills des projets fournis
    for project_path in project_paths {
        let project_skills_path = PathBuf::from(&project_path)
            .join(CLAUDE_DIR)
            .join(SKILLS_DIR);
        if project_skills_path.exists() {
            let project_name = PathBuf::from(&project_path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(String::from);

            if let Ok(proj_skills) =
                scan_skills_directory(&project_skills_path, SkillSource::Project, project_name.clone())
            {
                // Vérifier les symlinks (dossiers) et construire la map
                for skill in &proj_skills {
                    let skill_path = PathBuf::from(&skill.path);
                    if let Ok(metadata) = fs::symlink_metadata(&skill_path) {
                        if metadata.is_symlink() {
                            if let Ok(target) = fs::read_link(&skill_path) {
                                let target_str = target.to_string_lossy().to_string();
                                if let Some(ref pname) = project_name {
                                    symlink_map
                                        .entry(target_str)
                                        .or_insert_with(Vec::new)
                                        .push(pname.clone());
                                }
                            }
                        }
                    }
                }

                skills.extend(proj_skills);
            }
        }
    }

    // 3. Mettre à jour linked_projects pour chaque skill
    for skill in &mut skills {
        if let Some(projects) = symlink_map.get(&skill.path) {
            skill.linked_projects = projects.clone();
        }
    }

    Ok(skills)
}

/// Liste toutes les commandes (globales + projets)
#[tauri::command]
pub async fn list_commands(project_paths: Vec<String>) -> Result<Vec<Skill>, String> {
    let mut commands = Vec::new();

    let home = std::env::var("HOME").map_err(|_| "Variable HOME non définie".to_string())?;
    let global_commands_path = PathBuf::from(home).join(CLAUDE_DIR).join(COMMANDS_DIR);

    if let Ok(global_cmds) = scan_commands_directory(&global_commands_path, SkillSource::Global, None) {
        commands.extend(global_cmds);
    }

    for project_path in project_paths {
        let project_cmds_path = PathBuf::from(&project_path)
            .join(CLAUDE_DIR)
            .join(COMMANDS_DIR);
        if project_cmds_path.exists() {
            let project_name = PathBuf::from(&project_path)
                .file_name()
                .and_then(|n| n.to_str())
                .map(String::from);
            if let Ok(cmds) = scan_commands_directory(&project_cmds_path, SkillSource::Project, project_name) {
                commands.extend(cmds);
            }
        }
    }

    Ok(commands)
}

fn scan_commands_directory(
    base_path: &Path,
    source: SkillSource,
    project_name: Option<String>,
) -> Result<Vec<Skill>, String> {
    let mut commands = Vec::new();

    if !base_path.exists() {
        return Ok(commands);
    }

    for entry in WalkDir::new(base_path)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            let filename_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string();

            match load_command(path, source.clone(), project_name.clone()) {
                Ok(mut skill) => {
                    if skill.name == "Untitled Skill" || skill.name.is_empty() {
                        skill.name = filename_name;
                    }
                    commands.push(skill);
                }
                Err(e) => {
                    #[cfg(debug_assertions)]
                    eprintln!("Error loading command {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(commands)
}

/// Récupère un skill spécifique par son chemin (chemin du dossier)
#[tauri::command]
pub async fn get_skill(path: String) -> Result<Skill, String> {
    let folder_path = PathBuf::from(&path);

    let home = std::env::var("HOME").map_err(|_| "HOME env var not set")?;
    let global_skills_path = PathBuf::from(&home).join(CLAUDE_DIR).join(SKILLS_DIR);

    let (source, project_name) = if folder_path.starts_with(&global_skills_path) {
        (SkillSource::Global, None)
    } else {
        let project_name = folder_path
            .ancestors()
            .find(|p| p.join(CLAUDE_DIR).join(SKILLS_DIR).exists())
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .map(String::from);
        (SkillSource::Project, project_name)
    };

    load_skill(&folder_path, source, project_name).map_err(|e| e.to_string())
}

/// Scanne un répertoire pour trouver tous les dossiers contenant SKILL.md
fn scan_skills_directory(
    base_path: &Path,
    source: SkillSource,
    project_name: Option<String>,
) -> Result<Vec<Skill>, String> {
    let mut skills = Vec::new();

    if !base_path.exists() {
        return Ok(skills);
    }

    for entry in WalkDir::new(base_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Un skill est un dossier contenant SKILL.md
        if path.is_dir() && path.join("SKILL.md").exists() {
            match load_skill(path, source.clone(), project_name.clone()) {
                Ok(skill) => skills.push(skill),
                Err(e) => {
                    #[cfg(debug_assertions)]
                    eprintln!("Error loading skill {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(skills)
}

/// Charge un skill depuis un dossier (lit SKILL.md à l'intérieur)
fn load_skill(
    folder_path: &Path,
    source: SkillSource,
    project_name: Option<String>,
) -> anyhow::Result<Skill> {
    let skill_md = folder_path.join("SKILL.md");
    let content = fs::read_to_string(&skill_md)?;
    let (frontmatter, markdown) = parse_frontmatter(&content)?;

    let path_str = folder_path.to_string_lossy().to_string();
    let id = Skill::generate_id(&path_str);

    let metadata = fs::metadata(&skill_md)?;
    let last_modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let name = if frontmatter.name.is_empty() || frontmatter.name == "Untitled Skill" {
        folder_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Untitled Skill")
            .to_string()
    } else {
        frontmatter.name
    };

    Ok(Skill {
        id,
        name,
        description: frontmatter.description.unwrap_or_default(),
        path: path_str,
        source,
        project_name,
        content: markdown,
        last_modified,
        linked_projects: Vec::new(),
    })
}

/// Charge une commande depuis un fichier .md
fn load_command(
    file_path: &Path,
    source: SkillSource,
    project_name: Option<String>,
) -> anyhow::Result<Skill> {
    let content = fs::read_to_string(file_path)?;
    let (frontmatter, markdown) = parse_frontmatter(&content)?;

    let path_str = file_path.to_string_lossy().to_string();
    let id = Skill::generate_id(&path_str);

    let metadata = fs::metadata(file_path)?;
    let last_modified = metadata
        .modified()?
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    Ok(Skill {
        id,
        name: frontmatter.name,
        description: frontmatter.description.unwrap_or_default(),
        path: path_str,
        source,
        project_name,
        content: markdown,
        last_modified,
        linked_projects: Vec::new(),
    })
}

/// Crée un symlink d'un dossier skill vers un projet
#[tauri::command]
pub async fn create_skill_symlink(
    source_path: String,
    target_project_path: String,
) -> Result<String, String> {
    use std::os::unix::fs::symlink;

    let source = PathBuf::from(&source_path);
    if !source.exists() {
        return Err("Le dossier source n'existe pas".to_string());
    }

    let target_dir = PathBuf::from(&target_project_path)
        .join(CLAUDE_DIR)
        .join(SKILLS_DIR);

    fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Erreur création répertoire: {}", e))?;

    let folder_name = source
        .file_name()
        .ok_or("Impossible d'extraire le nom du dossier")?;
    let target_path = target_dir.join(folder_name);

    if target_path.exists() {
        return Err(format!(
            "Un skill avec ce nom existe déjà dans {}",
            target_project_path
        ));
    }

    symlink(&source, &target_path)
        .map_err(|e| format!("Erreur création symlink: {}", e))?;

    #[cfg(debug_assertions)]
    eprintln!("🔗 Symlink créé: {} -> {}", target_path.display(), source.display());

    Ok(target_path.to_string_lossy().to_string())
}

/// Duplique un skill (dossier) vers une nouvelle destination
#[tauri::command]
pub async fn duplicate_skill(
    source_path: String,
    destination_type: String,
    destination_project: Option<String>,
    new_name: String,
) -> Result<Skill, String> {
    let source = PathBuf::from(&source_path);
    if !source.exists() {
        return Err("Le dossier source n'existe pas".to_string());
    }

    let source_skill_md = source.join("SKILL.md");
    let source_content = fs::read_to_string(&source_skill_md)
        .map_err(|e| format!("Erreur lecture SKILL.md: {}", e))?;

    let (mut frontmatter, markdown) = parse_frontmatter(&source_content)
        .map_err(|e| format!("Erreur parsing frontmatter: {}", e))?;

    frontmatter.name = new_name.clone();

    let destination_base = if destination_type == "global" {
        let home = std::env::var("HOME").map_err(|_| "HOME env var not set")?;
        PathBuf::from(home).join(CLAUDE_DIR).join(SKILLS_DIR)
    } else {
        let project_path = destination_project
            .clone()
            .ok_or("Chemin du projet requis pour destination project")?;
        PathBuf::from(project_path)
            .join(CLAUDE_DIR)
            .join(SKILLS_DIR)
    };

    let destination_folder = base_name_to_kebab(&new_name);
    let destination_dir = destination_base.join(&destination_folder);

    if destination_dir.exists() {
        return Err(format!("Un skill nommé '{}' existe déjà à cet emplacement", new_name));
    }

    fs::create_dir_all(&destination_dir)
        .map_err(|e| format!("Erreur création dossier: {}", e))?;

    let new_content = build_skill_content(&frontmatter.name, frontmatter.description.as_deref(), &markdown);
    let skill_md_path = destination_dir.join("SKILL.md");
    fs::write(&skill_md_path, &new_content)
        .map_err(|e| format!("Erreur écriture SKILL.md: {}", e))?;

    #[cfg(debug_assertions)]
    eprintln!("📋 Skill dupliqué: {}", destination_dir.display());

    let source_type = if destination_type == "global" {
        SkillSource::Global
    } else {
        SkillSource::Project
    };

    load_skill(&destination_dir, source_type, destination_project).map_err(|e| e.to_string())
}

/// Supprime un skill (dossier ou symlink)
#[tauri::command]
pub async fn delete_skill(path: String) -> Result<(), String> {
    let skill_path = PathBuf::from(&path);

    if !skill_path.exists() && fs::symlink_metadata(&skill_path).is_err() {
        return Err("Le dossier n'existe pas".to_string());
    }

    let metadata = fs::symlink_metadata(&skill_path)
        .map_err(|e| format!("Erreur lecture metadata: {}", e))?;

    if metadata.is_symlink() {
        #[cfg(debug_assertions)]
        eprintln!("🗑️ Suppression du symlink: {}", skill_path.display());
        fs::remove_file(&skill_path)
            .map_err(|e| format!("Erreur suppression symlink: {}", e))?;
    } else if metadata.is_dir() {
        #[cfg(debug_assertions)]
        eprintln!("🗑️ Suppression du dossier: {}", skill_path.display());
        fs::remove_dir_all(&skill_path)
            .map_err(|e| format!("Erreur suppression dossier: {}", e))?;
    } else {
        // Fallback : fichier .md legacy
        fs::remove_file(&skill_path)
            .map_err(|e| format!("Erreur suppression fichier: {}", e))?;
    }

    Ok(())
}

/// Met à jour un skill (écrit dans path/SKILL.md)
#[tauri::command]
pub async fn update_skill(
    path: String,
    name: String,
    description: Option<String>,
    content: String,
) -> Result<Skill, String> {
    let folder_path = PathBuf::from(&path);
    let skill_md = folder_path.join("SKILL.md");

    if !skill_md.exists() {
        return Err("SKILL.md introuvable dans ce dossier".to_string());
    }

    let new_content = build_skill_content(&name, description.as_deref(), &content);
    fs::write(&skill_md, &new_content)
        .map_err(|e| format!("Erreur écriture: {}", e))?;

    #[cfg(debug_assertions)]
    eprintln!("✏️ Skill mis à jour: {}", skill_md.display());

    get_skill(path).await
}

fn build_skill_content(name: &str, description: Option<&str>, content: &str) -> String {
    let desc_line = description
        .map(|d| format!("\ndescription: {}", d))
        .unwrap_or_default();
    format!("---\nname: {}{}\n---\n\n{}", name, desc_line, content)
}

fn base_name_to_kebab(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
