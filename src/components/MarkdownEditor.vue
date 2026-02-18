<template>
  <div class="markdown-editor-wrapper">
    <div v-if="editor" class="editor-toolbar">
      <button
        @click="editor.chain().focus().toggleBold().run()"
        :class="{ 'is-active': editor.isActive('bold') }"
        class="toolbar-btn"
        type="button"
      >
        <strong>B</strong>
      </button>
      <button
        @click="editor.chain().focus().toggleItalic().run()"
        :class="{ 'is-active': editor.isActive('italic') }"
        class="toolbar-btn"
        type="button"
      >
        <em>I</em>
      </button>
      <button
        @click="editor.chain().focus().toggleStrike().run()"
        :class="{ 'is-active': editor.isActive('strike') }"
        class="toolbar-btn"
        type="button"
      >
        <s>S</s>
      </button>
      <span class="toolbar-separator"></span>
      <button
        @click="editor.chain().focus().toggleHeading({ level: 1 }).run()"
        :class="{ 'is-active': editor.isActive('heading', { level: 1 }) }"
        class="toolbar-btn"
        type="button"
      >
        H1
      </button>
      <button
        @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
        :class="{ 'is-active': editor.isActive('heading', { level: 2 }) }"
        class="toolbar-btn"
        type="button"
      >
        H2
      </button>
      <button
        @click="editor.chain().focus().toggleHeading({ level: 3 }).run()"
        :class="{ 'is-active': editor.isActive('heading', { level: 3 }) }"
        class="toolbar-btn"
        type="button"
      >
        H3
      </button>
      <span class="toolbar-separator"></span>
      <button
        @click="editor.chain().focus().toggleBulletList().run()"
        :class="{ 'is-active': editor.isActive('bulletList') }"
        class="toolbar-btn"
        type="button"
      >
        • Liste
      </button>
      <button
        @click="editor.chain().focus().toggleOrderedList().run()"
        :class="{ 'is-active': editor.isActive('orderedList') }"
        class="toolbar-btn"
        type="button"
      >
        1. Liste
      </button>
      <button
        @click="editor.chain().focus().toggleCodeBlock().run()"
        :class="{ 'is-active': editor.isActive('codeBlock') }"
        class="toolbar-btn"
        type="button"
      >
        &lt;/&gt; Code
      </button>
      <button
        @click="editor.chain().focus().toggleBlockquote().run()"
        :class="{ 'is-active': editor.isActive('blockquote') }"
        class="toolbar-btn"
        type="button"
      >
        " Quote
      </button>
      <span class="toolbar-separator"></span>
      <button
        @click="editor.chain().focus().setHorizontalRule().run()"
        class="toolbar-btn"
        type="button"
      >
        — Ligne
      </button>
    </div>
    <editor-content :editor="editor" class="editor-content" />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onBeforeUnmount } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Placeholder from '@tiptap/extension-placeholder'
import { marked } from 'marked'
import TurndownService from 'turndown'

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const turndownService = new TurndownService({
  headingStyle: 'atx',
  codeBlockStyle: 'fenced',
})

// Flag pour éviter la boucle de mise à jour
const isUpdatingFromEditor = ref(false)

const editor = useEditor({
  extensions: [
    StarterKit,
    Placeholder.configure({
      placeholder: 'Commencez à écrire votre contenu...',
    }),
  ],
  content: marked(props.modelValue) as string,
  onUpdate: ({ editor }) => {
    isUpdatingFromEditor.value = true
    const html = editor.getHTML()
    const markdown = turndownService.turndown(html)
    emit('update:modelValue', markdown)
    // Réinitialiser le flag après un court délai
    setTimeout(() => {
      isUpdatingFromEditor.value = false
    }, 10)
  },
  editorProps: {
    attributes: {
      class: 'prose prose-sm max-w-none focus:outline-none',
    },
  },
})

// Mettre à jour le contenu seulement si le changement vient de l'extérieur
watch(() => props.modelValue, (newValue) => {
  if (!isUpdatingFromEditor.value && editor.value) {
    const html = marked(newValue) as string
    editor.value.commands.setContent(html, { emitUpdate: false })
  }
})

onBeforeUnmount(() => {
  editor.value?.destroy()
})
</script>

<style scoped>
.markdown-editor-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: white;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
}

.editor-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  padding: 0.75rem;
  background: #f8f9fa;
  border-bottom: 1px solid #e0e0e0;
}

.toolbar-btn {
  padding: 0.375rem 0.75rem;
  background: white;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s;
}

.toolbar-btn:hover {
  background: #f0f0f0;
  border-color: #007bff;
}

.toolbar-btn.is-active {
  background: #007bff;
  color: white;
  border-color: #007bff;
}

.toolbar-separator {
  width: 1px;
  background: #e0e0e0;
  margin: 0 0.25rem;
}

.editor-content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

.editor-content :deep(.ProseMirror) {
  min-height: 100%;
  outline: none;
}

.editor-content :deep(.ProseMirror p.is-editor-empty:first-child::before) {
  color: #adb5bd;
  content: attr(data-placeholder);
  float: left;
  height: 0;
  pointer-events: none;
}

/* Styles Markdown */
.editor-content :deep(.ProseMirror h1) {
  font-size: 1.75rem;
  font-weight: 700;
  margin: 1.5rem 0 1rem;
  color: #333;
}

.editor-content :deep(.ProseMirror h2) {
  font-size: 1.5rem;
  font-weight: 700;
  margin: 1.25rem 0 0.75rem;
  color: #333;
}

.editor-content :deep(.ProseMirror h3) {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 1rem 0 0.5rem;
  color: #333;
}

.editor-content :deep(.ProseMirror p) {
  margin: 0.5rem 0;
  line-height: 1.6;
  color: #333;
}

.editor-content :deep(.ProseMirror ul),
.editor-content :deep(.ProseMirror ol) {
  padding-left: 2rem;
  margin: 0.5rem 0;
}

.editor-content :deep(.ProseMirror li) {
  margin: 0.25rem 0;
}

.editor-content :deep(.ProseMirror code) {
  background: #f5f5f5;
  padding: 0.125rem 0.25rem;
  border-radius: 3px;
  font-family: monospace;
  font-size: 0.875em;
  color: #e83e8c;
}

.editor-content :deep(.ProseMirror pre) {
  background: #f5f5f5;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
  margin: 1rem 0;
}

.editor-content :deep(.ProseMirror pre code) {
  background: none;
  padding: 0;
  color: #333;
}

.editor-content :deep(.ProseMirror blockquote) {
  border-left: 4px solid #007bff;
  padding-left: 1rem;
  margin: 1rem 0;
  color: #666;
  font-style: italic;
}

.editor-content :deep(.ProseMirror hr) {
  border: none;
  border-top: 2px solid #e0e0e0;
  margin: 1.5rem 0;
}
</style>
