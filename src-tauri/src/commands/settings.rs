use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use super::CLAUDE_DIR;

fn claude_json_path() -> Result<PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "Variable HOME non définie".to_string())?;
    Ok(PathBuf::from(home).join(".claude.json"))
}

fn read_json_file(path: &PathBuf) -> serde_json::Value {
    if !path.exists() {
        return serde_json::Value::Object(serde_json::Map::new());
    }
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new()))
}

fn merge_and_write(path: &PathBuf, key: &str, value: serde_json::Value) -> Result<(), String> {
    let mut json = read_json_file(path);
    if let Some(obj) = json.as_object_mut() {
        obj.insert(key.to_string(), value);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Erreur création dossier: {}", e))?;
    }
    let content = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("Erreur sérialisation: {}", e))?;
    fs::write(path, content).map_err(|e| format!("Erreur écriture: {}", e))
}

/// Source de hooks avec son chemin et label lisible
#[derive(Serialize)]
pub struct HooksSource {
    pub path: String,
    pub label: String,
    pub hooks: serde_json::Value,
}

/// Charge les hooks depuis ~/.claude/settings.json ET tous les projets fournis
#[tauri::command]
pub async fn load_hooks(project_paths: Vec<String>) -> Result<Vec<HooksSource>, String> {
    let mut sources: Vec<HooksSource> = Vec::new();

    // 1. Source globale : ~/.claude/settings.json
    let home = std::env::var("HOME").map_err(|_| "Variable HOME non définie".to_string())?;
    let global_path = PathBuf::from(&home).join(CLAUDE_DIR).join("settings.json");
    let global_json = read_json_file(&global_path);
    sources.push(HooksSource {
        path: global_path.to_string_lossy().to_string(),
        label: "Global".to_string(),
        hooks: global_json
            .get("hooks")
            .cloned()
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
    });

    // 2. Tous les projets (même sans settings.json existant)
    for project_path in project_paths {
        let settings_path = PathBuf::from(&project_path)
            .join(CLAUDE_DIR)
            .join("settings.json");

        let label = PathBuf::from(&project_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Projet")
            .to_string();

        let json = read_json_file(&settings_path);
        sources.push(HooksSource {
            path: settings_path.to_string_lossy().to_string(),
            label,
            hooks: json
                .get("hooks")
                .cloned()
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
        });
    }

    Ok(sources)
}

/// Sauvegarde les hooks dans un fichier settings.json spécifique (merge JSON)
#[tauri::command]
pub async fn save_hooks(source_path: String, hooks: serde_json::Value) -> Result<(), String> {
    let path = PathBuf::from(&source_path);
    merge_and_write(&path, "hooks", hooks)
}

/// Charge les plugins depuis ~/.claude/settings.json (clé enabledPlugins)
#[tauri::command]
pub async fn load_plugins() -> Result<serde_json::Value, String> {
    let home = std::env::var("HOME").map_err(|_| "Variable HOME non définie".to_string())?;
    let path = PathBuf::from(home).join(CLAUDE_DIR).join("settings.json");
    let json = read_json_file(&path);
    Ok(json
        .get("enabledPlugins")
        .cloned()
        .unwrap_or(serde_json::Value::Object(serde_json::Map::new())))
}

/// Sauvegarde les plugins dans ~/.claude/settings.json (merge JSON)
#[tauri::command]
pub async fn save_plugins(enabled_plugins: serde_json::Value) -> Result<(), String> {
    let home = std::env::var("HOME").map_err(|_| "Variable HOME non définie".to_string())?;
    let path = PathBuf::from(home).join(CLAUDE_DIR).join("settings.json");
    merge_and_write(&path, "enabledPlugins", enabled_plugins)
}

/// Source de serveurs MCP avec son chemin et label lisible
#[derive(Serialize)]
pub struct McpSource {
    pub path: String,
    pub label: String,
    pub servers: serde_json::Value,
}

/// Charge les serveurs MCP depuis ~/.claude.json ET tous les projets fournis
#[tauri::command]
pub async fn load_mcp(project_paths: Vec<String>) -> Result<Vec<McpSource>, String> {
    let mut sources: Vec<McpSource> = Vec::new();

    // 1. Source globale : ~/.claude.json
    let path = claude_json_path()?;
    let json = read_json_file(&path);
    sources.push(McpSource {
        path: path.to_string_lossy().to_string(),
        label: "Global".to_string(),
        servers: json
            .get("mcpServers")
            .cloned()
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
    });

    // 2. Tous les projets (même sans settings.json existant)
    for project_path in project_paths {
        let settings_path = PathBuf::from(&project_path)
            .join(CLAUDE_DIR)
            .join("settings.json");

        let label = PathBuf::from(&project_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Projet")
            .to_string();

        let proj_json = read_json_file(&settings_path);
        sources.push(McpSource {
            path: settings_path.to_string_lossy().to_string(),
            label,
            servers: proj_json
                .get("mcpServers")
                .cloned()
                .unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
        });
    }

    Ok(sources)
}

/// Sauvegarde les serveurs MCP dans un fichier spécifique (merge JSON)
#[tauri::command]
pub async fn save_mcp(source_path: String, mcp_servers: serde_json::Value) -> Result<(), String> {
    let path = PathBuf::from(&source_path);
    merge_and_write(&path, "mcpServers", mcp_servers)
}
