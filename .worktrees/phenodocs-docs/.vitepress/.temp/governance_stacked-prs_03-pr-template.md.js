import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"PR Stack Template","description":"","frontmatter":{},"headers":[],"relativePath":"governance/stacked-prs/03-pr-template.md","filePath":"governance/stacked-prs/03-pr-template.md","lastUpdated":1772179526000}');
const _sfc_main = { name: "governance/stacked-prs/03-pr-template.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="pr-stack-template" tabindex="-1">PR Stack Template <a class="header-anchor" href="#pr-stack-template" aria-label="Permalink to &quot;PR Stack Template&quot;">​</a></h1><p>PR description should include these fields:</p><ul><li>Depends-on: <code>PR-1-id | none</code></li><li>Stack-Layer: L1 | L2 | L3 | L4 (maps to capabilities, work packages, tasks, deliverables)</li><li>Release-Channel: alpha | beta | rc | canary | release</li><li>State: optional override when lifecycle differs from release channel</li><li>Rollback-Plan: <code>command or rollback branch</code></li><li>Compatibility: BREAKING | NON_BREAKING</li><li>API-Shape-Impact: none | additive | breaking</li><li>Validation: lint, test, migration check with command list</li></ul><p>If API-Shape-Impact is <code>breaking</code>:</p><ul><li>child PRs that compile against this API cannot merge first.</li></ul></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("governance/stacked-prs/03-pr-template.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const _03PrTemplate = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  _03PrTemplate as default
};
