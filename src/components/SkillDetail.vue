<template>
  <div v-if="skill" class="skill-detail">
    <div class="detail-header">
      <div class="header-content">
        <div class="title-row">
          <h2 v-if="!isEditing" class="skill-title">{{ skill.name }}</h2>
          <input
            v-else
            v-model="editName"
            type="text"
            class="skill-title-input"
            placeholder="Nom du skill"
          />
          <div v-if="!isEditing" class="title-actions">
            <button @click="handleEditClick" class="action-btn" title="Éditer">
              <font-awesome-icon icon="pen" />
            </button>
            <button @click="handleDuplicateClick" class="action-btn" title="Dupliquer">
              <font-awesome-icon icon="copy" />
            </button>
            <button @click="handleSymlinkClick" class="action-btn" title="Créer symlink">
              <font-awesome-icon icon="link" />
            </button>
            <button @click="handleDeleteClick" class="action-btn danger" title="Supprimer">
              <font-awesome-icon icon="trash" />
            </button>
          </div>
        </div>
        <p v-if="skill.description && !isEditing" class="skill-description">
          {{ skill.description }}
        </p>
        <p class="skill-meta">
          <span class="meta-item">
            <font-awesome-icon :icon="skill.source === 'global' ? 'globe' : 'folder'" />
            {{ skill.source === 'global' ? 'Global' : 'Projet' }}
          </span>
          <span v-if="skill.project_name" class="meta-item">
            {{ skill.project_name }}
          </span>
          <span
            v-for="project in skill.linked_projects"
            :key="project"
            class="meta-item linked"
          >
            <font-awesome-icon icon="link" /> {{ project }}
          </span>
        </p>
      </div>
      <div class="header-actions">
        <button @click="handleClose" class="close-btn">✕</button>
      </div>
    </div>

    <!-- Onglets (masqués en mode édition) -->
    <div v-if="!isEditing" class="detail-tabs">
      <button
        :class="['tab-btn', { active: activeTab === 'content' }]"
        @click="activeTab = 'content'"
      >
        <font-awesome-icon icon="file-lines" /> Contenu
      </button>
      <button
        :class="['tab-btn', { active: activeTab === 'analysis' }]"
        @click="activeTab = 'analysis'"
      >
        <font-awesome-icon icon="chart-bar" /> Analyse
        <span v-if="analysis" :class="['status-dot', analysis.status]"></span>
      </button>
    </div>

    <!-- Mode édition -->
    <div v-if="isEditing" class="edit-mode">
      <div class="edit-header">
        <div class="form-group-inline">
          <label>Description</label>
          <input
            v-model="editDescription"
            type="text"
            class="form-input-inline"
            placeholder="Description du skill (optionnel)"
          />
        </div>
        <div class="edit-actions">
          <button @click="handleSave" class="btn btn-success">
            <font-awesome-icon icon="floppy-disk" /> Sauvegarder
          </button>
          <button @click="handleCancelEdit" class="btn btn-secondary">
            <font-awesome-icon icon="xmark" /> Annuler
          </button>
        </div>
      </div>

      <div class="editor-container">
        <MarkdownEditor v-model="editContent" />
      </div>
    </div>

    <!-- Onglet Contenu -->
    <div v-else-if="activeTab === 'content'" class="detail-body">
      <div class="content-section">
        <div class="markdown-preview" v-html="renderedMarkdown"></div>
      </div>

      <div class="info-section">
        <h3>Informations</h3>
        <dl class="info-list">
          <dt>Dossier :</dt>
          <dd>{{ skill.path }}</dd>
          <dt>Dernière modification :</dt>
          <dd>{{ formatDate(skill.last_modified) }}</dd>
        </dl>
      </div>
    </div>

    <!-- Onglet Analyse -->
    <div v-else-if="activeTab === 'analysis'" class="analysis-tab">
      <div class="analysis-actions">
        <button
          @click="handleAnalyze"
          class="btn btn-secondary btn-sm"
          :disabled="store.analyzing"
        >
          <font-awesome-icon :icon="store.analyzing ? 'spinner' : 'rotate-right'" :spin="store.analyzing" />
          {{ store.analyzing ? 'Analyse en cours...' : 'Re-analyser' }}
        </button>
      </div>

      <div v-if="analysis" class="analysis-content">
        <!-- Score -->
        <div class="score-card">
          <div :class="['score-circle', analysis.status]">
            {{ analysis.score }}/100
          </div>
          <div class="score-info">
            <span :class="['status-badge', analysis.status]">
              {{ analysis.status === 'valid' ? '✅ Valide' : analysis.status === 'warning' ? '⚠️ Avertissements' : '❌ Erreurs critiques' }}
            </span>
            <span class="category-tag">{{ analysis.category }}</span>
          </div>
        </div>

        <!-- Issues -->
        <section v-if="analysis.issues.length" class="issues-section">
          <h3>Problèmes détectés ({{ analysis.issues.length }})</h3>
          <div
            v-for="issue in analysis.issues"
            :key="issue.code"
            :class="['issue-card', issue.severity]"
          >
            <div class="issue-header">
              <span :class="['severity-badge', issue.severity]">
                {{ issue.severity === 'error' ? 'Erreur' : issue.severity === 'warning' ? 'Avertissement' : 'Info' }}
              </span>
              <span class="issue-code">{{ issue.code }}</span>
            </div>
            <p class="issue-message">{{ issue.message }}</p>
            <div v-if="issue.current_value" class="current-value">
              <strong>Valeur actuelle :</strong>
              <code>{{ issue.current_value }}</code>
            </div>
          </div>
        </section>

        <section v-else class="no-issues">
          <font-awesome-icon icon="check" /> Aucun problème détecté
        </section>

        <!-- Suggestions -->
        <section v-if="analysis.suggestions.length" class="suggestions-section">
          <h3>Améliorations recommandées ({{ analysis.suggestions.length }})</h3>

          <div
            v-for="suggestion in analysis.suggestions"
            :key="suggestion.id"
            class="suggestion-card"
          >
            <div class="suggestion-header">
              <h4>{{ suggestion.title }}</h4>
              <div class="suggestion-meta">
                <span :class="['priority-badge', suggestion.priority]">
                  {{ suggestion.priority === 'high' ? 'Haute' : suggestion.priority === 'medium' ? 'Moyenne' : 'Faible' }}
                </span>
                <span class="effort-badge">{{ suggestion.effort }}</span>
              </div>
            </div>

            <p class="suggestion-description">{{ suggestion.description }}</p>
            <p class="impact-text"><strong>Impact :</strong> {{ suggestion.impact }}</p>

            <div class="code-comparison">
              <div class="code-block before">
                <label>Avant</label>
                <pre><code>{{ suggestion.current }}</code></pre>
              </div>
              <div class="code-block after">
                <label>Après</label>
                <pre><code>{{ suggestion.suggested }}</code></pre>
              </div>
            </div>

            <div class="checklist">
              <strong>Étapes :</strong>
              <ul>
                <li v-for="(step, idx) in suggestion.checklist" :key="idx">{{ step }}</li>
              </ul>
            </div>

            <div class="suggestion-actions">
              <button @click="copySuggested(suggestion.suggested)" class="btn btn-secondary btn-sm">
                <font-awesome-icon icon="copy" /> Copier le code suggéré
              </button>
            </div>
          </div>
        </section>
      </div>

      <div v-else-if="store.analyzing" class="analysis-empty">
        <font-awesome-icon icon="spinner" spin />
        <p>Analyse en cours...</p>
      </div>

      <div v-else class="analysis-empty">
        <font-awesome-icon icon="magnifying-glass" />
        <p>L'analyse sera disponible dans quelques instants.</p>
      </div>
    </div>

    <!-- Dialogues -->
    <ConfirmDialog
      :show="showDeleteDialog"
      title="Confirmer la suppression"
      message="Êtes-vous sûr de vouloir supprimer ce skill ? Cette action est irréversible."
      confirm-text="Supprimer"
      cancel-text="Annuler"
      @confirm="handleConfirmDelete"
      @cancel="showDeleteDialog = false"
    />

    <DuplicateDialog
      :show="showDuplicateDialog"
      :skill-name="skill.name"
      :projects="store.projects"
      @confirm="handleConfirmDuplicate"
      @cancel="showDuplicateDialog = false"
    />

    <SymlinkDialog
      :show="showSymlinkDialog"
      :skill-name="skill.name"
      :projects="store.projects"
      @confirm="handleConfirmSymlink"
      @cancel="showSymlinkDialog = false"
    />
  </div>
  <div v-else class="empty-state">
    <p>Sélectionnez un skill dans la liste</p>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { marked } from 'marked'
