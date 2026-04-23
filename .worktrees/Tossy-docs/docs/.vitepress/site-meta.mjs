export function createSiteMeta({ base = '/' } = {}) {
  return {
    base,
    title: 'trash-cli',
    description: 'Documentation',
    themeConfig: {
      nav: [
        { text: 'Home', link: base || '/' },
      ],
    },
  }
}
