export const localeConfig = {
  root: {
    label: 'English',
    lang: 'en-US',
  },
  'zh-CN': {
    label: 'Simplified Chinese',
    lang: 'zh-CN',
  },
  'zh-TW': {
    label: 'Traditional Chinese',
    lang: 'zh-TW',
  },
  fa: {
    label: 'Persian',
    lang: 'fa-IR',
    dir: 'rtl',
  },
  'fa-Latn': {
    label: 'Persian Latin',
    lang: 'fa-Latn',
  },
}

export const nav = [
  { text: 'Home', link: '/' },
  { text: 'Guide', link: '/guide/' },
  { text: 'API Reference', link: '/api/' },
  { text: 'Archive', link: '/archive/' },
]

export const sidebar = {
  '/guide/': [
    {
      text: 'Guide',
      items: [{ text: 'Getting Started', link: '/guide/' }],
    },
  ],
  '/api/': [
    {
      text: 'API Reference',
      items: [{ text: 'Registry and Discovery', link: '/api/' }],
    },
  ],
  '/archive/': [
    {
      text: 'Archive',
      items: [{ text: 'Migration Notes', link: '/archive/' }],
    },
  ],
  '/zh-CN/': [{ text: 'Chinese', items: [{ text: 'Overview', link: '/zh-CN/' }] }],
  '/zh-TW/': [{ text: 'Traditional Chinese', items: [{ text: 'Overview', link: '/zh-TW/' }] }],
  '/fa/': [{ text: 'Persian', items: [{ text: 'Overview', link: '/fa/' }] }],
  '/fa-Latn/': [{ text: 'Persian Latin', items: [{ text: 'Overview', link: '/fa-Latn/' }] }],
}
