import assert from 'node:assert/strict'
import { createSiteMeta } from '../../.vitepress/site-meta.mjs'
Deno.test('createSiteMeta returns valid object', () => {
  const m = createSiteMeta({ base: '/' })
  assert.equal(m.title, 'helMo')
})
