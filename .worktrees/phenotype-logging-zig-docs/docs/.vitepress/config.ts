import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Phenotype Logging Zig',
  description: 'Structured logging library for Zig with minimal allocations',
  base: '/phenotype-logging-zig/',

  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'API', link: '/api/' },
      { text: 'Examples', link: '/examples/' }
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Getting Started',
          items: [
            { text: 'Installation', link: '/guide/getting-started' },
            { text: 'Basic Usage', link: '/guide/basic-usage' },
            { text: 'Configuration', link: '/guide/configuration' }
          ]
        },
        {
          text: 'Advanced',
          items: [
            { text: 'Custom Adapters', link: '/guide/custom-adapters' },
            { text: 'Performance', link: '/guide/performance' },
            { text: 'Zero Allocation', link: '/guide/zero-allocation' }
          ]
        }
      ],
      '/api/': [
        {
          text: 'API Reference',
          items: [
            { text: 'Overview', link: '/api/' },
            { text: 'Core Types', link: '/api/core' },
            { text: 'Level', link: '/api/level' },
            { text: 'Entry', link: '/api/entry' },
            { text: 'Logger', link: '/api/logger' }
          ]
        },
        {
          text: 'Adapters',
          items: [
            { text: 'Stderr Adapter', link: '/api/adapters/stderr' },
            { text: 'File Adapter', link: '/api/adapters/file' },
            { text: 'Custom Adapters', link: '/api/adapters/custom' }
          ]
        },
        {
          text: 'Interface',
          items: [
            { text: 'Transport Interface', link: '/api/interface' }
          ]
        }
      ],
      '/examples/': [
        {
          text: 'Examples',
          items: [
            { text: 'Overview', link: '/examples/' },
            { text: 'Basic Logging', link: '/examples/basic' },
            { text: 'Structured Logging', link: '/examples/structured' },
            { text: 'File Rotation', link: '/examples/file-rotation' },
            { text: 'Custom Adapter', link: '/examples/custom-adapter' }
          ]
        }
      ]
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/phenotype-logging-zig' }
    ],

    footer: {
      message: 'Released under the MIT License',
      copyright: 'Copyright 2026 Phenotype Project'
    },

    search: {
      provider: 'local'
    }
  }
})
