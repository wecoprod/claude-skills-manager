<template>
  <div class="app">
    <AppNav v-model="activeSection" />

    <!-- Skills -->
    <template v-if="activeSection === 'skills'">
      <div class="sidebar">
        <SkillList />
      </div>
      <div class="main">
        <SkillDetail
          :skill="skillsStore.currentSkill"
          @close="skillsStore.clearSelection()"
        />
      </div>
    </template>

    <!-- Commandes -->
    <template v-else-if="activeSection === 'commands'">
      <div class="sidebar">
        <CommandList />
      </div>
      <div class="main">
        <SkillDetail
          :skill="skillsStore.currentCommand"
          @close="skillsStore.clearCurrentCommand()"
        />
      </div>
    </template>

    <!-- Hooks -->
    <template v-else-if="activeSection === 'hooks'">
      <div class="full-content">
        <HooksEditor />
      </div>
    </template>

    <!-- MCP -->
    <template v-else-if="activeSection === 'mcp'">
      <div class="full-content">
        <McpEditor />
      </div>
    </template>

    <!-- Plugins -->
    <template v-else-if="activeSection === 'plugins'">
      <div class="full-content">
        <PluginsEditor />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useSkillsStore } from './stores/skillsStore'
import AppNav from './components/AppNav.vue'
import SkillList from './components/SkillList.vue'
import SkillDetail from './components/SkillDetail.vue'
import CommandList from './components/CommandList.vue'
import HooksEditor from './components/HooksEditor.vue'
import McpEditor from './components/McpEditor.vue'
import PluginsEditor from './components/PluginsEditor.vue'

const skillsStore = useSkillsStore()
const activeSection = ref<'skills' | 'commands' | 'hooks' | 'mcp' | 'plugins'>('skills')

onMounted(() => {
  skillsStore.loadSkills()
})
</script>

<style>
* {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
    'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
    sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

#app {
  height: 100vh;
  overflow: hidden;
}
</style>

<style scoped>
.app {
  display: flex;
  height: 100vh;
}

.sidebar {
  width: 400px;
  border-right: 1px solid #e0e0e0;
  overflow: hidden;
  flex-shrink: 0;
}

.main {
  flex: 1;
  overflow: hidden;
}

.full-content {
  flex: 1;
  overflow: hidden;
}
</style>
