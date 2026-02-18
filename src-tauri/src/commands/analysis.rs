use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Issue {
    pub severity: String,   // "error" | "warning" | "info"
    pub category: String,   // "description" | "naming" | "structure" | "instructions" | "frontmatter"
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Suggestion {
    pub id: String,
    pub priority: String,   // "high" | "medium" | "low"
    pub category: String,
    pub title: String,
    pub description: String,
    pub current: String,
    pub suggested: String,
    pub impact: String,
    pub effort: String,
    pub checklist: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SkillStructure {
    pub has_skill_md: bool,
    pub has_readme: bool,
    pub folders: Vec<String>,
    pub total_size_kb: f64,
    pub instruction_word_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SkillAnalysis {
    pub path: String,
    pub name: String,
    pub status: String,   // "valid" | "warning" | "error"
    pub score: i32,
    pub category: String,
    pub frontmatter: Value,
    pub structure: SkillStructure,
    pub issues: Vec<Issue>,
    pub suggestions: Vec<Suggestion>,
}

/// Analyse un skill (dossier) et retourne un rapport détaillé
#[tauri::command]
pub async fn analyze_skill(path: String) -> Result<SkillAnalysis, String> {
    let folder = PathBuf::from(&path);

    if !folder.exists() || !folder.is_dir() {
        return Err(format!("Dossier introuvable: {}", path));
    }

    let name = folder
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let mut issues: Vec<Issue> = Vec::new();
    let mut suggestions: Vec<Suggestion> = Vec::new();
    let mut score: i32 = 100;

    // --- Structure ---
    let skill_md_path = folder.join("SKILL.md");
    let has_skill_md = skill_md_path.exists();
    let has_readme = folder.join("README.md").exists();

    let folders: Vec<String> = fs::read_dir(&folder)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();

    let total_size_kb = dir_size_kb(&folder);

    // --- Vérifications critiques ---

    // MISSING_SKILL_MD
    if !has_skill_md {
        score -= 30;
        issues.push(Issue {
            severity: "error".into(),
            category: "structure".into(),
            code: "MISSING_SKILL_MD".into(),
            message: "SKILL.md est absent du dossier skill".into(),
            current_value: None,
            location: Some("folder".into()),
        });
        suggestions.push(Suggestion {
            id: "add-skill-md".into(),
            priority: "high".into(),
            category: "structure".into(),
            title: "Créer le fichier SKILL.md".into(),
            description: "Chaque skill doit avoir un fichier SKILL.md à sa racine".into(),
            current: "Pas de SKILL.md trouvé".into(),
            suggested: format!("---\nname: {}\ndescription: ...\n---\n\n# Instructions\n\n...", name),
            impact: "Le skill ne sera pas chargé par Claude sans SKILL.md".into(),
            effort: "5 minutes".into(),
            checklist: vec![
                "Créer SKILL.md à la racine du dossier skill".into(),
                "Ajouter un frontmatter YAML avec name et description".into(),
                "Rédiger les instructions du skill".into(),
            ],
        });

        // Sans SKILL.md on ne peut pas aller plus loin sur le frontmatter
        let structure = SkillStructure {
            has_skill_md,
            has_readme,
            folders,
            total_size_kb,
            instruction_word_count: 0,
        };
        let status = compute_status(score);
        return Ok(SkillAnalysis {
            path,
            name,
            status,
            score: score.max(0),
            category: "unknown".into(),
            frontmatter: Value::Object(serde_json::Map::new()),
            structure,
            issues,
            suggestions,
        });
    }

    // README_FORBIDDEN
    if has_readme {
        score -= 30;
        issues.push(Issue {
            severity: "error".into(),
            category: "structure".into(),
            code: "README_FORBIDDEN".into(),
            message: "README.md est interdit dans un dossier skill (utiliser SKILL.md)".into(),
            current_value: Some("README.md présent".into()),
            location: Some("folder/README.md".into()),
        });
        suggestions.push(Suggestion {
            id: "remove-readme".into(),
            priority: "high".into(),
            category: "structure".into(),
            title: "Supprimer ou renommer README.md".into(),
            description: "Claude pourrait charger README.md au lieu de SKILL.md".into(),
            current: "README.md présent dans le dossier".into(),
            suggested: "Supprimer README.md ou fusionner son contenu dans SKILL.md".into(),
            impact: "Évite la confusion entre README.md et SKILL.md".into(),
            effort: "2 minutes".into(),
            checklist: vec![
                "Supprimer README.md".into(),
                "Ou déplacer son contenu pertinent dans SKILL.md".into(),
            ],
        });
    }

    // --- Parser le frontmatter ---
    let raw_content = fs::read_to_string(&skill_md_path)
        .map_err(|e| format!("Erreur lecture SKILL.md: {}", e))?;

    let instruction_word_count = count_words(&raw_content);

    let (frontmatter_value, yaml_valid) = parse_frontmatter_raw(&raw_content);

    if !yaml_valid {
        score -= 30;
        issues.push(Issue {
            severity: "error".into(),
            category: "frontmatter".into(),
            code: "INVALID_YAML".into(),
            message: "Le frontmatter YAML ne peut pas être parsé".into(),
            current_value: None,
            location: Some("frontmatter".into()),
        });
    }

    // --- Extraire les champs ---
    let fm_name = frontmatter_value.get("name").and_then(Value::as_str).unwrap_or("").to_string();
    let fm_description = frontmatter_value.get("description").and_then(Value::as_str).unwrap_or("").to_string();
    let fm_license = frontmatter_value.get("license").and_then(Value::as_str);
    let fm_compatibility = frontmatter_value.get("compatibility").and_then(Value::as_str);
    let fm_metadata = frontmatter_value.get("metadata");

    // MISSING_NAME
    if fm_name.is_empty() {
        score -= 30;
        issues.push(Issue {
            severity: "error".into(),
            category: "frontmatter".into(),
            code: "MISSING_NAME".into(),
            message: "Le champ name est obligatoire dans le frontmatter".into(),
            current_value: None,
            location: Some("frontmatter.name".into()),
        });
    } else {
        // INVALID_NAME_FORMAT (espaces ou majuscules)
        if fm_name.chars().any(|c| c.is_uppercase() || c == ' ') {
            score -= 30;
            issues.push(Issue {
                severity: "error".into(),
                category: "naming".into(),
                code: "INVALID_NAME_FORMAT".into(),
                message: "Le nom doit être en kebab-case (minuscules, tirets uniquement)".into(),
                current_value: Some(fm_name.clone()),
                location: Some("frontmatter.name".into()),
            });
            suggestions.push(Suggestion {
                id: "fix-name-format".into(),
                priority: "high".into(),
                category: "naming".into(),
                title: "Corriger le format du nom".into(),
                description: "Le nom doit être en kebab-case pour être reconnu correctement".into(),
                current: format!("name: {}", fm_name),
                suggested: format!("name: {}", to_kebab_case(&fm_name)),
                impact: "Format requis par Claude Code pour identifier les skills".into(),
                effort: "1 minute".into(),
                checklist: vec![
                    "Remplacer les espaces par des tirets".into(),
                    "Mettre tout en minuscules".into(),
                ],
            });
        }

        // RESERVED_NAME
        if fm_name.starts_with("claude-") || fm_name.starts_with("anthropic-") {
            score -= 30;
            issues.push(Issue {
                severity: "error".into(),
                category: "naming".into(),
                code: "RESERVED_NAME".into(),
                message: "Les préfixes 'claude-' et 'anthropic-' sont réservés".into(),
                current_value: Some(fm_name.clone()),
                location: Some("frontmatter.name".into()),
            });
        }
    }

    // XML_IN_FRONTMATTER
    let fm_str = extract_frontmatter_string(&raw_content);
    if fm_str.contains('<') && fm_str.contains('>') {
        score -= 30;
        issues.push(Issue {
            severity: "error".into(),
            category: "frontmatter".into(),
            code: "XML_IN_FRONTMATTER".into(),
            message: "Les balises XML < > sont interdites dans le frontmatter".into(),
            current_value: None,
            location: Some("frontmatter".into()),
        });
    }

    // MISSING_DESCRIPTION
    if fm_description.is_empty() {
        score -= 30;
        issues.push(Issue {
            severity: "error".into(),
            category: "description".into(),
            code: "MISSING_DESCRIPTION".into(),
            message: "Le champ description est obligatoire dans le frontmatter".into(),
            current_value: None,
            location: Some("frontmatter.description".into()),
        });
        suggestions.push(Suggestion {
            id: "add-description".into(),
            priority: "high".into(),
            category: "description".into(),
            title: "Ajouter une description".into(),
            description: "La description est cruciale pour que Claude détecte automatiquement ce skill".into(),
            current: "description: (manquant)".into(),
            suggested: format!("description: Implements {} features. Use when ... Triggers on '...', '...'.", name),
            impact: "Sans description, Claude ne peut pas charger ce skill automatiquement".into(),
            effort: "5 minutes".into(),
            checklist: vec![
                "Ajouter description: dans le frontmatter".into(),
                "Commencer par un verbe d'action".into(),
                "Inclure 'Use when...' avec cas d'usage".into(),
                "Lister des trigger phrases".into(),
            ],
        });
    } else {
        // DESCRIPTION_TOO_LONG
        if fm_description.len() > 1024 {
            score -= 10;
            issues.push(Issue {
                severity: "warning".into(),
                category: "description".into(),
                code: "DESCRIPTION_TOO_LONG".into(),
                message: format!("Description trop longue ({} chars, max 1024)", fm_description.len()),
                current_value: Some(format!("{} caractères", fm_description.len())),
                location: Some("frontmatter.description".into()),
            });
        }

        // DESCRIPTION_TOO_VAGUE
        let is_vague = fm_description.len() < 50 || is_generic_description(&fm_description);
        if is_vague {
            score -= 10;
            issues.push(Issue {
                severity: "warning".into(),
                category: "description".into(),
                code: "DESCRIPTION_TOO_VAGUE".into(),
                message: "Description trop vague — manque de spécificité ou de trigger phrases".into(),
                current_value: Some(fm_description.clone()),
                location: Some("frontmatter.description".into()),
            });
            suggestions.push(Suggestion {
                id: "improve-description-specificity".into(),
                priority: "high".into(),
                category: "description".into(),
                title: "Rendre la description plus spécifique".into(),
                description: "Ajouter un verbe d'action, 'Use when...', et des trigger phrases".into(),
                current: format!("description: {}", fm_description),
                suggested: format!("description: Implements {} features following project patterns. Use when creating new features or refactoring. Triggers on 'new feature', 'implement', 'create'.", name),
                impact: "Claude détectera automatiquement ce skill dans les bons contextes".into(),
                effort: "5 minutes".into(),
                checklist: vec![
                    "Ajouter un verbe d'action précis (implements, generates, analyzes...)".into(),
                    "Inclure 'Use when...' avec 3-4 cas d'usage concrets".into(),
                    "Lister 4-5 trigger phrases entre guillemets".into(),
                    "Viser 80-200 caractères".into(),
                ],
            });
        } else {
            // DESCRIPTION_MISSING_WHAT
            if !has_action_verb(&fm_description) {
                score -= 10;
                issues.push(Issue {
                    severity: "warning".into(),
                    category: "description".into(),
                    code: "DESCRIPTION_MISSING_WHAT".into(),
                    message: "La description devrait commencer par un verbe d'action".into(),
                    current_value: Some(fm_description.clone()),
                    location: Some("frontmatter.description".into()),
                });
            }

            // DESCRIPTION_MISSING_WHEN
            if !fm_description.to_lowercase().contains("use when")
                && !fm_description.to_lowercase().contains("when to use")
                && !fm_description.to_lowercase().contains("quand")
            {
                score -= 10;
                issues.push(Issue {
                    severity: "warning".into(),
                    category: "description".into(),
                    code: "DESCRIPTION_MISSING_WHEN".into(),
                    message: "La description devrait indiquer quand utiliser ce skill ('Use when...')".into(),
                    current_value: Some(fm_description.clone()),
                    location: Some("frontmatter.description".into()),
                });
            }

            // DESCRIPTION_MISSING_TRIGGERS
            if !fm_description.contains('\'') && !fm_description.contains('"') {
                score -= 10;
                issues.push(Issue {
                    severity: "warning".into(),
                    category: "description".into(),
                    code: "DESCRIPTION_MISSING_TRIGGERS".into(),
                    message: "La description manque de trigger phrases (ex: 'new feature', 'create controller')".into(),
                    current_value: Some(fm_description.clone()),
                    location: Some("frontmatter.description".into()),
                });
            }
        }
    }

    // MISSING_CATEGORY
    let category_value = fm_metadata
        .and_then(|m| m.get("category"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    if category_value.is_empty() {
        score -= 10;
        issues.push(Issue {
            severity: "warning".into(),
            category: "frontmatter".into(),
            code: "MISSING_CATEGORY".into(),
            message: "metadata.category n'est pas défini".into(),
            current_value: None,
            location: Some("frontmatter.metadata.category".into()),
        });
        suggestions.push(Suggestion {
            id: "add-metadata".into(),
            priority: "medium".into(),
            category: "frontmatter".into(),
            title: "Ajouter les métadonnées".into(),
            description: "Enrichir le frontmatter avec category, version, author".into(),
            current: format!("---\nname: {}\ndescription: ...\n---", fm_name),
            suggested: format!("---\nname: {}\ndescription: ...\nmetadata:\n  category: workflow-automation\n  version: 1.0.0\n  author: ...\n---", fm_name),
            impact: "Meilleure organisation et découvrabilité du skill".into(),
            effort: "2 minutes".into(),
            checklist: vec![
                "Ajouter metadata: dans le frontmatter".into(),
                "Définir category: (workflow-automation | document-creation | mcp-enhancement)".into(),
                "Ajouter version: 1.0.0".into(),
                "Ajouter author: votre nom".into(),
            ],
        });
    }

    // INSTRUCTIONS_TOO_LONG
    if instruction_word_count > 5000 {
        score -= 10;
        issues.push(Issue {
            severity: "warning".into(),
            category: "instructions".into(),
            code: "INSTRUCTIONS_TOO_LONG".into(),
            message: format!("Instructions trop longues ({} mots, max 5000)", instruction_word_count),
            current_value: Some(format!("{} mots", instruction_word_count)),
            location: Some("instructions".into()),
        });
        suggestions.push(Suggestion {
            id: "extract-to-references".into(),
            priority: "low".into(),
            category: "optimization".into(),
            title: "Utiliser la progressive disclosure".into(),
            description: "Extraire les détails techniques vers references/ pour réduire la taille de SKILL.md".into(),
            current: format!("SKILL.md contient {} mots", instruction_word_count),
            suggested: "Créer references/details.md et y déplacer les détails techniques".into(),
            impact: "Réduction de la consommation de tokens".into(),
            effort: "20 minutes".into(),
            checklist: vec![
                "Créer un dossier references/".into(),
                "Déplacer les détails API/techniques vers references/".into(),
                "Référencer depuis SKILL.md: 'See references/details.md'".into(),
                "Garder SKILL.md < 3000 mots".into(),
            ],
        });
    }

    // MISSING_EXAMPLES
    let lower_content = raw_content.to_lowercase();
    if !lower_content.contains("## examples") && !lower_content.contains("## exemples") {
        score -= 10;
        issues.push(Issue {
            severity: "warning".into(),
            category: "instructions".into(),
            code: "MISSING_EXAMPLES".into(),
            message: "Aucune section ## Examples trouvée".into(),
            current_value: None,
            location: Some("instructions".into()),
        });
        suggestions.push(Suggestion {
            id: "add-examples".into(),
            priority: "medium".into(),
            category: "instructions".into(),
            title: "Ajouter une section Examples".into(),
            description: "Documenter 2-3 cas d'usage concrets avec format standardisé".into(),
            current: "Pas de section ## Examples".into(),
            suggested: "## Examples\n\n### Example 1: ...\nUser says: \"...\"\n\nActions:\n1. ...\n\nResult: ...".into(),
            impact: "Claude comprendra mieux les cas d'usage et fournira des résultats cohérents".into(),
            effort: "10 minutes".into(),
            checklist: vec![
                "Ajouter ## Examples après ## Instructions".into(),
                "Documenter 2-3 scénarios réels".into(),
                "Format: User says → Actions → Result".into(),
            ],
        });
    }

    // MISSING_TROUBLESHOOTING
    if !lower_content.contains("## troubleshooting") && !lower_content.contains("## dépannage") {
        score -= 10;
        issues.push(Issue {
            severity: "warning".into(),
            category: "instructions".into(),
            code: "MISSING_TROUBLESHOOTING".into(),
            message: "Aucune section ## Troubleshooting trouvée".into(),
            current_value: None,
            location: Some("instructions".into()),
        });
    }

    // --- Infos ---

    // MISSING_LICENSE
    if fm_license.is_none() {
        score -= 5;
        issues.push(Issue {
            severity: "info".into(),
            category: "frontmatter".into(),
            code: "MISSING_LICENSE".into(),
            message: "Aucun champ license défini".into(),
            current_value: None,
            location: Some("frontmatter.license".into()),
        });
    }

    // MISSING_VERSION
    if fm_metadata.and_then(|m| m.get("version")).is_none() {
        score -= 5;
        issues.push(Issue {
            severity: "info".into(),
            category: "frontmatter".into(),
            code: "MISSING_VERSION".into(),
            message: "metadata.version n'est pas défini".into(),
            current_value: None,
            location: Some("frontmatter.metadata.version".into()),
        });
    }

    // MISSING_AUTHOR
    if fm_metadata.and_then(|m| m.get("author")).is_none() {
        score -= 5;
        issues.push(Issue {
            severity: "info".into(),
            category: "frontmatter".into(),
            code: "MISSING_AUTHOR".into(),
            message: "metadata.author n'est pas défini".into(),
            current_value: None,
            location: Some("frontmatter.metadata.author".into()),
        });
    }

    // MISSING_COMPATIBILITY
    if fm_compatibility.is_none() {
        score -= 5;
        issues.push(Issue {
            severity: "info".into(),
            category: "frontmatter".into(),
            code: "MISSING_COMPATIBILITY".into(),
            message: "Le champ compatibility n'est pas défini".into(),
            current_value: None,
            location: Some("frontmatter.compatibility".into()),
        });
    }

    // NO_STRUCTURE_FOLDERS
    let has_structure = folders.iter().any(|f| {
        matches!(f.as_str(), "scripts" | "references" | "assets")
    });
    if !has_structure {
        score -= 5;
        issues.push(Issue {
            severity: "info".into(),
            category: "structure".into(),
            code: "NO_STRUCTURE_FOLDERS".into(),
            message: "Pas de dossiers scripts/, references/, ou assets/ trouvés".into(),
            current_value: None,
            location: Some("folder".into()),
        });
    }

    let detected_category = if !category_value.is_empty() {
        category_value
    } else {
        detect_category(&fm_name, &fm_description)
    };

    let structure = SkillStructure {
        has_skill_md,
        has_readme,
        folders,
        total_size_kb,
        instruction_word_count,
    };

    let status = compute_status(score);

    Ok(SkillAnalysis {
        path,
        name,
        status,
        score: score.max(0),
        category: detected_category,
        frontmatter: frontmatter_value,
        structure,
        issues,
        suggestions,
    })
}

fn compute_status(score: i32) -> String {
    if score >= 80 {
        "valid".into()
    } else if score >= 50 {
        "warning".into()
    } else {
        "error".into()
    }
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn dir_size_kb(path: &PathBuf) -> f64 {
    let bytes: u64 = walkdir::WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|m| m.is_file())
        .map(|m| m.len())
        .sum();
    bytes as f64 / 1024.0
}

fn is_generic_description(desc: &str) -> bool {
    let lower = desc.to_lowercase();
    let generic_phrases = [
        "helps with",
        "used for",
        "a tool",
        "this skill",
        "assistant",
        "aide",
        "outil",
    ];
    generic_phrases.iter().any(|p| lower.contains(p)) && desc.len() < 100
}

fn has_action_verb(desc: &str) -> bool {
    let first_word = desc.split_whitespace().next().unwrap_or("").to_lowercase();
    let action_verbs = [
        "implements", "generates", "creates", "analyzes", "manages",
        "builds", "converts", "transforms", "validates", "extracts",
        "implémente", "génère", "crée", "analyse", "gère", "construit",
    ];
    action_verbs.contains(&first_word.as_str())
}

fn parse_frontmatter_raw(content: &str) -> (Value, bool) {
    let content = content.trim();
    if !content.starts_with("---") {
        return (Value::Object(serde_json::Map::new()), true);
    }

    let end = content[3..].find("\n---");
    if let Some(pos) = end {
        let yaml_str = &content[3..pos + 3];
        match serde_yaml::from_str::<Value>(yaml_str) {
            Ok(v) => (v, true),
            Err(_) => (Value::Object(serde_json::Map::new()), false),
        }
    } else {
        (Value::Object(serde_json::Map::new()), false)
    }
}

fn extract_frontmatter_string(content: &str) -> String {
    let content = content.trim();
    if !content.starts_with("---") {
        return String::new();
    }
    let end = content[3..].find("\n---");
    end.map(|pos| content[3..pos + 3].to_string())
        .unwrap_or_default()
}

fn to_kebab_case(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn detect_category(name: &str, description: &str) -> String {
    let text = format!("{} {}", name, description).to_lowercase();
    if text.contains("mcp") || text.contains("plugin") || text.contains("server") {
        "mcp-enhancement".into()
    } else if text.contains("doc") || text.contains("report") || text.contains("document")
        || text.contains("rapport")
    {
        "document-creation".into()
    } else {
        "workflow-automation".into()
    }
}
