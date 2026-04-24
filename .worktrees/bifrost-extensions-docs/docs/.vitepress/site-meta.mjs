export function createSiteMeta({ base = '/' } = {}) {
  return {
    base,
    title: 'bifrost-extensions',
    description: 'Documentation',
    themeConfig: {
      nav: [
        { text: 'Home', link: base || '/' },
      ],
    },
  }
}
