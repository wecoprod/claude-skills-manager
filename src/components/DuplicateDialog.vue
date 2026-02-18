<template>
  <div v-if="show" class="dialog-overlay" @click="handleCancel">
    <div class="dialog" @click.stop>
      <div class="dialog-header">
        <h3>📋 Dupliquer le skill</h3>
        <button @click="handleCancel" class="close-btn">✕</button>
      </div>

      <div class="dialog-body">
        <div class="form-group">
          <label for="skill-name">Nouveau nom</label>
          <input
            id="skill-name"
            v-model="newName"
            type="text"
            class="form-input"
            placeholder="Entrez le nouveau nom"
            @keyup.enter="handleConfirm"
          />
        </div>

        <div class="form-group">
          <label>Destination</label>
          <div class="radio-group">
            <label class="radio-label">
              <input
                v-model="destinationType"
                type="radio"
                value="global"
                name="destination"
              />
              <span>🌐 Global (~/.claude/skills/)</span>
            </label>
            <label class="radio-label">
              <input
                v-model="destinationType"
                type="radio"
                value="project"
                name="destination"
              />
              <span>📁 Projet</span>
            </label>
          </div>
        </div>

        <div v-if="destinationType === 'project'" class="form-group">
          <label for="project-select">Sélectionner le projet</label>
          <select
            id="project-select"
            v-model="selectedProject"
            class="form-select"
          >
            <option value="">-- Choisir un projet --</option>
            <option
              v-for="project in projects"
              :key="project.path"
              :value="project.path"
            >
              {{ project.name }}
            </option>
          </select>
        </div>

        <div v-if="previewPath" class="preview-path">
          <strong>Chemin :</strong> {{ previewPath }}
        </div>
      </div>

      <div class="dialog-footer">
        <button @click="handleCancel" class="btn btn-secondary">
          Annuler
        </button>
        <button
          @click="handleConfirm"
          class="btn btn-primary"
          :disabled="!canConfirm"
        >
          Dupliquer
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { Project } from '../types/skill'

const props = defineProps<{
  show: boolean
  skillName: string
  projects: Project[]
}>()

const emit = defineEmits<{
  confirm: [data: {
    newName: string
    destinationType: 'global' | 'project'
    destinationProject?: string
  }]
  cancel: []
}>()

const newName = ref('')
const destinationType = ref<'global' | 'project'>('global')
const selectedProject = ref('')

// Auto-générer le nom suggéré quand le dialogue s'ouvre
watch(() => props.show, (show) => {
  if (show) {
    newName.value = `${props.skillName}-copy`
    destinationType.value = 'global'
    selectedProject.value = ''
  }
})

const canConfirm = computed(() => {
  if (!newName.value.trim()) return false
  if (destinationType.value === 'project' && !selectedProject.value) return false
  return true
})

const previewPath = computed(() => {
  if (!newName.value.trim()) return ''

  if (destinationType.value === 'global') {
    return `~/.claude/skills/${newName.value}.md`
  } else if (selectedProject.value) {
    const project = props.projects.find(p => p.path === selectedProject.value)
    return `${project?.name}/.claude/skills/${newName.value}.md`
  }
  return ''
})

function handleConfirm() {
  if (!canConfirm.value) return

  emit('confirm', {
    newName: newName.value.trim(),
    destinationType: destinationType.value,
    destinationProject: destinationType.value === 'project' ? selectedProject.value : undefined
  })
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
  min-width: 500px;
  max-width: 600px;
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
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #333;
  font-size: 0.875rem;
}

.form-input,
.form-select {
  width: 100%;
  padding: 0.625rem 0.75rem;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 0.875rem;
  transition: border-color 0.2s;
}

.form-input:focus,
.form-select:focus {
  outline: none;
  border-color: #007bff;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 4px;
  transition: background 0.2s;
}

.radio-label:hover {
  background: #f5f5f5;
}

.radio-label input[type="radio"] {
  cursor: pointer;
}

.preview-path {
  margin-top: 1rem;
  padding: 0.75rem;
  background: #f5f5f5;
  border-radius: 4px;
  font-size: 0.875rem;
  color: #666;
  font-family: monospace;
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

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  background: #e0e0e0;
  color: #333;
}

.btn-secondary:hover:not(:disabled) {
  background: #d0d0d0;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
}
</style>
