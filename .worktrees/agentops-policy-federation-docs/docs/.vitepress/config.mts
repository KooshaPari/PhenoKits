import { withMermaid } from 'vitepress-plugin-mermaid'

export default withMermaid({
  title: 'AgentOps Policy Federation',
  description: 'Single source of truth for agent/devops governance, scope federation, and runtime extensions.',
  appearance: 'dark',
  lastUpdated: true,
  themeConfig: {
    nav: [{ text: 'Home', link: '/' }],
    sidebar: [],
    search: { provider: 'local' },
  },
  mermaid: { theme: 'dark' },
})
