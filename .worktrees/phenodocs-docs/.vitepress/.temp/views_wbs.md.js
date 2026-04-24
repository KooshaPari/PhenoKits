import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"WBS & DAG views","description":"","frontmatter":{},"headers":[],"relativePath":"views/wbs.md","filePath":"views/wbs.md","lastUpdated":1774352049000}');
const _sfc_main = { name: "views/wbs.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="wbs-dag-views" tabindex="-1">WBS &amp; DAG views <a class="header-anchor" href="#wbs-dag-views" aria-label="Permalink to &quot;WBS &amp; DAG views&quot;">​</a></h1><p><strong>PhenoDocs batch plans:</strong> <a href="/planning/full-turn-next50-20260326">Full-turn Next 50</a> — five lanes, ten waves, <strong>Depends</strong> edges.</p><p><strong>Work breakdown structures</strong> and <strong>dependency graphs (DAG)</strong> for engineering work live primarily under <strong>Governance → Stacked PRs</strong>:</p><ul><li><a href="/governance/stacked-prs/02-wbs-and-dag">WBS and DAG</a></li><li><a href="/governance/stacked-prs/01-branching-and-channels">Branching and channels</a></li><li><a href="/governance/stacked-prs/">Overview</a></li></ul><h2 id="future-rich-elements" tabindex="-1">Future rich elements <a class="header-anchor" href="#future-rich-elements" aria-label="Permalink to &quot;Future rich elements&quot;">​</a></h2><ul><li><strong>On-page DAG:</strong> mermaid or SVG generated from the same WBS source.</li><li><strong>Cross-repo WBS:</strong> aggregate from multiple repos’ <code>docs/</code> or manifest files (see <a href="/planning/rich-workspace-views">planning</a>).</li></ul></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("views/wbs.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const wbs = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  wbs as default
};
