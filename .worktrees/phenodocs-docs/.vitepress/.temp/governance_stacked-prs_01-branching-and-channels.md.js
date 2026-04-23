import { ssrRenderAttrs } from "vue/server-renderer";
import { useSSRContext } from "vue";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Branching and Release Channels","description":"","frontmatter":{},"headers":[],"relativePath":"governance/stacked-prs/01-branching-and-channels.md","filePath":"governance/stacked-prs/01-branching-and-channels.md","lastUpdated":1772179526000}');
const _sfc_main = { name: "governance/stacked-prs/01-branching-and-channels.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="branching-and-release-channels" tabindex="-1">Branching and Release Channels <a class="header-anchor" href="#branching-and-release-channels" aria-label="Permalink to &quot;Branching and Release Channels&quot;">​</a></h1><h2 id="branching-baseline" tabindex="-1">Branching Baseline <a class="header-anchor" href="#branching-baseline" aria-label="Permalink to &quot;Branching Baseline&quot;">​</a></h2><ul><li>Canonical branches track <code>main</code> state.</li><li>Feature, analysis, and QA work happens in <code>PROJECT-wtrees/&lt;topic&gt;</code>.</li><li>PR prep and merges stay in stacks from worktree branches.</li></ul><h2 id="channel-rules" tabindex="-1">Channel Rules <a class="header-anchor" href="#channel-rules" aria-label="Permalink to &quot;Channel Rules&quot;">​</a></h2><ul><li><code>alpha</code>: early code, guarded by flags and unit tests.</li><li><code>beta</code>: feature-complete slice; integration smoke + migration checks.</li><li><code>rc</code>: release-candidate validation; requires rollback path.</li><li><code>canary</code>: selective, pre-prod exposure; strict rollback monitor.</li><li><code>release</code>: full gating and release checklist complete.</li></ul><p>Do not merge alpha work into release paths.</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("governance/stacked-prs/01-branching-and-channels.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const _01BranchingAndChannels = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  _01BranchingAndChannels as default
};
