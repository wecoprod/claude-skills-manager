import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Skill, Project, Config, SkillAnalysis } from '../types/skill'
import { invoke } from '@tauri-apps/api/core'

export const useSkillsStore = defineStore('skills', () => {
  const skills = ref<Skill[]>([])
  const commands = ref<Skill[]>([])
  const projects = ref<Project[]>([])
  const currentSkill = ref<Skill | null>(null)
  const currentCommand = ref<Skill | null>(null)
  const analyses = ref<Record<string, SkillAnalysis>>({})
  const analyzing = ref(false)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const config = ref<Config | null>(null)
  const configPath = ref<string>('')

  const globalSkills = computed(() =>
    skills.value.filter(s => s.source === 'global')
  )

  const projectSkills = computed(() =>
    skills.value.filter(s => s.source === 'project')
  )

  const projectGroups = computed(() => {
    return projectSkills.value.reduce((groups, skill) => {
      const name = skill.project_name || 'Unknown'
      if (!groups.has(name)) {
        groups.set(name, [])
      }
      const group = groups.get(name)
      if (group) {
        group.push(skill)
      }
      return groups
    }, new Map<string, Skill[]>())
  })

  async function loadConfig() {
    try {
      config.value = await invoke<Config>('load_config')
      configPath.value = await invoke<string>('get_config_path')
      if (import.meta.env.DEV) {
        console.log('⚙️ Config loaded:', config.value)
      }
    } catch (e) {
      if (import.meta.env.DEV) {
        console.error('❌ Failed to load config:', e)
      }
      // Utiliser une config par défaut si échec
      const home = '~'
      config.value = {
        global_skills_path: `${home}/.claude/skills`,
        projects_base_path: `${home}/Development`,
        custom_project_paths: []
      }
    }
  }

  async function saveConfig(newConfig: Config) {
    try {
      await invoke('save_config', { config: newConfig })
      config.value = newConfig
      if (import.meta.env.DEV) {
        console.log('⚙️ Config saved:', newConfig)
      }
      // Recharger les skills après modification de la config
      await loadSkills()
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Une erreur inconnue est survenue'
      error.value = errorMessage
      if (import.meta.env.DEV) {
        console.error('Failed to save config:', e)
      }
      throw e
    }
  }

  async function loadSkills() {
    // Prevent concurrent loads (race condition protection)
    if (loading.value) return

    loading.value = true
    error.value = null

    try {
      // S'assurer que la config est chargée
      if (!config.value) {
        await loadConfig()
      }

      // D'abord, scanner les projets
      await scanProjects()

      // Ensuite, charger tous les skills (global + projets)
      const projectPaths = projects.value.map(p => p.path)
      if (import.meta.env.DEV) {
        console.log('📂 Project paths for skills:', projectPaths)
      }
      skills.value = await invoke<Skill[]>('list_skills', { projectPaths })
      commands.value = await invoke<Skill[]>('list_commands', { projectPaths })
      if (import.meta.env.DEV) {
        console.log('📚 Skills loaded:', skills.value.length, 'skills,', commands.value.length, 'commands')
      }
      // Analyser tous les skills en arrière-plan (sans bloquer le chargement)
      analyzeAllSkills()
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Une erreur inconnue est survenue'
      error.value = errorMessage
      if (import.meta.env.DEV) {
        console.error('Failed to load skills:', e)
      }
    } finally {
      loading.value = false
    }
  }

  async function scanProjects() {
    try {
      if (!config.value) {
        await loadConfig()
      }

      // Construire la liste des répertoires à scanner
      const baseDirs = [
        config.value!.projects_base_path,
        ...config.value!.custom_project_paths
      ]

      projects.value = await invoke<Project[]>('scan_projects', { baseDirs })
      if (import.meta.env.DEV) {
        console.log('📁 Projects scanned:', projects.value.length, projects.value)
      }
    } catch (e) {
      if (import.meta.env.DEV) {
        console.error('❌ Failed to scan projects:', e)
      }
      projects.value = [] // Reset on error
      throw e // Re-throw to let loadSkills handle it
    }
  }

  async function selectSkill(skillId: string) {
    const skill = skills.value.find(s => s.id === skillId)
    currentSkill.value = skill || null
  }

  function clearSelection() {
    currentSkill.value = null
  }

  async function selectCommand(commandId: string) {
    const command = commands.value.find(c => c.id === commandId)
    currentCommand.value = command || null
  }

  function clearCurrentCommand() {
    currentCommand.value = null
  }

  async function createSymlink(skillId: string, targetProjectPath: string) {
    try {
      const skill = skills.value.find(s => s.id === skillId)
      if (!skill) {
        throw new Error('Skill introuvable')
      }

      await invoke('create_skill_symlink', {
        sourcePath: skill.path,
        targetProjectPath
      })

      // Recharger les skills pour voir le nouveau symlink
      await loadSkills()

      if (import.meta.env.DEV) {
        console.log('🔗 Symlink créé avec succès')
      }
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Erreur lors de la création du symlink'
      error.value = errorMessage
      if (import.meta.env.DEV) {
        console.error('Failed to create symlink:', e)
      }
      throw e
    }
  }

  async function duplicateSkill(
    skillId: string,
    destinationType: 'global' | 'project',
    destinationProject: string | undefined,
    newName: string
  ) {
    try {
      const skill = skills.value.find(s => s.id === skillId)
      if (!skill) {
        throw new Error('Skill introuvable')
      }

      const newSkill = await invoke<Skill>('duplicate_skill', {
        sourcePath: skill.path,
        destinationType,
        destinationProject,
        newName
      })

      // Recharger les skills
      await loadSkills()

      // Sélectionner le nouveau skill
      currentSkill.value = newSkill

      if (import.meta.env.DEV) {
        console.log('📋 Skill dupliqué avec succès:', newSkill.name)
      }

      return newSkill
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Erreur lors de la duplication'
      error.value = errorMessage
      if (import.meta.env.DEV) {
        console.error('Failed to duplicate skill:', e)
      }
      throw e
    }
  }

  async function deleteSkill(skillId: string) {
    try {
      const skill = skills.value.find(s => s.id === skillId)
      if (!skill) {
        throw new Error('Skill introuvable')
      }

      await invoke('delete_skill', { path: skill.path })

      // Recharger les skills
      await loadSkills()

      // Désélectionner si c'était le skill actuel
      if (currentSkill.value?.id === skillId) {
        currentSkill.value = null
      }

      if (import.meta.env.DEV) {
        console.log('🗑️ Skill supprimé avec succès')
      }
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Erreur lors de la suppression'
      error.value = errorMessage
      if (import.meta.env.DEV) {
        console.error('Failed to delete skill:', e)
      }
      throw e
    }
  }

  async function analyzeSkill(skillId: string) {
    const skill = skills.value.find(s => s.id === skillId)
    if (!skill) throw new Error('Skill introuvable')

    analyzing.value = true
    try {
      const result = await invoke<SkillAnalysis>('analyze_skill', { path: skill.path })
      analyses.value = { ...analyses.value, [skillId]: result }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    } finally {
      analyzing.value = false
    }
  }

  async function analyzeAllSkills() {
    for (const skill of skills.value) {
      try {
        const result = await invoke<SkillAnalysis>('analyze_skill', { path: skill.path })
        analyses.value = { ...analyses.value, [skill.id]: result }
      } catch {
        // Ignorer les erreurs individuelles silencieusement
      }
    }
  }

  async function updateSkill(
    skillId: string,
    name: string,
    description: string | undefined,
    content: string
  ) {
    try {
      const skill = skills.value.find(s => s.id === skillId)
      if (!skill) {
        throw new Error('Skill introuvable')
      }

      const updatedSkill = await invoke<Skill>('update_skill', {
        path: skill.path,
        name,
        description: description || undefined,
        content
      })

      // Recharger les skills
      await loadSkills()

      // Mettre à jour le skill actuel
      currentSkill.value = updatedSkill

      if (import.meta.env.DEV) {
        console.log('✏️ Skill mis à jour avec succès')
      }

      return updatedSkill
    } catch (e) {
      const errorMessage = e instanceof Error ? e.message : 'Erreur lors de la mise à jour'
      error.value = errorMessage
      if (import.meta.env.DEV) {
        console.error('Failed to update skill:', e)
      }
      throw e
    }
  }

  return {
    skills,
    commands,
    projects,
    currentSkill,
    currentCommand,
    analyses,
    analyzing,
    loading,
    error,
    config,
    configPath,
    globalSkills,
    projectSkills,
    projectGroups,
    loadConfig,
    saveConfig,
    loadSkills,
    scanProjects,
    selectSkill,
    clearSelection,
    selectCommand,
    clearCurrentCommand,
    createSymlink,
    duplicateSkill,
    deleteSkill,
    updateSkill,
    analyzeSkill,
    analyzeAllSkills,
  }
})
