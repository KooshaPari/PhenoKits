import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'phenotype-gauge',
  description: 'Modern benchmarking + xDD testing framework for Rust with statistical analysis',
  base: '/phenotype-gauge/',

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
        text: 'Testing Strategies',
        items: [
          { text: 'Property-Based Testing', link: '/guide/property/' },
          { text: 'Mutation Testing', link: '/guide/mutation/' },
          { text: 'Contract Testing', link: '/guide/contract/' },
          { text: 'Spec Testing', link: '/guide/spec/' },
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
      { icon: 'github', link: 'https://github.com/KooshaPari/phenotype-gauge' },
    ],

    footer: {
      message: 'Part of the Phenotype ecosystem.',
      copyright: 'MIT License',
    },
  },
})
