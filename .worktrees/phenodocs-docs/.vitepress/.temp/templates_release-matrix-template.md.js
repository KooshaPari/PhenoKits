import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Release Matrix Template","description":"","frontmatter":{},"headers":[],"relativePath":"templates/release-matrix-template.md","filePath":"templates/release-matrix-template.md","lastUpdated":1772179526000}');
const _sfc_main = { name: "templates/release-matrix-template.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="release-matrix-template" tabindex="-1">Release Matrix Template <a class="header-anchor" href="#release-matrix-template" aria-label="Permalink to &quot;Release Matrix Template&quot;">​</a></h1><p>Date: YYYY-MM-DD Program: <code>name</code> Channel: alpha|beta|rc|canary|release</p><table tabindex="0"><thead><tr><th>Task ID</th><th>Parent ID</th><th>PR Name</th><th>State</th><th>Depends On</th><th>Risks</th><th>Gate Result</th><th>Owner</th><th>Rollback</th><th>Notes</th></tr></thead><tbody><tr><td>1</td><td>-</td><td>feat/...-alpha-core</td><td>alpha</td><td>-</td><td>low</td><td>pass</td><td><code>owner</code></td><td>none</td><td>foundation</td></tr><tr><td>1.1</td><td>1</td><td>feat/...-beta-cache</td><td>beta</td><td>1</td><td>med</td><td>pass</td><td><code>owner</code></td><td><code>git revert &lt;sha&gt;</code></td><td>perf cache</td></tr><tr><td>1.2</td><td>1</td><td>feat/...-rc-renderer</td><td>rc</td><td>1</td><td>high</td><td>pass</td><td><code>owner</code></td><td>revert to legacy renderer</td><td>gated rollout</td></tr></tbody></table></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("templates/release-matrix-template.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const releaseMatrixTemplate = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  releaseMatrixTemplate as default
};
