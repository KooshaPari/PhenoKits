export function createSiteMeta({ base = '/' } = {}) {
  return {
    base,
    title: 'helMo',
    description: 'helMo documentation',
    themeConfig: { nav: [{ text: 'Home', link: base || '/' }, { text: 'Guide', link: '/guide/' }] },
  }
}
