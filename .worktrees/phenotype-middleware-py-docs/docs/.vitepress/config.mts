import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid({
  title: 'phenotype-middleware-py',
  description: 'Python middleware patterns aligned with hexagonal architecture',
  appearance: 'dark',
  lastUpdated: true,
  themeConfig: {
    nav: [{ text: 'Home', link: '/' }],
    sidebar: [
      {
        text: 'Guide',
        items: [{ text: 'Getting Started', link: '/getting-started' }],
      },
    ],
    search: { provider: 'local' },
  },
  mermaid: { theme: 'dark' },
})
