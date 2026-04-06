export function createSiteMeta({ base = '/' } = {}) {
  return {
    base,
    title: 'profiler',
    description: 'Documentation',
    themeConfig: {
      nav: [
        { text: 'Home', link: base || '/' },
      ],
    },
  }
}
