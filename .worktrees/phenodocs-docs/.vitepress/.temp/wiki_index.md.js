import { resolveComponent, useSSRContext } from "vue";
import { ssrRenderAttrs, ssrRenderComponent } from "vue/server-renderer";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Wiki (User Guides)","description":"","frontmatter":{},"headers":[],"relativePath":"wiki/index.md","filePath":"wiki/index.md","lastUpdated":1772197872000}');
const _sfc_main = { name: "wiki/index.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  const _component_CategorySwitcher = resolveComponent("CategorySwitcher");
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="wiki-user-guides" tabindex="-1">Wiki (User Guides) <a class="header-anchor" href="#wiki-user-guides" aria-label="Permalink to &quot;Wiki (User Guides)&quot;">​</a></h1>`);
  _push(ssrRenderComponent(_component_CategorySwitcher, null, null, _parent));
  _push(`<p>User-focused how-to and operational guidance.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("wiki/index.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const index = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  index as default
};