import DOMPurify from 'dompurify'
import { useSkillsStore } from '../stores/skillsStore'
import type { Skill, SkillSuggestion } from '../types/skill'
import ConfirmDialog from './ConfirmDialog.vue'
import DuplicateDialog from './DuplicateDialog.vue'
import SymlinkDialog from './SymlinkDialog.vue'
import MarkdownEditor from './MarkdownEditor.vue'

const props = defineProps<{
  skill: Skill | null
}>()

const emit = defineEmits<{
  close: []
}>()

const store = useSkillsStore()

const activeTab = ref<'content' | 'analysis'>('content')
const showActionsMenu = ref(false)
const isEditing = ref(false)
const editName = ref('')
const editDescription = ref('')
const editContent = ref('')
const showDeleteDialog = ref(false)
const showDuplicateDialog = ref(false)
const showSymlinkDialog = ref(false)

const analysis = computed(() => {
  if (!props.skill) return null
  return store.analyses[props.skill.id] ?? null
})

watch(() => props.skill, (newSkill) => {
  if (newSkill && isEditing.value) {
    editName.value = newSkill.name
    editDescription.value = newSkill.description || ''
    editContent.value = newSkill.content
  }
  showActionsMenu.value = false
  activeTab.value = 'content'
})

const renderedMarkdown = computed(() => {
  if (!props.skill) return ''
  try {
    const html = marked(props.skill.content) as string
    return DOMPurify.sanitize(html)
  } catch (e) {
    if (import.meta.env.DEV) {
      console.error('Error rendering markdown:', e)
    }
    return '<p>Erreur de rendu du Markdown</p>'
  }
})

