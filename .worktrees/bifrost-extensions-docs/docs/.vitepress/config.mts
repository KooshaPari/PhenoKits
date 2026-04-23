import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

import { defineConfig } from 'vitepress'

const docsDir = dirname(fileURLToPath(import.meta.url))
const phenodocsRoot = resolve(docsDir, '../../../phenodocs')
const phenodocsTheme = resolve(phenodocsRoot, '.vitepress/theme/index.ts')

export default defineConfig({
  title: 'Bifrost Extensions',
  description: 'Clean extension layer for the Bifrost LLM gateway',
  lastUpdated: true,
  ignoreDeadLinks: true,

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/development-guide' },
      { text: 'API', link: '/api' },
      { text: 'Roadmap', link: '/roadmap' },
      { text: 'Wiki', link: '/wiki' },
    ],
    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Overview', link: '/' },
          { text: 'Development Guide', link: '/development-guide' },
        ],
      },
      {
        text: 'Documentation',
        items: [
          { text: 'API Reference', link: '/api' },
          { text: 'AI/ML Architecture', link: '/AI_ML_ARCHITECTURE.md' },
          { text: 'Index', link: '/document-index' },
        ],
      },
      {
        text: 'Project',
        items: [
          { text: 'Roadmap', link: '/roadmap' },
          { text: 'Wiki', link: '/wiki' },
        ],
      },
    ],
  },

  vite: {
    resolve: {
      alias: {
        '@phenodocs-theme': phenodocsTheme,
      },
    },
    server: {
      fs: {
        allow: [phenodocsRoot],
      },
    },
  },
})
