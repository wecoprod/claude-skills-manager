// Représentation à plat d'un hook pour l'UI
export interface HookItem {
  id: string
  event: string
  matcher: string
  command: string
  source_path: string
  source_label: string
}

// Source retournée par le backend
export interface HooksSource {
  path: string
  label: string
  hooks: RawHooks
}

// Format interne Claude settings.json
export interface HookEntry {
  matcher: string
  hooks: Array<{ type: string; command: string }>
}

export type RawHooks = Record<string, HookEntry[]>

// Source de serveurs MCP
export interface McpSource {
  path: string
  label: string
}

// Serveur MCP
export interface McpServer {
  name: string
  command: string
  args: string[]
  env: Record<string, string>
  type?: string
  url?: string
  source_path: string
  source_label: string
}

export const HOOK_EVENTS = [
  'PreToolUse',
  'PostToolUse',
  'PreWrite',
  'PostWrite',
  'Stop',
  'SubagentStop',
  'PreCompact',
]
