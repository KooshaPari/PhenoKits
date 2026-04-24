import { resolveComponent, useSSRContext } from "vue";
import { ssrRenderAttrs, ssrRenderComponent } from "vue/server-renderer";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"PhenoDocs","description":"","frontmatter":{},"headers":[],"relativePath":"index.md","filePath":"index.md","lastUpdated":1772197872000}');
const _sfc_main = { name: "index.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  const _component_CategorySwitcher = resolveComponent("CategorySwitcher");
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="phenodocs" tabindex="-1">PhenoDocs <a class="header-anchor" href="#phenodocs" aria-label="Permalink to &quot;PhenoDocs&quot;">​</a></h1>`);
  _push(ssrRenderComponent(_component_CategorySwitcher, null, null, _parent));
  _push(`<p>Welcome to the unified docs surface.</p><h2 id="super-categories" tabindex="-1">Super Categories <a class="header-anchor" href="#super-categories" aria-label="Permalink to &quot;Super Categories&quot;">​</a></h2><ul><li><a href="/wiki/">Wiki (User Guides)</a></li><li><a href="/development/">Development Guide</a></li><li><a href="/index/">Document Index</a></li><li><a href="/api/">API</a></li><li><a href="/roadmap/">Roadmap</a></li></ul></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("index.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const index = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  index as default
};
