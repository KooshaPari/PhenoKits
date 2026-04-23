import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Workspace views","description":"","frontmatter":{},"headers":[],"relativePath":"views/index.md","filePath":"views/index.md","lastUpdated":1774352049000}');
const _sfc_main = { name: "views/index.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="workspace-views" tabindex="-1">Workspace views <a class="header-anchor" href="#workspace-views" aria-label="Permalink to &quot;Workspace views&quot;">​</a></h1><p>Structured <strong>modules</strong> (and pages) for plan-, research-, spec-, and work-oriented reading—plus <strong>roadmap</strong>, <strong>changelog</strong>, <strong>git commit log</strong>, and <strong>WBS/DAG</strong> surfaces.</p><table tabindex="0"><thead><tr><th>View</th><th>Page</th><th>Notes</th></tr></thead><tbody><tr><td>Changelog (rich)</td><td><a href="./changelog">changelog</a></td><td>Categories, versions, link to source <code>CHANGELOG</code> where present</td></tr><tr><td>Commit log</td><td><a href="./commits">commits</a></td><td>Table view from generated JSON (<code>&lt;CommitLog /&gt;</code>)</td></tr><tr><td>WBS &amp; DAG</td><td><a href="./wbs">wbs</a></td><td>Links governance WBS docs; future graph view</td></tr></tbody></table><p><strong>Index buckets</strong> for markdown corpora: <a href="/index/">Document index</a> — Planning, Specs, Research, Worklogs.</p><p><strong>Plan:</strong> <a href="/planning/rich-workspace-views">Rich workspace views</a> (roadmap for implementation).</p><p><strong>Shipping:</strong> <a href="/guides/full-turn-delivery">Full-turn delivery</a> (PR merge, changelog, version, docs). <strong>Next batch:</strong> <a href="/planning/full-turn-next50-20260326">Full-turn Next 50</a> — five agents, ten waves, DAG.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("views/index.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const index = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  index as default
};
