import test from 'node:test'
import assert from 'node:assert/strict'

import { getSiteMeta } from '../helpers/site-meta.mjs'

test('site metadata exposes archive navigation and locales', () => {
  const meta = getSiteMeta()

  assert.equal(meta.title, 'phenotype-docs-engine')
  assert.ok(meta.nav.some((item) => item.text === 'Migration' && item.link === '/guide/'))
  assert.deepEqual(meta.locales.sort(), ['fa', 'fa-Latn', 'zh-CN', 'zh-TW'])
})
