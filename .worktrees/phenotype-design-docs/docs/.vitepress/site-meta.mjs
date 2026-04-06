export function createSiteMeta({ base = '/' } = {}) {
  return {
    base,
    title: 'phenotype-design',
    description: 'Documentation',
    themeConfig: {
      nav: [
        { text: 'Home', link: base || '/' },
      ],
    },
  }
}
