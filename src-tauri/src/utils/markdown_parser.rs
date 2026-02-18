use crate::models::skill::SkillFrontmatter;
use anyhow::{Context, Result};
use regex::Regex;

/// Parse le frontmatter YAML d'un fichier Markdown
///
/// Format attendu :
/// ```
/// ---
/// name: Nom du skill
/// description: Description optionnelle
/// ---
///
/// Contenu Markdown...
/// ```
pub fn parse_frontmatter(content: &str) -> Result<(SkillFrontmatter, String)> {
    let content = content.trim();

    // Regex pour extraire frontmatter YAML
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$")
        .context("Regex compilation failed")?;

    if let Some(caps) = re.captures(content) {
        let yaml_str = caps.get(1)
            .context("Failed to extract YAML frontmatter")?
            .as_str();

        let markdown = caps.get(2)
            .context("Failed to extract markdown content")?
            .as_str()
            .to_string();

        // Parse YAML frontmatter
        let frontmatter: SkillFrontmatter = serde_yaml::from_str(yaml_str)
            .context("Failed to parse YAML frontmatter")?;

        Ok((frontmatter, markdown))
    } else {
        // Pas de frontmatter trouvé, créer un frontmatter par défaut
        Ok((
            SkillFrontmatter {
                name: "Untitled Skill".to_string(),
                description: None,
            },
            content.to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_frontmatter() {
        let content = r#"---
name: test-skill
description: Test description
---

# Markdown content
"#;
        let (frontmatter, markdown) = parse_frontmatter(content).unwrap();
        assert_eq!(frontmatter.name, "test-skill");
        assert_eq!(frontmatter.description, Some("Test description".to_string()));
        assert!(markdown.contains("# Markdown content"));
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let content = "Just markdown content";
        let (frontmatter, markdown) = parse_frontmatter(content).unwrap();
        assert_eq!(frontmatter.name, "Untitled Skill");
        assert_eq!(markdown, content);
    }

    #[test]
    fn test_parse_no_description() {
        let content = r#"---
name: simple-skill
---

Content here"#;
        let (frontmatter, _) = parse_frontmatter(content).unwrap();
        assert_eq!(frontmatter.name, "simple-skill");
        assert_eq!(frontmatter.description, None);
    }
}
