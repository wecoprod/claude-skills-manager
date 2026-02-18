<template>
  <div v-if="show" class="dialog-overlay" @click="handleCancel">
    <div class="dialog" @click.stop>
      <div class="dialog-header">
        <h3><font-awesome-icon icon="gear" /> Configuration des dossiers</h3>
        <button @click="handleCancel" class="close-btn">✕</button>
      </div>

      <div class="dialog-body">
        <div class="form-group">
          <label for="global-path">Dossier des skills globaux</label>
          <input
            id="global-path"
            v-model="config.global_skills_path"
            type="text"
            class="form-input"
            placeholder="~/.claude/skills"
          />
          <span class="help-text">Emplacement des skills globaux (partagés entre tous les projets)</span>
        </div>

        <div class="form-group">
          <label for="base-path">Dossier de base pour les projets</label>
          <input
            id="base-path"
            v-model="config.projects_base_path"
            type="text"
            class="form-input"
            placeholder="~/Development"
          />
          <span class="help-text">Dossier principal où scanner les projets avec des skills</span>
        </div>

        <div class="form-group">
          <label>Dossiers de projets supplémentaires</label>
          <div class="custom-paths-list">
            <div
              v-for="(_, index) in config.custom_project_paths"
              :key="index"
              class="custom-path-item"
            >
              <input
                v-model="config.custom_project_paths[index]"
                type="text"
                class="form-input"
                placeholder="~/Projects/mon-projet"
              />
              <button
                @click="removePath(index)"
                class="btn-remove"
                title="Supprimer"
              >
                <font-awesome-icon icon="trash" />
              </button>
            </div>
            <button @click="addPath" class="btn-add">
              <font-awesome-icon icon="plus" /> Ajouter un dossier
            </button>
          </div>
          <span class="help-text">Dossiers supplémentaires contenant des projets avec skills</span>
        </div>

        <div class="info-box">
          <strong><font-awesome-icon icon="lightbulb" /> Astuce :</strong> Les changements seront sauvegardés dans<br/>
          <code>{{ configPath }}</code>
        </div>
      </div>

      <div class="dialog-footer">
        <button @click="handleCancel" class="btn btn-secondary">
          Annuler
        </button>
        <button @click="handleSave" class="btn btn-primary">
          <font-awesome-icon icon="floppy-disk" /> Sauvegarder
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface Config {
  global_skills_path: string
  projects_base_path: string
  custom_project_paths: string[]
}

const props = defineProps<{
  show: boolean
  initialConfig: Config
  configPath: string
}>()

const emit = defineEmits<{
  save: [config: Config]
  cancel: []
}>()

const config = ref<Config>({
  global_skills_path: '',
  projects_base_path: '',
  custom_project_paths: []
})

// Réinitialiser le formulaire quand le dialogue s'ouvre
watch(() => props.show, (show) => {
  if (show) {
    config.value = JSON.parse(JSON.stringify(props.initialConfig))
  }
})

function addPath() {
  config.value.custom_project_paths.push('')
}

function removePath(index: number) {
  config.value.custom_project_paths.splice(index, 1)
}

function handleSave() {
  // Filtrer les chemins vides
  config.value.custom_project_paths = config.value.custom_project_paths.filter(p => p.trim() !== '')
  emit('save', config.value)
}

function handleCancel() {
  emit('cancel')
}
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid #e0e0e0;
}

.dialog-header h3 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: #333;
}

.close-btn {
  padding: 0.5rem;
  background: transparent;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  color: #666;
  line-height: 1;
}

.close-btn:hover {
  color: #333;
}

.dialog-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 600;
  color: #333;
  font-size: 0.875rem;
}

.form-input {
  width: 100%;
  padding: 0.625rem 0.75rem;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 0.875rem;
  font-family: monospace;
}

.form-input:focus {
  outline: none;
  border-color: #007bff;
}

.help-text {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.75rem;
  color: #666;
  font-style: italic;
}

.custom-paths-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.custom-path-item {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.btn-remove {
  padding: 0.625rem;
  background: transparent;
  border: 1px solid #dc3545;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.2s;
  flex-shrink: 0;
}

.btn-remove:hover {
  background: #fff5f5;
}

.btn-add {
  padding: 0.625rem 1rem;
  background: #f0f0f0;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  transition: all 0.2s;
  margin-top: 0.5rem;
}

.btn-add:hover {
  background: #e0e0e0;
  border-color: #007bff;
}

.info-box {
  margin-top: 1.5rem;
  padding: 1rem;
  background: #f8f9fa;
  border-left: 4px solid #007bff;
  border-radius: 4px;
  font-size: 0.875rem;
  color: #666;
}

.info-box code {
  display: block;
  margin-top: 0.5rem;
  padding: 0.25rem 0.5rem;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  font-size: 0.75rem;
  word-break: break-all;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1.5rem;
  border-top: 1px solid #e0e0e0;
}

.btn {
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 4px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background: #e0e0e0;
  color: #333;
}

.btn-secondary:hover {
  background: #d0d0d0;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-primary:hover {
  background: #0056b3;
}
</style>
