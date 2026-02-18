pub mod analysis;
pub mod config;
pub mod projects;
pub mod settings;
pub mod skills;

// Constantes partagées pour les chemins
pub const CLAUDE_DIR: &str = ".claude";
pub const SKILLS_DIR: &str = "skills";
pub const COMMANDS_DIR: &str = "commands";
pub const MAX_SCAN_DEPTH: usize = 3;
