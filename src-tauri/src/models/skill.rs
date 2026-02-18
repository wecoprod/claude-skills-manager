use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFrontmatter {
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,              // Hash du path
    pub name: String,            // frontmatter "name"
    pub description: String,     // frontmatter "description" (vide si None)
    pub path: String,            // Chemin absolu du fichier
    pub source: SkillSource,     // Global ou Project
    pub project_name: Option<String>, // Nom du projet si source = Project
    pub content: String,         // Markdown sans frontmatter
    pub last_modified: u64,      // Timestamp
    pub linked_projects: Vec<String>, // Liste des projets qui utilisent ce skill (symlinks)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillSource {
    Global,
    Project,
}

impl Skill {
    /// Génère un ID unique basé sur le hash SHA256 du chemin
    pub fn generate_id(path: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(path.as_bytes());
        format!("{:x}", hasher.finalize())[..16].to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub path: String,
    pub skills_path: String,
    pub skill_count: usize,
}
