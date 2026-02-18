export interface SkillFrontmatter {
  name: string
  description?: string
}

export interface Skill {
  id: string
  name: string
  description: string
  path: string
  source: 'global' | 'project'
  project_name?: string
  content: string
  last_modified: number
  linked_projects: string[]
}

export interface Project {
  name: string
  path: string
  skills_path: string
  skill_count: number
}

export interface Config {
  global_skills_path: string
  projects_base_path: string
  custom_project_paths: string[]
}

export interface SkillIssue {
  severity: 'error' | 'warning' | 'info'
  category: 'description' | 'naming' | 'structure' | 'instructions' | 'frontmatter' | 'optimization'
  code: string
  message: string
  current_value?: string
  location?: string
}

export interface SkillSuggestion {
  id: string
  priority: 'high' | 'medium' | 'low'
  category: string
  title: string
  description: string
  current: string
  suggested: string
  impact: string
  effort: string
  checklist: string[]
}

export interface SkillStructure {
  has_skill_md: boolean
  has_readme: boolean
  folders: string[]
  total_size_kb: number
  instruction_word_count: number
}

export interface SkillAnalysis {
  path: string
  name: string
  status: 'valid' | 'warning' | 'error'
  score: number
  category: string
  frontmatter: Record<string, unknown>
  structure: SkillStructure
  issues: SkillIssue[]
  suggestions: SkillSuggestion[]
}
