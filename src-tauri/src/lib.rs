mod commands;
mod models;
mod utils;

use commands::{analysis, config, projects, settings, skills};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            skills::list_skills,
            skills::list_commands,
            skills::get_skill,
            skills::create_skill_symlink,
            skills::duplicate_skill,
            skills::delete_skill,
            skills::update_skill,
            projects::scan_projects,
            config::load_config,
            config::save_config,
            config::get_config_path,
            settings::load_hooks,
            settings::save_hooks,
            settings::load_mcp,
            settings::save_mcp,
            settings::load_plugins,
            settings::save_plugins,
            analysis::analyze_skill,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
