import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Merge Scenarios","description":"","frontmatter":{},"headers":[],"relativePath":"governance/stacked-prs/04-merge-scenarios.md","filePath":"governance/stacked-prs/04-merge-scenarios.md","lastUpdated":1772179526000}');
const _sfc_main = { name: "governance/stacked-prs/04-merge-scenarios.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="merge-scenarios" tabindex="-1">Merge Scenarios <a class="header-anchor" href="#merge-scenarios" aria-label="Permalink to &quot;Merge Scenarios&quot;">​</a></h1><h2 id="tree-example" tabindex="-1">Tree example <a class="header-anchor" href="#tree-example" aria-label="Permalink to &quot;Tree example&quot;">​</a></h2><p>A: playback core (alpha) ├─ B: cache layer (beta) └─ C: advanced renderer (beta/rc)</p><p>C is in the same feature family only if C depends on A (directly or transitively). If C is renderer risk, it must stay at beta/rc even if A is on main.</p><h2 id="rule-of-merge-order" tabindex="-1">Rule of merge order <a class="header-anchor" href="#rule-of-merge-order" aria-label="Permalink to &quot;Rule of merge order&quot;">​</a></h2><ul><li>Never merge a child PR before its required parent PR.</li><li>Do not merge PR-3 with API contracts from PR-2 unless PR-2 is merged.</li><li>Prefer small PR slices, each passing its own gates.</li></ul><h2 id="practical-check-before-merge" tabindex="-1">Practical check before merge <a class="header-anchor" href="#practical-check-before-merge" aria-label="Permalink to &quot;Practical check before merge&quot;">​</a></h2><ul><li>All depends_on IDs resolved</li><li>Rollback plan is real and tested</li><li>Channel label aligns with risk and migration strategy</li></ul></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("governance/stacked-prs/04-merge-scenarios.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const _04MergeScenarios = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  _04MergeScenarios as default
};
