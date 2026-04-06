import { resolveComponent, useSSRContext } from "vue";
import { ssrRenderAttrs, ssrRenderComponent } from "vue/server-renderer";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Document Index","description":"","frontmatter":{},"headers":[],"relativePath":"index/index.md","filePath":"index/index.md","lastUpdated":1774350903000}');
const _sfc_main = { name: "index/index.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  const _component_CategorySwitcher = resolveComponent("CategorySwitcher");
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="document-index" tabindex="-1">Document Index <a class="header-anchor" href="#document-index" aria-label="Permalink to &quot;Document Index&quot;">​</a></h1>`);
  _push(ssrRenderComponent(_component_CategorySwitcher, null, null, _parent));
  _push(`<p>Index buckets:</p><ul><li><a href="./raw-all">Raw/All</a></li><li><a href="./planning">Planning</a></li><li><a href="./specs">Specs</a></li><li><a href="./research">Research</a></li><li><a href="./worklogs">Worklogs</a></li><li><a href="./other">Other</a></li></ul><p>Rich <strong>workspace views</strong> (changelog, commit log, WBS hub): <a href="/views/">Workspace views</a>.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("index/index.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const index = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  index as default
};