function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000)
  return date.toLocaleString('fr-FR')
}

function handleClose() {
  emit('close')
}

function handleEditClick() {
  if (!props.skill) return
  isEditing.value = true
  editName.value = props.skill.name
  editDescription.value = props.skill.description || ''
  editContent.value = props.skill.content
  showActionsMenu.value = false
}

function handleDuplicateClick() {
  showDuplicateDialog.value = true
  showActionsMenu.value = false
}

function handleSymlinkClick() {
  showSymlinkDialog.value = true
  showActionsMenu.value = false
}

function handleDeleteClick() {
  showDeleteDialog.value = true
  showActionsMenu.value = false
}

async function handleAnalyze() {
  if (!props.skill) return
  try {
    await store.analyzeSkill(props.skill.id)
  } catch {
    // error set in store
  }
}

function copySuggested(text: string) {
  navigator.clipboard.writeText(text)
}

async function handleSave() {
  if (!props.skill) return
  try {
    await store.updateSkill(
      props.skill.id,
      editName.value,
      editDescription.value || undefined,
      editContent.value
    )
    isEditing.value = false
  } catch (e) {
    alert(`Erreur lors de la sauvegarde: ${e}`)
  }
}

function handleCancelEdit() {
  isEditing.value = false
}

async function handleConfirmDelete() {
  if (!props.skill) return
  try {
    await store.deleteSkill(props.skill.id)
    showDeleteDialog.value = false
  } catch (e) {
    alert(`Erreur lors de la suppression: ${e}`)
  }
}

async function handleConfirmDuplicate(data: {
  newName: string
  destinationType: 'global' | 'project'
  destinationProject?: string
}) {
  if (!props.skill) return
  try {
    await store.duplicateSkill(
      props.skill.id,
      data.destinationType,
      data.destinationProject,
      data.newName
    )
    showDuplicateDialog.value = false
  } catch (e) {
    alert(`Erreur lors de la duplication: ${e}`)
  }
}

async function handleConfirmSymlink(projectPath: string) {
  if (!props.skill) return
  try {
    await store.createSymlink(props.skill.id, projectPath)
    showSymlinkDialog.value = false
    alert('Symlink créé avec succès !')
  } catch (e) {
    alert(`Erreur lors de la création du symlink: ${e}`)
  }
}
</script>

<style scoped>
.skill-detail {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: white;
}

.detail-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  padding: 1.5rem;
  border-bottom: 1px solid #e0e0e0;
  background: #f8f9fa;
}

.header-content {
  flex: 1;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.title-row {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.5rem;
}

.skill-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
}

.skill-title-input {
  flex: 1;
  padding: 0.5rem;
  font-size: 1.5rem;
  font-weight: 600;
  border: 2px solid #007bff;
  border-radius: 4px;
}

.title-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  padding: 0.5rem 0.75rem;
  background: white;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
}

.action-btn:hover {
  background: #f5f5f5;
  border-color: #007bff;
}

.action-btn.danger:hover {
  background: #fff5f5;
  border-color: #dc3545;
}

