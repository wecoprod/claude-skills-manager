use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub global_skills_path: String,
    pub projects_base_path: String,
    pub custom_project_paths: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            global_skills_path: String::from("~/.claude/skills"),
            projects_base_path: String::from("~/Projects"),
            custom_project_paths: Vec::new(),
        }
    }
}

impl Config {
    /// Expanse le tilde (~) dans un chemin
    pub fn expand_tilde(path: &str) -> String {
        if path.starts_with("~/") {
            let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
            path.replacen("~", &home, 1)
        } else if path == "~" {
            std::env::var("HOME").unwrap_or_else(|_| String::from("/tmp"))
        } else {
            path.to_string()
        }
    }

    /// Récupère le chemin du fichier de configuration
    pub fn config_file_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("~"));
        PathBuf::from(home)
            .join(".config")
            .join("claude-manager")
            .join("config.json")
    }

    /// Retourne le chemin des skills globaux avec ~ expansé
    pub fn expanded_global_skills_path(&self) -> String {
        Self::expand_tilde(&self.global_skills_path)
    }

    /// Retourne le chemin de base des projets avec ~ expansé
    pub fn expanded_projects_base_path(&self) -> String {
        Self::expand_tilde(&self.projects_base_path)
    }

    /// Retourne tous les chemins de projets avec ~ expansé
    pub fn expanded_custom_project_paths(&self) -> Vec<String> {
        self.custom_project_paths
            .iter()
            .map(|p| Self::expand_tilde(p))
            .collect()
    }

    /// Charge la configuration depuis le fichier, ou retourne la config par défaut
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_file_path();

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Sauvegarde la configuration dans le fichier
    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::config_file_path();

        // Créer le répertoire parent si nécessaire
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&config_path, content)?;

        Ok(())
    }

    /// Retourne tous les chemins de projets (base + custom) avec ~ expansé
    pub fn all_project_paths(&self) -> Vec<String> {
        let mut paths = vec![self.expanded_projects_base_path()];
        paths.extend(self.expanded_custom_project_paths());
        paths
    }
}
