<template>
  <div :class="['skill-card', { selected: isSelected }]">
    <div class="skill-header">
      <div class="skill-name-row">
        <h4 class="skill-name">{{ skill.name }}</h4>
        <span v-if="analysisStatus" :class="['analysis-dot', analysisStatus]" :title="analysisStatus"></span>
      </div>
      <div class="skill-badges">
        <span v-if="skill.source === 'global'" class="skill-badge global">
          <font-awesome-icon icon="globe" /> Global
        </span>
        <span v-else class="skill-badge project">
          <font-awesome-icon icon="folder" /> {{ skill.project_name || 'Projet' }}
        </span>
        <span
          v-for="project in skill.linked_projects"
          :key="project"
          class="skill-badge linked"
        >
          <font-awesome-icon icon="link" /> {{ project }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Skill } from '../types/skill'
import { useSkillsStore } from '../stores/skillsStore'

const props = defineProps<{
  skill: Skill
  isSelected: boolean
}>()

const store = useSkillsStore()
const analysisStatus = computed(() => store.analyses[props.skill.id]?.status ?? null)
</script>

<style scoped>
.skill-card {
  padding: 1rem;
  background: white;
  border: 2px solid transparent;
  border-radius: 8px;
  margin-bottom: 0.75rem;
  cursor: pointer;
  transition: all 0.2s;
}

.skill-card:hover {
  border-color: #007bff;
  box-shadow: 0 2px 8px rgba(0, 123, 255, 0.1);
}

.skill-card.selected {
  border-color: #007bff;
  background: #f0f8ff;
}

.skill-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  gap: 0.5rem;
}

.skill-name-row {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  flex: 1;
  min-width: 0;
}

.skill-name {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: #333;
  min-width: 0;
}

.analysis-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.analysis-dot.valid { background: #10b981; }
.analysis-dot.warning { background: #f59e0b; }
.analysis-dot.error { background: #ef4444; }

.skill-badges {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  align-items: start;
}

.skill-badge {
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  white-space: nowrap;
  font-weight: 500;
}

.skill-badge.global {
  background: #e3f2fd;
  color: #1976d2;
}

.skill-badge.project {
  background: #f3e5f5;
  color: #7b1fa2;
}

.skill-badge.linked {
  background: #fff3e0;
  color: #e65100;
}

.skill-description {
  margin: 0 0 0.5rem 0;
  font-size: 0.875rem;
  color: #666;
  line-height: 1.4;
}

.skill-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.75rem;
  color: #999;
}

.skill-path {
  font-family: monospace;
}
</style>
