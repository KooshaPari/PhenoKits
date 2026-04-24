import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Changelog (rich view)","description":"","frontmatter":{},"headers":[],"relativePath":"views/changelog.md","filePath":"views/changelog.md","lastUpdated":1774350903000}');
const _sfc_main = { name: "views/changelog.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="changelog-rich-view" tabindex="-1">Changelog (rich view) <a class="header-anchor" href="#changelog-rich-view" aria-label="Permalink to &quot;Changelog (rich view)&quot;">​</a></h1><p>This page will host a <strong>rich changelog</strong> for the hub: version headers, <strong>Added / Changed / Fixed / Removed</strong> groupings, and links to PRs or commits.</p><h2 id="today" tabindex="-1">Today <a class="header-anchor" href="#today" aria-label="Permalink to &quot;Today&quot;">​</a></h2><ul><li>Prefer the repository <strong><code>CHANGELOG.md</code></strong> at the PhenoDocs root (or per federated package) as the source of truth.</li><li>For static sites, either <strong>mirror</strong> the markdown here or <strong>generate</strong> a JSON slice in CI from Keep a Changelog–compatible files.</li></ul><h2 id="next" tabindex="-1">Next <a class="header-anchor" href="#next" aria-label="Permalink to &quot;Next&quot;">​</a></h2><ul><li>Style blocks per version with <code>DocStatusBadge</code> or custom callouts.</li><li>Optional: import <code>changelog.json</code> alongside markdown for dashboard-style filtering.</li></ul><p>See also <a href="/planning/rich-workspace-views">planning: rich workspace views</a>.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("views/changelog.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const changelog = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  changelog as default
};
