<template>
  <div class="plugins-editor">
    <div class="page-header">
      <div class="header-left">
        <h2><font-awesome-icon icon="puzzle-piece" /> Plugins Claude Code</h2>
        <span class="subtitle">~/.claude/settings.json → enabledPlugins</span>
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
        Activez ou désactivez les plugins Claude Code disponibles dans vos sessions.
      </p>

      <div class="plugins-list">
        <div v-for="(enabled, key) in store.plugins" :key="key" class="plugin-row">
          <label class="plugin-label">
            <input
              type="checkbox"
              :checked="enabled"
              @change="store.plugins[key] = ($event.target as HTMLInputElement).checked"
              class="plugin-toggle"
            />
            <div class="plugin-info">
              <span class="plugin-name">{{ pluginName(String(key)) }}</span>
              <span class="plugin-registry">{{ pluginRegistry(String(key)) }}</span>
            </div>
          </label>
          <button @click="removePlugin(String(key))" class="btn-icon danger" title="Supprimer">
            <font-awesome-icon icon="trash" />
          </button>
        </div>
        <div v-if="Object.keys(store.plugins).length === 0" class="empty">
          <font-awesome-icon icon="puzzle-piece" /> Aucun plugin configuré
        </div>
      </div>

      <div class="add-row">
        <input
          v-model="newPluginKey"
          type="text"
          class="form-input"
          placeholder="ex: figma@claude-plugins-official"
          @keyup.enter="addPlugin"
        />
        <button @click="addPlugin" class="btn btn-secondary" :disabled="!newPluginKey.trim()">
          <font-awesome-icon icon="plus" /> Ajouter
        </button>
      </div>

      <div v-if="saveSuccess" class="success-banner">
        <font-awesome-icon icon="check" /> Plugins sauvegardés avec succès
      </div>

      <div v-if="store.error" class="error-banner">
        <font-awesome-icon icon="triangle-exclamation" /> {{ store.error }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSettingsStore } from '../stores/settingsStore'

const store = useSettingsStore()
const saving = ref(false)
const saveSuccess = ref(false)
const newPluginKey = ref('')

onMounted(() => {
  store.loadPlugins()
})

function pluginName(key: string): string {
  return key.split('@')[0]
}

function pluginRegistry(key: string): string {
  const parts = key.split('@')
  return parts.length > 1 ? `@${parts[1]}` : ''
}

function addPlugin() {
  const key = newPluginKey.value.trim()
  if (!key) return
  store.plugins[key] = true
  newPluginKey.value = ''
}

function removePlugin(key: string) {
  delete store.plugins[key]
}

async function handleSave() {
  saving.value = true
  saveSuccess.value = false
  try {
    await store.savePlugins()
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
.plugins-editor {
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
}

.plugins-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.plugin-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  background: #f8f9fa;
  border-radius: 6px;
  border: 1px solid #e0e0e0;
  transition: border-color 0.2s;
}

.plugin-row:hover {
  border-color: #007bff;
}

.plugin-label {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  cursor: pointer;
  flex: 1;
}

.plugin-toggle {
  width: 16px;
  height: 16px;
  cursor: pointer;
  accent-color: #007bff;
  flex-shrink: 0;
}

.plugin-info {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.plugin-name {
  font-weight: 600;
  font-size: 0.9rem;
  color: #333;
}

.plugin-registry {
  font-size: 0.75rem;
  color: #888;
  font-family: monospace;
}

.empty {
  padding: 2rem;
  text-align: center;
  color: #999;
  font-size: 1rem;
}

.add-row {
  display: flex;
  gap: 0.5rem;
}

.form-input {
  flex: 1;
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

.btn-secondary:hover:not(:disabled) {
  background: #e0e0e0;
}

.btn-secondary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
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
