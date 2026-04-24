import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Stacked PR & Release Governance","description":"","frontmatter":{},"headers":[],"relativePath":"governance/stacked-prs/README.md","filePath":"governance/stacked-prs/README.md","lastUpdated":1772179526000}');
const _sfc_main = { name: "governance/stacked-prs/README.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="stacked-pr-release-governance" tabindex="-1">Stacked PR &amp; Release Governance <a class="header-anchor" href="#stacked-pr-release-governance" aria-label="Permalink to &quot;Stacked PR &amp; Release Governance&quot;">​</a></h1><p>Use this module for all roadmap, feature, and review planning.</p><ul><li><a href="./01-branching-and-channels">01 Branching and Channels</a></li><li><a href="./02-wbs-and-dag">02 WBS + DAG Rules</a></li><li><a href="./03-pr-template">03 PR Template</a></li><li><a href="./04-merge-scenarios">04 Merge Scenarios</a></li><li><a href="./05-pr-reconciliation">05 PR Reconciliation</a></li><li><a href="./../../templates/release-matrix-template">06 Release Matrix Template</a></li></ul><p>General rules:</p><ul><li>Keep canonical folders on <code>main</code>.</li><li>Use <code>PROJECT-wtrees/&lt;topic&gt;</code> for all work, QA, and PR prep.</li><li>No top-level merge until required dependencies are in place.</li><li>UI/renderer changes require beta/rc policy unless duplicate-safe.</li></ul></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("governance/stacked-prs/README.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const README = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  README as default
};
