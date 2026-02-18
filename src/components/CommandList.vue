<template>
  <div class="command-list">
    <div class="header">
      <h2 class="title">Commandes</h2>
      <button @click="handleReload" :disabled="store.loading" class="reload-btn">
        <font-awesome-icon icon="rotate-right" :spin="store.loading" />
      </button>
    </div>

    <div class="search-bar">
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Rechercher une commande..."
        class="search-input"
      />
    </div>

    <div v-if="store.loading" class="loading">
      Chargement...
    </div>

    <div v-else-if="store.error" class="error">
      <font-awesome-icon icon="triangle-exclamation" /> {{ store.error }}
    </div>

    <div v-else-if="filteredCommands.length === 0" class="empty">
      <font-awesome-icon icon="inbox" /> Aucune commande trouvée
    </div>

    <div v-else class="items-container">
      <SkillCard
        v-for="cmd in filteredCommands"
        :key="cmd.id"
        :skill="cmd"
        :is-selected="store.currentCommand?.id === cmd.id"
        @click="store.selectCommand(cmd.id)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useSkillsStore } from '../stores/skillsStore'
import SkillCard from './SkillCard.vue'

const store = useSkillsStore()
const searchQuery = ref('')

const filteredCommands = computed(() => {
  if (!searchQuery.value) return store.commands
  const query = searchQuery.value.toLowerCase()
  return store.commands.filter(c =>
    c.name.toLowerCase().includes(query) ||
    c.description.toLowerCase().includes(query)
  )
})

function handleReload() {
  store.loadSkills()
}
</script>

<style scoped>
.command-list {
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

.reload-btn {
  padding: 0.5rem 0.75rem;
  background: white;
  color: #666;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.2s;
}

.reload-btn:hover:not(:disabled) {
  background: #f5f5f5;
  border-color: #007bff;
  color: #007bff;
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

.items-container {
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
