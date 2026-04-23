import { resolveComponent, useSSRContext } from "vue";
import { ssrRenderAttrs, ssrRenderComponent } from "vue/server-renderer";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Development Guide","description":"","frontmatter":{},"headers":[],"relativePath":"development/index.md","filePath":"development/index.md","lastUpdated":1772197872000}');
const _sfc_main = { name: "development/index.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  const _component_CategorySwitcher = resolveComponent("CategorySwitcher");
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="development-guide" tabindex="-1">Development Guide <a class="header-anchor" href="#development-guide" aria-label="Permalink to &quot;Development Guide&quot;">​</a></h1>`);
  _push(ssrRenderComponent(_component_CategorySwitcher, null, null, _parent));
  _push(`<p>Engineering setup, architecture notes, and contributor workflows.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("development/index.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const index = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  index as default
};
