import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid({
  title: 'phenotype-gauge',
  description: 'Modern benchmarking and xDD testing framework for Rust with statistical analysis and HTML reports',
  base: '/phenotype-gauge/',
  appearance: 'dark',
  lastUpdated: true,
  ignoreDeadLinks: true,
  locales: {
    root: {
      label: 'English',
      lang: 'en-US',
      themeConfig: {
        nav: [
          { text: 'Home', link: '/' },
          { text: 'Guide', link: '/guide/' },
          { text: 'Reference', link: '/reference/api' },
        ],
        sidebar: [
          {
            text: 'Guide',
            items: [
              { text: 'Overview', link: '/guide/' },
              { text: 'Getting Started', link: '/getting-started' },
            ],
          },
          {
            text: 'Reference',
            items: [
              { text: 'API Reference', link: '/reference/api' },
            ],
          },
        ],
      },
    },
    'zh-CN': {
      label: '简体中文',
      lang: 'zh-CN',
      link: '/zh-CN/',
      themeConfig: {
        nav: [
          { text: '首页', link: '/zh-CN/' },
          { text: '指南', link: '/guide/' },
          { text: '参考', link: '/reference/api' },
        ],
      },
    },
    'zh-TW': {
      label: '繁體中文',
      lang: 'zh-TW',
      link: '/zh-TW/',
      themeConfig: {
        nav: [
          { text: '首頁', link: '/zh-TW/' },
          { text: '指南', link: '/guide/' },
          { text: '參考', link: '/reference/api' },
        ],
      },
    },
    fa: {
      label: 'فارسی',
      lang: 'fa-IR',
      link: '/fa/',
      dir: 'rtl',
      themeConfig: {
        nav: [
          { text: 'خانه', link: '/fa/' },
          { text: 'راهنما', link: '/guide/' },
          { text: 'مرجع', link: '/reference/api' },
        ],
      },
    },
    'fa-Latn': {
      label: 'Farsi (Latin)',
      lang: 'fa',
      link: '/fa-Latn/',
      themeConfig: {
        nav: [
          { text: 'Home', link: '/fa-Latn/' },
          { text: 'Guide', link: '/guide/' },
          { text: 'Reference', link: '/reference/api' },
        ],
      },
    },
  },
  themeConfig: {
    socialLinks: [
      { icon: 'github', link: 'https://github.com/KooshaPari/phenotype-gauge' },
    ],
    search: { provider: 'local' },
  },
  mermaid: { theme: 'dark' },
})
