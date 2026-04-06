export const localeConfig = {
  root: {
    label: 'English',
    lang: 'en-US'
  },
  'zh-CN': {
    label: 'Simplified Chinese',
    lang: 'zh-CN'
  },
  'zh-TW': {
    label: 'Traditional Chinese',
    lang: 'zh-TW'
  },
  fa: {
    label: 'Persian',
    lang: 'fa-IR',
    dir: 'rtl'
  },
  'fa-Latn': {
    label: 'Persian Latin',
    lang: 'fa-Latn'
  }
}

export const nav = [
  { text: 'Archive', link: '/' },
  { text: 'Migration', link: '/guide/' },
  { text: 'Reference', link: '/reference/' }
]

export const sidebar = {
  '/guide/': [{ text: 'Guide', items: [{ text: 'Archived Status', link: '/guide/' }] }],
  '/reference/': [
    {
      text: 'Reference',
      items: [
        { text: 'Overview', link: '/reference/' },
        { text: 'Migration', link: '/reference/migration' },
        { text: 'Package', link: '/reference/package' }
      ]
    }
  ],
  '/zh-CN/': [{ text: 'Chinese', items: [{ text: 'Overview', link: '/zh-CN/' }] }],
  '/zh-TW/': [{ text: 'Traditional Chinese', items: [{ text: 'Overview', link: '/zh-TW/' }] }],
  '/fa/': [{ text: 'Persian', items: [{ text: 'Overview', link: '/fa/' }] }],
  '/fa-Latn/': [{ text: 'Persian Latin', items: [{ text: 'Overview', link: '/fa-Latn/' }] }]
}
