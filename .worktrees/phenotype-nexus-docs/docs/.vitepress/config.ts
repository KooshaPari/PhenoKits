import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'phenotype-nexus',
  description: 'In-process service registry and discovery for microservices — Rust',
  base: '/phenotype-nexus/',

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'API', link: '/api/' },
    ],

    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Overview', link: '/' },
          { text: 'Getting Started', link: '/guide/' },
        ],
      },
      {
        text: 'Reference',
        items: [
          { text: 'API Reference', link: '/api/' },
          { text: 'Architecture', link: '/architecture/' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/phenotype-nexus' },
    ],

    footer: {
      message: 'Part of the Phenotype ecosystem.',
      copyright: 'MIT License',
    },
  },
})
