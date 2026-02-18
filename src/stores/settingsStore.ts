import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { HookItem, HooksSource, McpServer, McpSource, RawHooks } from '../types/settings'

function flattenHooks(sources: HooksSource[]): HookItem[] {
  const items: HookItem[] = []
  let id = 0
  for (const source of sources) {
    for (const [event, matchers] of Object.entries(source.hooks)) {
      for (const matcherGroup of matchers) {
        for (const hook of matcherGroup.hooks) {
          items.push({
            id: String(id++),
            event,
            matcher: matcherGroup.matcher,
            command: hook.command,
            source_path: source.path,
            source_label: source.label,
          })
        }
      }
    }
  }
  return items
}

function groupBySource(items: HookItem[]): Map<string, RawHooks> {
  const groups = new Map<string, RawHooks>()
  for (const item of items) {
    if (!item.command.trim()) continue
    if (!groups.has(item.source_path)) groups.set(item.source_path, {})
    const raw = groups.get(item.source_path)!
    if (!raw[item.event]) raw[item.event] = []
    const existing = raw[item.event].find(m => m.matcher === item.matcher)
    if (existing) {
      existing.hooks.push({ type: 'command', command: item.command })
    } else {
      raw[item.event].push({
        matcher: item.matcher,
        hooks: [{ type: 'command', command: item.command }],
      })
    }
  }
  return groups
}

export const useSettingsStore = defineStore('settings', () => {
  const hooks = ref<HookItem[]>([])
  const hooksSources = ref<HooksSource[]>([])
  const mcpServers = ref<McpServer[]>([])
  const mcpSources = ref<McpSource[]>([])
  const plugins = ref<Record<string, boolean>>({})
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadHooks(projectPaths: string[]) {
    loading.value = true
    error.value = null
    try {
      const sources = await invoke<HooksSource[]>('load_hooks', { projectPaths })
      hooksSources.value = sources
      hooks.value = flattenHooks(sources)
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
    } finally {
      loading.value = false
    }
  }

  async function saveHooks() {
    try {
      const groups = groupBySource(hooks.value)
      for (const [sourcePath, rawHooks] of groups) {
        await invoke('save_hooks', { sourcePath, hooks: rawHooks })
      }
      // Sauvegarder aussi les sources qui ont été vidées de leurs hooks
      for (const source of hooksSources.value) {
        if (!groups.has(source.path)) {
          await invoke('save_hooks', { sourcePath: source.path, hooks: {} })
        }
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    }
  }

  async function loadMcp(projectPaths: string[] = []) {
    loading.value = true
    error.value = null
    try {
      const sources = await invoke<{ path: string; label: string; servers: Record<string, Record<string, unknown>> }[]>('load_mcp', { projectPaths })
      mcpSources.value = sources.map(s => ({ path: s.path, label: s.label }))
      mcpServers.value = []
      for (const source of sources) {
        for (const [name, cfg] of Object.entries(source.servers)) {
          mcpServers.value.push({
            name,
            command: (cfg.command as string) || '',
            args: (cfg.args as string[]) || [],
            env: (cfg.env as Record<string, string>) || {},
            type: cfg.type as string | undefined,
            url: cfg.url as string | undefined,
            source_path: source.path,
            source_label: source.label,
          })
        }
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
    } finally {
      loading.value = false
    }
  }

  async function loadPlugins() {
    try {
      plugins.value = await invoke<Record<string, boolean>>('load_plugins')
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
    }
  }

  async function savePlugins() {
    try {
      await invoke('save_plugins', { enabledPlugins: plugins.value })
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    }
  }

  async function saveMcp() {
    try {
      const groups = new Map<string, Record<string, unknown>>()
      for (const server of mcpServers.value) {
        if (!groups.has(server.source_path)) groups.set(server.source_path, {})
        const { name, source_path, source_label, ...rest } = server
        groups.get(server.source_path)![name] = rest
      }
      // Sauvegarder aussi les sources qui ont été vidées
      for (const source of mcpSources.value) {
        if (!groups.has(source.path)) {
          await invoke('save_mcp', { sourcePath: source.path, mcpServers: {} })
        }
      }
      for (const [sourcePath, servers] of groups) {
        await invoke('save_mcp', { sourcePath, mcpServers: servers })
      }
    } catch (e) {
      error.value = e instanceof Error ? e.message : String(e)
      throw e
    }
  }

  return {
    hooks,
    hooksSources,
    mcpServers,
    mcpSources,
    plugins,
    loading,
    error,
    loadHooks,
    saveHooks,
    loadMcp,
    saveMcp,
    loadPlugins,
    savePlugins,
  }
})
