<template>
  <div class="mcp-editor">
    <div class="page-header">
      <div class="header-left">
        <h2><font-awesome-icon icon="plug" /> Serveurs MCP</h2>
        <span class="subtitle">~/.claude.json + projets</span>
      </div>
      <div class="header-actions">
        <button @click="handleSave" class="btn btn-primary" :disabled="saving">
          <font-awesome-icon icon="floppy-disk" />
          {{ saving ? 'Sauvegarde...' : 'Sauvegarder' }}
        </button>
      </div>
    </div>

    <div v-if="store.loading" class="loading">Chargement...</div>

    <div v-else class="content">
      <p class="description">
        Configurez les serveurs MCP (Model Context Protocol) disponibles dans Claude Code.
      </p>

      <div class="servers-list">
        <div v-for="server in store.mcpServers" :key="server.name" class="server-card">
          <div class="server-header" @click="toggleServer(server.name)">
            <div class="server-title">
              <font-awesome-icon icon="plug" class="server-icon" />
              <span class="server-name">{{ server.name || 'Nouveau serveur' }}</span>
              <span :class="['source-badge', server.source_label === 'Global' ? 'global' : 'project']">
                <font-awesome-icon :icon="server.source_label === 'Global' ? 'globe' : 'folder'" />
                {{ server.source_label }}
              </span>
              <span v-if="server.command" class="server-preview">{{ server.command }} {{ server.args.slice(0, 2).join(' ') }}</span>
            </div>
            <div class="server-actions">
              <button @click.stop="removeServer(server.name)" class="btn-icon danger" title="Supprimer">
                <font-awesome-icon icon="trash" />
              </button>
              <font-awesome-icon :icon="expandedServers.has(server.name) ? 'chevron-up' : 'chevron-down'" class="chevron" />
            </div>
          </div>

          <div v-if="expandedServers.has(server.name)" class="server-form">
            <div class="form-row">
              <div class="form-group">
                <label>Nom</label>
                <input
                  :value="server.name"
                  @input="renameServer(server.name, ($event.target as HTMLInputElement).value)"
                  type="text"
                  class="form-input"
                  placeholder="ex: filesystem"
                />
              </div>
              <div class="form-group">
                <label>Commande</label>
                <input
                  v-model="server.command"
                  type="text"
                  class="form-input"
                  placeholder="ex: npx"
                />
              </div>
            </div>

            <div class="form-group">
              <label>Arguments (un par ligne)</label>
              <textarea
                :value="server.args.join('\n')"
                @input="server.args = ($event.target as HTMLTextAreaElement).value.split('\n').filter(a => a.trim())"
                class="form-textarea"
                placeholder="-y&#10;@modelcontextprotocol/server-filesystem"
                rows="3"
              ></textarea>
            </div>

            <div class="form-group">
              <label>Variables d'environnement (KEY=value, un par ligne)</label>
              <textarea
                :value="Object.entries(server.env).map(([k, v]) => `${k}=${v}`).join('\n')"
                @input="server.env = parseEnv(($event.target as HTMLTextAreaElement).value)"
                class="form-textarea"
                placeholder="API_KEY=abc123"
                rows="2"
              ></textarea>
            </div>
          </div>
        </div>

        <div v-if="store.mcpServers.length === 0" class="empty">
          <font-awesome-icon icon="plug" /> Aucun serveur MCP configuré
        </div>
      </div>

      <div class="add-row">
        <select v-model="newServerSource" class="form-select source-select">
          <option v-for="s in store.mcpSources" :key="s.path" :value="s.path">
            {{ s.label }}
          </option>
        </select>
        <button @click="addServer" class="btn btn-secondary">
          <font-awesome-icon icon="plus" /> Ajouter un serveur
        </button>
      </div>

      <div v-if="saveSuccess" class="success-banner">
        <font-awesome-icon icon="check" /> Serveurs MCP sauvegardés avec succès
      </div>

      <div v-if="store.error" class="error-banner">
        <font-awesome-icon icon="triangle-exclamation" /> {{ store.error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useSettingsStore } from '../stores/settingsStore'
import { useSkillsStore } from '../stores/skillsStore'

const store = useSettingsStore()
const skillsStore = useSkillsStore()
const saving = ref(false)
const saveSuccess = ref(false)
const expandedServers = ref<Set<string>>(new Set())
const newServerSource = ref('')

watch(() => store.mcpSources, (sources) => {
  if (sources.length > 0 && !newServerSource.value) {
    newServerSource.value = sources[0].path
  }
}, { immediate: true })

onMounted(() => {
  const projectPaths = skillsStore.projects.map(p => p.path)
  store.loadMcp(projectPaths)
})

function toggleServer(name: string) {
  if (expandedServers.value.has(name)) {
    expandedServers.value.delete(name)
  } else {
    expandedServers.value.add(name)
  }
}

function addServer() {
  const name = `nouveau-serveur-${Date.now()}`
  const source = store.mcpSources.find(s => s.path === newServerSource.value) ?? store.mcpSources[0]
  store.mcpServers.push({
    name,
    command: '',
    args: [],
    env: {},
    source_path: source?.path ?? '',
    source_label: source?.label ?? 'Global',
  })
  expandedServers.value.add(name)
}

function removeServer(name: string) {
  const idx = store.mcpServers.findIndex(s => s.name === name)
  if (idx !== -1) {
    store.mcpServers.splice(idx, 1)
    expandedServers.value.delete(name)
  }
}

function renameServer(oldName: string, newName: string) {
  const server = store.mcpServers.find(s => s.name === oldName)
  if (server) {
    if (expandedServers.value.has(oldName)) {
      expandedServers.value.delete(oldName)
      expandedServers.value.add(newName)
    }
    server.name = newName
  }
}

function parseEnv(text: string): Record<string, string> {
  const env: Record<string, string> = {}
  for (const line of text.split('\n')) {
    const eq = line.indexOf('=')
    if (eq > 0) {
      env[line.slice(0, eq).trim()] = line.slice(eq + 1).trim()
    }
  }
  return env
}

async function handleSave() {
  saving.value = true
  saveSuccess.value = false
  try {
    await store.saveMcp()
    saveSuccess.value = true
    setTimeout(() => { saveSuccess.value = false }, 3000)
  } catch {
    // error is set in store
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.mcp-editor {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: white;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e0e0e0;
  background: #f8f9fa;
}

.header-left h2 {
  margin: 0 0 0.25rem 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
}

.subtitle {
  font-size: 0.8rem;
  color: #888;
  font-family: monospace;
}

.header-actions {
  display: flex;
  gap: 0.75rem;
}

.content {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.description {
  margin: 0 0 1.5rem 0;
  color: #666;
  font-size: 0.875rem;
  line-height: 1.5;
}

.servers-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.server-card {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  overflow: hidden;
}

.server-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: #f8f9fa;
  cursor: pointer;
  transition: background 0.2s;
}

.server-header:hover {
  background: #f0f0f0;
}

.server-title {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  min-width: 0;
}

.server-icon {
  color: #007bff;
  flex-shrink: 0;
}

.server-name {
  font-weight: 600;
  color: #333;
  font-size: 0.9rem;
}

.server-preview {
  font-family: monospace;
  font-size: 0.8rem;
  color: #888;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.server-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-shrink: 0;
}

.chevron {
  color: #666;
  font-size: 0.875rem;
}

.server-form {
  padding: 1rem;
  border-top: 1px solid #e0e0e0;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.75rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.form-group label {
  font-size: 0.8rem;
  font-weight: 600;
  color: #555;
}

.form-input {
  padding: 0.5rem 0.625rem;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 0.875rem;
  font-family: monospace;
}

.form-input:focus {
  outline: none;
  border-color: #007bff;
}

.form-textarea {
  padding: 0.5rem 0.625rem;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 0.8rem;
  font-family: monospace;
  resize: vertical;
}

.form-textarea:focus {
  outline: none;
  border-color: #007bff;
}

.empty {
  padding: 3rem;
  text-align: center;
  color: #999;
  font-size: 1rem;
}

.source-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  font-size: 0.7rem;
  font-weight: 500;
  white-space: nowrap;
}

.source-badge.global {
  background: #e3f2fd;
  color: #1976d2;
}

.source-badge.project {
  background: #f3e5f5;
  color: #7b1fa2;
}

.add-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.form-select {
  padding: 0.5rem 0.625rem;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 0.875rem;
  font-family: inherit;
}

.form-select:focus {
  outline: none;
  border-color: #007bff;
}

.source-select {
  width: auto;
  min-width: 140px;
}

.btn-icon {
  padding: 0.4rem 0.5rem;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  transition: all 0.2s;
  color: #666;
}

.btn-icon.danger:hover {
  background: #fff5f5;
  border-color: #dc3545;
  color: #dc3545;
}

.btn {
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 4px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: #f0f0f0;
  color: #333;
  border: 1px solid #d0d0d0;
}

.btn-secondary:hover {
  background: #e0e0e0;
}

.success-banner {
  margin-top: 1rem;
  padding: 0.75rem 1rem;
  background: #d4edda;
  color: #155724;
  border-radius: 4px;
  font-size: 0.875rem;
}

.error-banner {
  margin-top: 1rem;
  padding: 0.75rem 1rem;
  background: #f8d7da;
  color: #721c24;
  border-radius: 4px;
  font-size: 0.875rem;
}

.loading {
  padding: 2rem;
  text-align: center;
  color: #666;
}

</style>
