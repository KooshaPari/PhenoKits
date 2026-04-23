import { defineConfig } from 'vitepress'

export default defineConfig({
  title: "Phenotype Agent Core",
  description: "Multi-Agent Orchestration Platform",

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'API', link: '/api/' },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Installation', link: '/guide/installation' },
            { text: 'Quick Start', link: '/guide/quickstart' },
            { text: 'Configuration', link: '/guide/configuration' },
          ]
        },
        {
          text: 'Core Concepts',
          items: [
            { text: 'Agents', link: '/guide/agents' },
            { text: 'Providers', link: '/guide/providers' },
            { text: 'Tools', link: '/guide/tools' },
            { text: 'Skills', link: '/guide/skills' },
          ]
        },
      ],
      '/api/': [
        {
          text: 'API Reference',
          items: [
            { text: 'Python API', link: '/api/python' },
            { text: 'CLI Reference', link: '/api/cli' },
            { text: 'REST API', link: '/api/rest' },
          ]
        },
      ],
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/phenotype-agent-core' }
    ]
  }
})
