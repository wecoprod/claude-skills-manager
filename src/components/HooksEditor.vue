<template>
  <div class="hooks-editor">
    <div class="page-header">
      <div class="header-left">
        <h2><font-awesome-icon icon="bolt" /> Hooks</h2>
        <span class="subtitle">~/.claude/settings.json + projets</span>
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
        Commandes shell exécutées automatiquement lors d'événements Claude Code.
      </p>

      <div class="hooks-table-wrapper">
        <table class="hooks-table">
          <thead>
            <tr>
              <th>Source</th>
              <th>Événement</th>
              <th>Matcher (optionnel)</th>
              <th>Commande shell</th>
              <th></th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="hook in store.hooks" :key="hook.id">
              <td>
                <span :class="['source-badge', hook.source_label === 'Global' ? 'global' : 'project']">
                  <font-awesome-icon :icon="hook.source_label === 'Global' ? 'globe' : 'folder'" />
                  {{ hook.source_label }}
                </span>
              </td>
              <td>
                <select v-model="hook.event" class="form-select">
                  <option v-for="evt in HOOK_EVENTS" :key="evt" :value="evt">{{ evt }}</option>
                </select>
              </td>
              <td>
                <input
                  v-model="hook.matcher"
                  type="text"
                  class="form-input"
                  placeholder="ex: Bash"
                />
              </td>
              <td>
                <input
                  v-model="hook.command"
                  type="text"
                  class="form-input mono"
                  placeholder="ex: echo 'hook triggered'"
                />
              </td>
              <td>
                <button @click="removeHook(hook.id)" class="btn-icon danger" title="Supprimer">
                  <font-awesome-icon icon="trash" />
                </button>
              </td>
            </tr>
            <tr v-if="store.hooks.length === 0">
              <td colspan="5" class="empty-row">
                Aucun hook configuré. Cliquez sur "Ajouter" pour commencer.
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="add-row">
        <select v-model="newHookSource" class="form-select source-select">
          <option v-for="s in store.hooksSources" :key="s.path" :value="s.path">
            {{ s.label }}
          </option>
        </select>
        <button @click="addHook" class="btn btn-secondary">
          <font-awesome-icon icon="plus" /> Ajouter un hook
        </button>
      </div>

      <div v-if="saveSuccess" class="success-banner">
        <font-awesome-icon icon="check" /> Hooks sauvegardés avec succès
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
import { HOOK_EVENTS } from '../types/settings'

const store = useSettingsStore()
const skillsStore = useSkillsStore()
const saving = ref(false)
const saveSuccess = ref(false)
const newHookSource = ref('')

watch(() => store.hooksSources, (sources) => {
  if (sources.length > 0 && !newHookSource.value) {
    newHookSource.value = sources[0].path
  }
}, { immediate: true })

onMounted(() => {
  const projectPaths = skillsStore.projects.map(p => p.path)
  store.loadHooks(projectPaths)
})

function addHook() {
  const source = store.hooksSources.find(s => s.path === newHookSource.value)
  store.hooks.push({
    id: String(Date.now()),
    event: 'PreToolUse',
    matcher: '',
    command: '',
    source_path: source?.path ?? store.hooksSources[0]?.path ?? '',
    source_label: source?.label ?? store.hooksSources[0]?.label ?? 'Global',
  })
}

function removeHook(id: string) {
  const idx = store.hooks.findIndex(h => h.id === id)
  if (idx !== -1) store.hooks.splice(idx, 1)
}

async function handleSave() {
  saving.value = true
  saveSuccess.value = false
  try {
    await store.saveHooks()
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
.hooks-editor {
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

.hooks-table-wrapper {
  overflow-x: auto;
  margin-bottom: 1rem;
}

.hooks-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.875rem;
}

.hooks-table th {
  text-align: left;
  padding: 0.75rem 0.5rem;
  border-bottom: 2px solid #e0e0e0;
  color: #555;
  font-weight: 600;
  font-size: 0.8rem;
  text-transform: uppercase;
}

.hooks-table td {
  padding: 0.5rem;
  border-bottom: 1px solid #f0f0f0;
  vertical-align: middle;
}

.source-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  font-size: 0.75rem;
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

.form-select,
.form-input {
  width: 100%;
  padding: 0.4rem 0.5rem;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 0.875rem;
  font-family: inherit;
}

.form-input.mono {
  font-family: monospace;
}

.form-select:focus,
.form-input:focus {
  outline: none;
  border-color: #007bff;
}

.empty-row {
  text-align: center;
  color: #999;
  padding: 2rem !important;
  font-style: italic;
}

.add-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 1rem;
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
