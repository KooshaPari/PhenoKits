import { defineConfig } from 'vitepress'
import type { ConfigOptions } from '../types/index.ts'
import { deepMerge } from '../utils/config-merger.ts'

/**
 * Create a VitePress config pre-loaded with the Phenotype keycap theme defaults.
 *
 * Consumers call this in their `.vitepress/config.mts`:
 * ```ts
 * import { createPhenotypeConfig } from '@phenotype/docs/config'
 * export default createPhenotypeConfig({ title: 'My Project', description: '...' })
 * ```
 */
export function createPhenotypeConfig(options: ConfigOptions) {
  const {
    title,
    description,
    base = '/',
    srcDir = 'docs',
    githubOrg = 'KooshaPari',
    githubRepo,
    nav = [],
    sidebar = {},
    overrides = {},
  } = options

  const repoSlug = githubRepo ?? title.toLowerCase().replace(/\s+/g, '-')

  const baseConfig = defineConfig({
    title,
    description,
    lang: 'en-US',
    srcDir,
    base,
    lastUpdated: true,
    cleanUrls: true,

    head: [
      ['link', { rel: 'icon', href: `${base}favicon.ico` }],
    ],

    themeConfig: {
      siteTitle: title,

      nav,

      sidebar,

      socialLinks: [
        { icon: 'github', link: `https://github.com/${githubOrg}/${repoSlug}` },
      ],

      footer: {
        message: 'Released under the MIT License.',
        copyright: `Copyright \u00a9 ${new Date().getFullYear()} Phenotype`,
      },

      search: {
        provider: 'local',
      },

      editLink: {
        pattern: `https://github.com/${githubOrg}/${repoSlug}/edit/main/${srcDir}/:path`,
        text: 'Edit this page on GitHub',
      },

      outline: {
        level: [2, 3],
        label: 'On this page',
      },

      externalLinkIcon: true,
    },

    markdown: {
      lineNumbers: true,
      theme: {
        light: 'github-light',
        dark: 'github-dark',
      },
    },

    ignoreDeadLinks: true,
  })

  // Deep-merge consumer overrides on top of base config
  return deepMerge(baseConfig, overrides) as ReturnType<typeof defineConfig>
}
