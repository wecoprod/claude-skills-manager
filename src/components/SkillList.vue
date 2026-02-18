<template>
  <div class="skill-list">
    <div class="header">
      <h2 class="title">Claude Skills Manager</h2>
      <div class="header-actions">
        <button @click="showSettings = true" class="settings-btn" title="Paramètres">
          <font-awesome-icon icon="gear" />
        </button>
        <button @click="handleReload" :disabled="store.loading" class="reload-btn">
          <font-awesome-icon icon="rotate-right" :spin="store.loading" />
          {{ store.loading ? 'Chargement...' : 'Recharger' }}
        </button>
      </div>
    </div>

    <div class="search-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Rechercher un skill..."
        class="search-input"
      />
    </div>

    <div v-if="store.loading" class="loading">
      Chargement des skills...
    </div>

    <div v-else-if="store.error" class="error">
      <font-awesome-icon icon="triangle-exclamation" /> {{ store.error }}
    </div>

    <div v-else-if="filteredSkills.length === 0" class="empty">
      <font-awesome-icon icon="inbox" /> Aucun skill trouvé
    </div>

    <div v-else class="skills-container">
      <SkillCard
        v-for="skill in filteredSkills"
        :key="skill.id"
        :skill="skill"
        :is-selected="store.currentSkill?.id === skill.id"
        @click="handleSelectSkill(skill.id)"
      />
    </div>

    <!-- Dialogue de settings -->
    <SettingsDialog
      :show="showSettings"
      :initial-config="store.config || defaultConfig"
      :config-path="store.configPath"
      @save="handleSaveSettings"
      @cancel="showSettings = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useSkillsStore } from '../stores/skillsStore'
import SkillCard from './SkillCard.vue'
import SettingsDialog from './SettingsDialog.vue'
import type { Config } from '../types/skill'

const store = useSkillsStore()
const searchQuery = ref('')
const showSettings = ref(false)

const defaultConfig: Config = {
  global_skills_path: '~/.claude/skills',
  projects_base_path: '~/Development',
  custom_project_paths: []
}

const filteredSkills = computed(() => {
  if (!searchQuery.value) return store.skills
  const query = searchQuery.value.toLowerCase()
  return store.skills.filter(s =>
    s.name.toLowerCase().includes(query) ||
    s.description.toLowerCase().includes(query)
  )
})

function handleSelectSkill(skillId: string) {
  store.selectSkill(skillId)
}

function handleReload() {
  store.loadSkills()
}

async function handleSaveSettings(config: Config) {
  try {
    await store.saveConfig(config)
    showSettings.value = false
  } catch (e) {
    alert(`Erreur lors de la sauvegarde de la configuration: ${e}`)
  }
}
</script>

<style scoped>
.skill-list {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #f5f5f5;
}

.header {
  padding: 1rem;
  background: white;
  border-bottom: 1px solid #e0e0e0;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.title {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
}

.header-actions {
  display: flex;
  gap: 0.5rem;
}

.settings-btn {
  padding: 0.5rem 0.75rem;
  background: white;
  color: #666;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.2s;
}

.settings-btn:hover {
  background: #f5f5f5;
  border-color: #007bff;
  color: #007bff;
}

.reload-btn {
  padding: 0.5rem 1rem;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.875rem;
}

.reload-btn:hover:not(:disabled) {
  background: #0056b3;
}

.reload-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.search-bar {
  padding: 1rem;
  background: white;
  border-bottom: 1px solid #e0e0e0;
}

.search-input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 0.875rem;
}

.skills-container {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

.loading, .error, .empty {
  padding: 2rem;
  text-align: center;
  color: #666;
}

.error {
  color: #dc3545;
}
</style>
