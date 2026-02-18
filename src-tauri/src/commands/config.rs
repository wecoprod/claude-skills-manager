use crate::models::config::Config;

/// Charge la configuration
#[tauri::command]
pub async fn load_config() -> Result<Config, String> {
    Config::load().map_err(|e| format!("Erreur chargement config: {}", e))
}

/// Sauvegarde la configuration
#[tauri::command]
pub async fn save_config(config: Config) -> Result<(), String> {
    config.save().map_err(|e| format!("Erreur sauvegarde config: {}", e))?;

    #[cfg(debug_assertions)]
    eprintln!("⚙️ Configuration sauvegardée: {:?}", Config::config_file_path());

    Ok(())
}

/// Récupère le chemin du fichier de configuration
#[tauri::command]
pub async fn get_config_path() -> Result<String, String> {
    Ok(Config::config_file_path()
        .to_string_lossy()
        .to_string())
}
