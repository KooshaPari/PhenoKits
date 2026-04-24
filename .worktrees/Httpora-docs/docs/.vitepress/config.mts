import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Httpora',
  description: 'HTTP client/server toolkit with middleware support',
  base: '/httpora/',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'API', link: '/api' },
    ],
    sidebar: [
      {
        text: 'Guide',
        items: [
          { text: 'Overview', link: '/' },
          { text: 'Installation', link: '/installation' },
          { text: 'Quick Start', link: '/quickstart' },
        ]
      },
      {
        text: 'Reference',
        items: [
          { text: 'API Reference', link: '/api' },
          { text: 'Architecture', link: '/architecture' },
        ]
      }
    ]
  }
})
