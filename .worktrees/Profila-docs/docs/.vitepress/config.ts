import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'profiler',
  description: 'Project documentation',
  outDir: '../docs-dist',
  ignoreDeadLinks: true,
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide' }
    ],
    sidebar: [
      {
        text: 'Documentation',
        items: [
          { text: 'Overview', link: '/' },
          { text: 'Getting Started', link: '/guide' }
        ]
      }
    ]
  }
})
