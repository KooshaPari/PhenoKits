import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"WBS and DAG Rules","description":"","frontmatter":{},"headers":[],"relativePath":"governance/stacked-prs/02-wbs-and-dag.md","filePath":"governance/stacked-prs/02-wbs-and-dag.md","lastUpdated":1772179526000}');
const _sfc_main = { name: "governance/stacked-prs/02-wbs-and-dag.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="wbs-and-dag-rules" tabindex="-1">WBS and DAG Rules <a class="header-anchor" href="#wbs-and-dag-rules" aria-label="Permalink to &quot;WBS and DAG Rules&quot;">​</a></h1><p>Use <code>X.X.X.X</code> hierarchy with explicit state and dependency.</p><ul><li>Level 1: Capabilities</li><li>Level 2: Work packages</li><li>Level 3: Tasks</li><li>Level 4: Deliverables</li></ul><p>Template fields:</p><ul><li>task_id</li><li>title</li><li>state: alpha|beta|canary|rc|release</li><li>depends_on</li><li>parent_task</li><li>pr_name</li></ul><p>Example:</p><ul><li>A (alpha): playback engine foundation</li><li>B (beta): offline cache for playback (<code>depends_on: A</code>)</li><li>C (rc): web player renderer (<code>depends_on: A</code>, stricter channel)</li></ul><p>Rule: if B changes API used by C, B must be merged before C.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("governance/stacked-prs/02-wbs-and-dag.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const _02WbsAndDag = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  _02WbsAndDag as default
};
