# Claude Manager

Application macOS native pour gérer votre environnement Claude Code : skills, commandes, MCP et hooks.

Construite avec **Tauri 2 + Vue 3**.

## Fonctionnalités

### Skills
- Liste des skills globaux et par projet
- Recherche par nom ou description
- Éditeur WYSIWYG (Tiptap) avec conversion Markdown ↔ HTML
- Duplication, création de symlinks, suppression
- Détection automatique des symlinks et des projets liés
- Analyse automatique des skills au chargement (badges de statut)

### Commandes
- Visualisation des commandes globales et par projet

### MCP Servers
- Visualisation des serveurs MCP globaux et par projet
- Gestion des plugins

### Hooks
- Visualisation des hooks globaux et par projet

### Configuration
- Chemins configurables (skills, projets, dossiers supplémentaires)
- Support tilde (`~`)
- Fichier de config : `~/.config/claude-manager/config.json`

## Installation

### Prérequis

- Node.js 18+ et npm
- Rust 1.70+
- macOS 10.13+

### Développement

```bash
git clone https://github.com/wecoprod/claude-manager.git
cd claude-manager
npm install
source ~/.cargo/env && npm run tauri dev
```

### Build de production

```bash
npm run tauri build
# Génère : src-tauri/target/release/bundle/macos/Claude Manager.app
```

## Stack technique

- **Frontend** : Vue 3 + TypeScript + Pinia + Tiptap + Marked.js + Turndown + DOMPurify
- **Backend** : Tauri 2.x + Rust (walkdir, serde, anyhow)
- **Build** : Vite

## Structure du projet

```
claude-manager/
├── src/
│   ├── components/
│   │   ├── AppNav.vue          # Navigation principale
│   │   ├── SkillList.vue       # Liste + recherche
│   │   ├── SkillDetail.vue     # Détail + actions
│   │   ├── SkillCard.vue       # Carte skill
│   │   ├── MarkdownEditor.vue  # Éditeur WYSIWYG Tiptap
│   │   ├── CommandList.vue     # Liste des commandes
│   │   ├── McpEditor.vue       # Éditeur MCP servers
│   │   ├── PluginsEditor.vue   # Éditeur plugins
│   │   ├── HooksEditor.vue     # Éditeur hooks
│   │   ├── SettingsDialog.vue  # Configuration
│   │   ├── ConfirmDialog.vue
│   │   ├── DuplicateDialog.vue
│   │   └── SymlinkDialog.vue
│   ├── stores/
│   │   ├── skillsStore.ts      # State skills, projets, config
│   │   └── settingsStore.ts    # State settings globaux/projet
│   └── types/
│       ├── skill.ts
│       └── settings.ts
└── src-tauri/
    └── src/
        ├── commands/           # Commandes Tauri exposées au frontend
        ├── models/             # Structures de données Rust
        └── utils/              # Parseur markdown, file watcher
```

## Licence

MIT

---

Développé avec l'aide de [Claude Code](https://claude.com/claude-code)
