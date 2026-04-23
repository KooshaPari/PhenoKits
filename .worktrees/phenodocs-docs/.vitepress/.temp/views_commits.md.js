import { resolveComponent, useSSRContext } from "vue";
import { ssrRenderAttrs, ssrRenderComponent, ssrRenderStyle } from "vue/server-renderer";
import { _ as _export_sfc } from "./plugin-vue_export-helper.1tPrXgE0.js";
const __pageData = JSON.parse('{"title":"Commit log (rich view)","description":"","frontmatter":{},"headers":[],"relativePath":"views/commits.md","filePath":"views/commits.md","lastUpdated":1774350903000}');
const _sfc_main = { name: "views/commits.md" };
function _sfc_ssrRender(_ctx, _push, _parent, _attrs, $props, $setup, $data, $options) {
  const _component_CommitLog = resolveComponent("CommitLog");
  _push(`<div${ssrRenderAttrs(_attrs)}><h1 id="commit-log-rich-view" tabindex="-1">Commit log (rich view) <a class="header-anchor" href="#commit-log-rich-view" aria-label="Permalink to &quot;Commit log (rich view)&quot;">​</a></h1><p>Sample <strong>git-style</strong> history for the documentation hub. Data is loaded from <code>.vitepress/data/commit-log.json</code> (replace in CI with output from <code>git log</code> across federated repos).</p>`);
  _push(ssrRenderComponent(_component_CommitLog, null, null, _parent));
  _push(`<h2 id="producing-data-locally" tabindex="-1">Producing data locally <a class="header-anchor" href="#producing-data-locally" aria-label="Permalink to &quot;Producing data locally&quot;">​</a></h2><div class="language-bash vp-adaptive-theme line-numbers-mode"><button title="Copy Code" class="copy"></button><span class="lang">bash</span><pre class="shiki shiki-themes github-light github-dark vp-code" tabindex="0"><code><span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#6A737D", "--shiki-dark": "#6A737D" })}"># Example: last 20 commits as JSON (shape must match commit-log.json)</span></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#6F42C1", "--shiki-dark": "#B392F0" })}">git</span><span style="${ssrRenderStyle({ "--shiki-light": "#032F62", "--shiki-dark": "#9ECBFF" })}"> log</span><span style="${ssrRenderStyle({ "--shiki-light": "#005CC5", "--shiki-dark": "#79B8FF" })}"> -20</span><span style="${ssrRenderStyle({ "--shiki-light": "#005CC5", "--shiki-dark": "#79B8FF" })}"> --pretty=format:</span><span style="${ssrRenderStyle({ "--shiki-light": "#032F62", "--shiki-dark": "#9ECBFF" })}">&#39;{&quot;sha&quot;:&quot;%h&quot;,&quot;date&quot;:&quot;%cs&quot;,&quot;author&quot;:&quot;%an&quot;,&quot;subject&quot;:&quot;%s&quot;,&quot;repo&quot;:&quot;phenodocs&quot;},&#39;</span><span style="${ssrRenderStyle({ "--shiki-light": "#005CC5", "--shiki-dark": "#79B8FF" })}"> \\</span></span>
<span class="line"><span style="${ssrRenderStyle({ "--shiki-light": "#D73A49", "--shiki-dark": "#F97583" })}">  |</span><span style="${ssrRenderStyle({ "--shiki-light": "#6F42C1", "--shiki-dark": "#B392F0" })}"> sed</span><span style="${ssrRenderStyle({ "--shiki-light": "#032F62", "--shiki-dark": "#9ECBFF" })}"> &#39;1s/^/[/; $s/,$/]/&#39;</span><span style="${ssrRenderStyle({ "--shiki-light": "#D73A49", "--shiki-dark": "#F97583" })}"> &gt;</span><span style="${ssrRenderStyle({ "--shiki-light": "#032F62", "--shiki-dark": "#9ECBFF" })}"> .vitepress/data/commit-log.json</span></span></code></pre><div class="line-numbers-wrapper" aria-hidden="true"><span class="line-number">1</span><br><span class="line-number">2</span><br><span class="line-number">3</span><br></div></div><p>(Adjust quoting for strict JSON; use <code>jq</code> in production.)</p></div>`);
}
const _sfc_setup = _sfc_main.setup;
_sfc_main.setup = (props, ctx) => {
  const ssrContext = useSSRContext();
  (ssrContext.modules || (ssrContext.modules = /* @__PURE__ */ new Set())).add("views/commits.md");
  return _sfc_setup ? _sfc_setup(props, ctx) : void 0;
};
const commits = /* @__PURE__ */ _export_sfc(_sfc_main, [["ssrRender", _sfc_ssrRender]]);
export {
  __pageData,
  commits as default
};