.skill-description {
  margin: 0 0 0.5rem 0;
  font-size: 0.875rem;
  color: #666;
  line-height: 1.4;
}

.skill-meta {
  margin: 0;
  display: flex;
  gap: 1rem;
  font-size: 0.875rem;
  color: #666;
}

.meta-item {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  background: #f5f5f5;
  border-radius: 4px;
}

.meta-item.linked {
  background: #fff3e0;
  color: #e65100;
}

.close-btn {
  padding: 0.5rem;
  background: transparent;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #666;
  line-height: 1;
}

.close-btn:hover {
  color: #333;
}

/* Onglets */
.detail-tabs {
  display: flex;
  border-bottom: 1px solid #e0e0e0;
  background: white;
  padding: 0 1.5rem;
}

.tab-btn {
  padding: 0.75rem 1.25rem;
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  cursor: pointer;
  font-size: 0.875rem;
  color: #666;
  display: flex;
  align-items: center;
  gap: 0.4rem;
  transition: all 0.2s;
  margin-bottom: -1px;
}

.tab-btn:hover {
  color: #333;
}

.tab-btn.active {
  color: #007bff;
  border-bottom-color: #007bff;
  font-weight: 500;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  display: inline-block;
}

.status-dot.valid { background: #10b981; }
.status-dot.warning { background: #f59e0b; }
.status-dot.error { background: #ef4444; }

/* Corps du contenu */
.detail-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

/* Mode édition */
.edit-mode {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 120px);
}

.edit-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.5rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e0e0e0;
}

.form-group-inline {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.form-group-inline label {
  font-weight: 600;
  color: #333;
  font-size: 0.875rem;
  white-space: nowrap;
}

.form-input-inline {
  flex: 1;
  padding: 0.5rem 0.75rem;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 0.875rem;
}

.form-input-inline:focus {
  outline: none;
  border-color: #007bff;
}

.editor-container {
  flex: 1;
  overflow: hidden;
  padding: 1.5rem;
}

.edit-actions {
  display: flex;
  gap: 0.75rem;
}

/* Boutons */
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
  gap: 0.4rem;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-sm {
  padding: 0.4rem 0.75rem;
  font-size: 0.8rem;
}

.btn-primary {
  background: #007bff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #0056b3;
}

.btn-success {
  background: #28a745;
  color: white;
}

.btn-success:hover {
  background: #218838;
}

.btn-secondary {
  background: #6c757d;
  color: white;
}

.btn-secondary:hover {
  background: #5a6268;
}

/* Markdown */
.content-section,
.info-section {
  margin-bottom: 2rem;
}

.info-section h3 {
  margin: 0 0 1rem 0;
  font-size: 1.125rem;
  font-weight: 600;
  color: #333;
  border-bottom: 2px solid #007bff;
  padding-bottom: 0.5rem;
}

.markdown-preview {
  line-height: 1.6;
  color: #333;
}

.markdown-preview :deep(h1) { font-size: 1.75rem; margin: 1.5rem 0 1rem; }
.markdown-preview :deep(h2) { font-size: 1.5rem; margin: 1.25rem 0 0.75rem; }
.markdown-preview :deep(h3) { font-size: 1.25rem; margin: 1rem 0 0.5rem; }
.markdown-preview :deep(p) { margin: 0.5rem 0; }
.markdown-preview :deep(code) {
  background: #f5f5f5;
  padding: 0.125rem 0.25rem;
  border-radius: 3px;
  font-family: monospace;
  font-size: 0.875em;
}
.markdown-preview :deep(pre) {
  background: #f5f5f5;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
}
.markdown-preview :deep(pre code) { background: none; padding: 0; }
.markdown-preview :deep(ul),
.markdown-preview :deep(ol) { padding-left: 2rem; margin: 0.5rem 0; }
.markdown-preview :deep(li) { margin: 0.25rem 0; }

.info-list {
  margin: 0;
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 0.5rem 1rem;
}

.info-list dt { font-weight: 600; color: #666; }
.info-list dd {
  margin: 0;
  color: #333;
  font-family: monospace;
  font-size: 0.875rem;
  word-break: break-all;
}

/* Onglet Analyse */
.analysis-tab {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.analysis-actions {
  display: flex;
  justify-content: flex-start;
}

.analysis-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.score-card {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  padding: 1.25rem;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #e0e0e0;
}

.score-circle {
  width: 72px;
  height: 72px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.9rem;
  font-weight: 700;
  color: white;
  flex-shrink: 0;
}

.score-circle.valid { background: #10b981; }
.score-circle.warning { background: #f59e0b; }
.score-circle.error { background: #ef4444; }

.score-info {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.status-badge {
  font-size: 1rem;
  font-weight: 600;
}

.status-badge.valid { color: #10b981; }
.status-badge.warning { color: #f59e0b; }
.status-badge.error { color: #ef4444; }

.category-tag {
  font-size: 0.8rem;
  color: #888;
  font-family: monospace;
}

.issues-section h3,
.suggestions-section h3 {
  margin: 0 0 1rem 0;
  font-size: 1rem;
  font-weight: 600;
  color: #333;
}

.issue-card {
  padding: 0.875rem 1rem;
  border-radius: 6px;
  margin-bottom: 0.625rem;
  border-left: 3px solid;
}

.issue-card.error {
  background: #fff5f5;
  border-color: #ef4444;
}

.issue-card.warning {
  background: #fffbeb;
  border-color: #f59e0b;
}

.issue-card.info {
  background: #eff6ff;
  border-color: #3b82f6;
}

.issue-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.375rem;
}

.severity-badge {
  font-size: 0.7rem;
  font-weight: 700;
  padding: 0.15rem 0.4rem;
  border-radius: 3px;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.severity-badge.error { background: #ef4444; color: white; }
.severity-badge.warning { background: #f59e0b; color: white; }
.severity-badge.info { background: #3b82f6; color: white; }

.issue-code {
  font-size: 0.75rem;
  font-family: monospace;
  color: #888;
}

.issue-message {
  margin: 0;
  font-size: 0.875rem;
  color: #333;
}

.current-value {
  margin-top: 0.375rem;
  font-size: 0.8rem;
  color: #666;
}

.current-value code {
  background: rgba(0,0,0,0.06);
  padding: 0.1rem 0.3rem;
  border-radius: 3px;
  font-family: monospace;
}

.no-issues {
  padding: 1rem;
  color: #10b981;
  font-weight: 500;
  font-size: 0.9rem;
}

.suggestion-card {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 1rem;
  margin-bottom: 1rem;
}

.suggestion-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 0.75rem;
  margin-bottom: 0.625rem;
}

.suggestion-header h4 {
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
  color: #333;
}

.suggestion-meta {
  display: flex;
  gap: 0.5rem;
  flex-shrink: 0;
}

.priority-badge {
  font-size: 0.7rem;
  font-weight: 700;
  padding: 0.15rem 0.4rem;
  border-radius: 3px;
  text-transform: uppercase;
}

.priority-badge.high { background: #fee2e2; color: #dc2626; }
.priority-badge.medium { background: #fef3c7; color: #d97706; }
.priority-badge.low { background: #f0fdf4; color: #16a34a; }

.effort-badge {
  font-size: 0.75rem;
  color: #888;
  padding: 0.15rem 0.4rem;
  background: #f5f5f5;
  border-radius: 3px;
}

.suggestion-description {
  margin: 0 0 0.5rem 0;
  font-size: 0.875rem;
  color: #555;
}

.impact-text {
  margin: 0 0 0.75rem 0;
  font-size: 0.8rem;
  color: #666;
}

.code-comparison {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.75rem;
  margin-bottom: 0.75rem;
}

.code-block label {
  display: block;
  font-size: 0.75rem;
  font-weight: 600;
  margin-bottom: 0.25rem;
}

.code-block.before label { color: #dc2626; }
.code-block.after label { color: #16a34a; }

.code-block pre {
  margin: 0;
  padding: 0.625rem;
  border-radius: 4px;
  overflow-x: auto;
  font-size: 0.75rem;
}

.code-block.before pre { background: #fff5f5; border: 1px solid #fca5a5; }
.code-block.after pre { background: #f0fdf4; border: 1px solid #86efac; }

.code-block code {
  font-family: monospace;
  white-space: pre-wrap;
  word-break: break-word;
}

.checklist {
  font-size: 0.8rem;
  color: #555;
  margin-bottom: 0.75rem;
}

.checklist ul {
  margin: 0.25rem 0 0 0;
  padding-left: 1.5rem;
}

.checklist li {
  margin: 0.2rem 0;
}

.suggestion-actions {
  display: flex;
  gap: 0.5rem;
}

.analysis-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 3rem;
  color: #999;
  font-size: 2rem;
  text-align: center;
}

.analysis-empty p {
  font-size: 0.875rem;
  max-width: 300px;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
  color: #999;
  font-size: 1.125rem;
}
</style>
