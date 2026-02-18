# Claude Manager

Application macOS native pour gérer votre environnement Claude Code : skills, MCP, hooks et plugins.

Construite avec **Tauri + Vue 3**.

## Fonctionnalités

### Skills
- Liste des skills globaux et par projet
- Recherche par nom ou description
- Éditeur WYSIWYG (Tiptap) avec conversion Markdown ↔ HTML
- Duplication, création de symlinks, suppression
- Détection automatique des symlinks et des projets liés
- Analyse automatique des skills au chargement (badges de statut)

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
- Rust 1.70+ (pour Tauri)
- macOS 10.13+

### Installation rapide

```bash
git clone https://github.com/wecoprod/claude-manager.git
cd claude-manager
npm install
npm run tauri dev
```

### Build de production

```bash
npm run tauri build
# L'app sera générée dans :
# src-tauri/target/release/bundle/macos/Claude Manager.app
```

## Stack technique

- **Frontend** : Vue 3 + TypeScript + Pinia + Tiptap + Marked.js + Turndown + DOMPurify
- **Backend** : Tauri 2.x + Rust (walkdir, serde, anyhow)
- **Build** : Vite + hot reload

## Structure du projet

```
claude-manager/
├── src/
│   ├── components/
│   │   ├── SkillList.vue
│   │   ├── SkillDetail.vue
│   │   ├── MarkdownEditor.vue
│   │   ├── SettingsDialog.vue
│   │   └── dialogs/
│   ├── stores/
│   ├── types/
│   └── main.ts
├── src-tauri/
│   ├── src/
│   │   ├── commands/
│   │   ├── models/
│   │   └── utils/
│   └── Cargo.toml
└── package.json
```

## Licence

MIT

---

Développé avec l'aide de [Claude Code](https://claude.com/claude-code)
