<template>
  <nav class="app-nav">
    <button
      v-for="section in sections"
      :key="section.id"
      :class="['nav-item', { active: modelValue === section.id }]"
      :title="section.label"
      @click="$emit('update:modelValue', section.id)"
    >
      <font-awesome-icon :icon="section.icon" />
      <span class="nav-label">{{ section.label }}</span>
    </button>
  </nav>
</template>

<script setup lang="ts">
const sections = [
  { id: 'skills', icon: 'book', label: 'Skills' },
  { id: 'commands', icon: 'terminal', label: 'Cmds' },
  { id: 'hooks', icon: 'bolt', label: 'Hooks' },
  { id: 'mcp', icon: 'plug', label: 'MCP' },
  { id: 'plugins', icon: 'puzzle-piece', label: 'Plugins' },
]

defineProps<{ modelValue: string }>()
defineEmits<{ 'update:modelValue': [value: string] }>()
</script>

<style scoped>
.app-nav {
  width: 64px;
  background: #1a1a2e;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 0.75rem 0;
  gap: 0.5rem;
  flex-shrink: 0;
}

.nav-item {
  width: 48px;
  height: 52px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.3rem;
  background: transparent;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  color: #888;
  font-size: 1.1rem;
  transition: all 0.2s;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #ccc;
}

.nav-item.active {
  background: #007bff;
  color: #fff;
}

.nav-label {
  font-size: 0.6rem;
  font-weight: 500;
  letter-spacing: 0.02em;
}
</style>
