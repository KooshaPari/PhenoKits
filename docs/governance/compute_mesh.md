# Phenotype Compute Mesh — 7-Node Hybrid Plan

## 1. Overview

The Phenotype org is migrating off the KooshaPari GitHub Actions billing tier (exhausted instantly by 65-agent swarms) onto a self-hosted hybrid compute mesh. Seven nodes — two Oracle Ampere VMs, a home Mac desktop, a GCP always-free micro VM, AWS Lambda, Cloudflare Workers, and a Hetzner spot VM (Phase 3) — are joined by a Tailscale overlay. Forgejo + Woodpecker on `oci-primary` replace GitHub Actions for internal CI; GitHub remains the public mirror. Runner labels route heavy jobs to the Mac, cheap jobs to OCI, burst to GCP, and edge handlers to Cloudflare. Vaultwarden on `oci-primary` is the canonical credential store — no plaintext env vars in repos.

## 2. Node Topology

| Node | Provider | Shape / Cost | MagicDNS | Role |
|------|----------|--------------|----------|------|
| `oci-primary` | Oracle OCI | Ampere A1, 2-4 OCPU, free | `forgejo.ts.internal` | Forgejo, Woodpecker server, Gitea Actions runner (`oci-cheap`), Vaultwarden |
| `oci-secondary` | Oracle OCI | Ampere A1, 2 OCPU, free | `obs.ts.internal` | Parallel runner (`oci-cheap`), Loki + Prometheus + Grafana |
| `home-desktop` | User Mac | M-series, owned | `home.ts.internal` | Heavy builds, GPU/VRAM work, runner label `heavy`, Parsec gaming pause |
| `gcp-micro` | GCP | e2-micro, always-free | `gcp.ts.internal` | Tertiary runner (`burst`), GCS artifact bucket |
| `aws-lambda` | AWS | 1M invokes/mo free | n/a (HTTPS) | Webhook fanout, cron fallback |
| `cloudflare-workers` | Cloudflare | free tier | `*.phenotype.workers.dev` | Edge ingress, R2 artifact CDN, Tunnel endpoints |
| `hetzner-spot` | Hetzner (Phase 3) | CPX11 spot, ~$4.50/mo | `spot.ts.internal` | Spillover when free tiers saturated |

## 3. Tailscale Mesh

All nodes join a single tailnet. MagicDNS gives stable names; ACLs gate who can reach Forgejo/Vaultwarden admin ports.

```bash
# Example: join oci-primary with an ephemeral auth key from Terraform
curl -fsSL https://tailscale.com/install.sh | sh
sudo tailscale up --authkey="${TS_AUTHKEY}" --hostname=oci-primary \
  --advertise-tags=tag:ci-server --ssh
```

ACL sketch (`tailscale-acl.hujson`):

| Source tag | Destination | Ports |
|------------|-------------|-------|
| `tag:runner` | `tag:ci-server` | 443 (Forgejo), 8000 (Woodpecker) |
| `tag:admin` | `tag:ci-server` | 22, 443, 8222 (Vaultwarden) |
| `tag:runner` | `tag:obs` | 3100 (Loki), 9090 (Prom push) |

Terraform provisions auth keys with `tailscale_tailnet_key` (ephemeral, tagged, 1-hour TTL).

## 4. Runner Routing

Woodpecker + Gitea Actions honor labels in `.woodpecker.yml` / `jobs.<id>.runs-on`. The scheduler uses first-match:

| Label | Target node(s) | Use for |
|-------|----------------|---------|
| `oci-cheap` | oci-primary, oci-secondary | Lint, unit tests, docs builds, small Rust crates |
| `heavy` | home-desktop | `cargo build --release` workspace, GPU, CUDA/Metal, large Docker builds |
| `burst` | gcp-micro | Overflow when OCI runners are saturated |
| `edge` | cloudflare-workers | Webhook transforms, edge-only jobs |
| `spillover` | hetzner-spot (Phase 3) | Declared only when free tiers full |

Convention: every job MUST carry at least one label. Default label in shared templates is `oci-cheap`; jobs that hit >2 GB memory, GPUs, or require macOS toolchains MUST declare `heavy`. A CI lint rejects un-labelled jobs.

## 5. Gaming-Mode Pause (home-desktop)

The Mac doubles as a Parsec host. Runner must pause when the user games so input latency is not perturbed by `cargo build`.

**Mechanism:** a sentinel file at `~/.phenotype/runner-paused`. A launchd agent watches Parsec's process list; when `parsecd` or a Parsec client session is detected, it `touch`es the sentinel. The Woodpecker agent wrapper aborts polling while the sentinel exists.

`~/Library/LaunchAgents/com.phenotype.runner-gate.plist` (scaffolded by setup script):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
  <key>Label</key><string>com.phenotype.runner-gate</string>
  <key>ProgramArguments</key>
  <array>
    <string>/usr/local/bin/phenotype-runner-gate</string>
  </array>
  <key>RunAtLoad</key><true/>
  <key>KeepAlive</key><true/>
  <key>StandardErrorPath</key><string>/tmp/phenotype-runner-gate.err</string>
  <key>StandardOutPath</key><string>/tmp/phenotype-runner-gate.out</string>
</dict>
</plist>
```

`phenotype-runner-gate` is a small Rust binary (per org scripting hierarchy) that polls `pgrep -x parsecd` every 2s and flips the sentinel. The Woodpecker agent is wrapped:

```bash
while :; do
  if [[ -f "$HOME/.phenotype/runner-paused" ]]; then sleep 5; continue; fi
  exec woodpecker-agent
done
```

Manual override: `touch ~/.phenotype/runner-paused` to pause, `rm` to resume.

## 6. Credential Checklist

The user must gather the following and paste into Vaultwarden (one collection per provider). Nothing below lands in repo env files.

### Oracle Cloud Infrastructure (OCI)
- Tenancy OCID
- User OCID
- Home region (e.g., `us-ashburn-1`)
- API key pair (private PEM + fingerprint)
- VCN OCID + public subnet OCID
- SSH public key for instance boot

### Cloudflare
- Account ID
- Scoped API tokens: Workers (edit), R2 (edit), DNS (edit), Tunnels (admin)
- Domain list (zones Terraform may touch)

### Vercel
- Scoped token from `vercel.com/account/tokens`
- Existing project list (name + project ID) to import

### Netlify
- PAT from `app.netlify.com/user/applications`
- Site list (slug + ID). Note: Cloudflare Pages may supersede Netlify in Phase 2; confirm before investing new infra here.

### AWS
- IAM user or access-key pair with `lambda:*` + `sns:*` + `logs:*` permissions (least-privilege policy recommended)
- Preferred region (e.g., `us-east-1`)

### GCP
- Service account JSON key (Compute Admin + Storage Admin scopes)
- Project ID
- Billing account linked (credit card required even for always-free tier)

### Tailscale
- Admin-panel API key (for Terraform-created ephemeral auth keys)
- Tailnet name

### Home-desktop
- User installs `act_runner` or `woodpecker-agent` locally
- User loads the launchd plist scaffolded by the setup script (`launchctl load ~/Library/LaunchAgents/com.phenotype.runner-gate.plist`)

## 7. Rollout (7 Steps)

1. **Day 1 — OCI primary provisioning.** Terraform creates `oci-primary` Ampere VM, installs Tailscale, Forgejo, Woodpecker server, and Vaultwarden behind Cloudflare Tunnel. Migrate 1-2 low-risk repos' CI to Woodpecker to validate the loop.
2. **Day 2 — OCI secondary + observability.** Provision `oci-secondary`, install second runner, stand up Loki + Prometheus + Grafana. Point Woodpecker logs + Forgejo metrics at the observability stack.
3. **Day 3 — Home-desktop runner.** Install Tailscale on the Mac, join tailnet, install `woodpecker-agent` with label `heavy`, scaffold + load the launchd gaming-pause plist. Route one repo's release build to `heavy` to validate.
4. **Day 4 — Cloudflare edge.** Configure Workers + R2 bucket for artifact CDN, provision Tunnel endpoints for Forgejo and Vaultwarden, wire DNS.
5. **Day 5 — GCP burst runner.** Provision `gcp-micro`, join tailnet, install runner with label `burst`, create GCS artifact bucket as secondary. Gate behind "OCI saturated" queue depth.
6. **Day 6 — AWS Lambda webhook fanout.** Deploy Lambda functions for Forgejo → downstream webhook distribution and cron fallbacks (in case OCI is down). SNS topic per event class.
7. **Phase 2 — Hetzner spot + hw-mesh crate.** When free tiers saturate, add `hetzner-spot` CPX11 (~$4.50/mo) with label `spillover`. Introduce `hw-mesh` crate (see §8) for agent-to-agent bus across nodes.

## 8. hw-mesh (Phase 2 Placeholder)

A new Rust crate at `phenotype-tooling/crates/hw-mesh` will provide an agent-to-agent bus over Tailscale, generalizing the existing `.argis-helios-bus/` JSONL pattern to be cross-machine. Out of scope for v1 of this doc; design notes will land with the Phase 2 spec. Expected surface: bus topics per tailnet node, at-least-once JSONL append semantics, optional gossip between runners for work-stealing.

## 9. Rollback / Kill-Switch

### Disable a single node
1. Tailscale admin panel → mark device as disabled (cuts network reachability without revoking keys).
2. Woodpecker admin UI → deregister runner so scheduler stops dispatching.
3. If compromised: rotate Tailscale auth key, re-image the node, rejoin.

### Flip back to GitHub Actions (global)
If `oci-primary` is lost and Forgejo is offline:
1. In each repo's `.forgejo/workflows/` or `.woodpecker.yml`, the mirror under `.github/workflows/` remains (kept in sync by a one-way replication rule). Re-enable by toggling the workflow's `on:` triggers — GitHub Actions resumes.
2. Accept the billing hit explicitly for the duration; open an incident note in `worklogs/GOVERNANCE.md`.
3. Remove required-status-check rules that reference Woodpecker:
   ```bash
   gh api repos/KooshaPari/<repo>/branches/main/protection/required_status_checks -X DELETE
   ```
4. Once `oci-primary` is restored, re-disable GitHub workflow triggers and re-add Woodpecker required checks.

### Vaultwarden loss
Vaultwarden DB lives on `oci-primary` block volume with nightly Backblaze B2 snapshot. Rebuild = restore snapshot on a fresh OCI VM, re-point Tailscale MagicDNS `forgejo.ts.internal` via ACL/DNS record update. Credentials are never recoverable from repos; rotation of any lost secret is the fallback.

### Kill-switch summary

| Failure | First response | Fallback |
|---------|----------------|----------|
| Single runner down | Deregister in Woodpecker; queue reroutes by label | Spin up `burst` or `spillover` |
| oci-primary down | Cloudflare Tunnel returns 503; pager on `obs.ts.internal` fires | Re-enable GitHub Actions per repo |
| Tailscale outage | SSH over public IP (OCI) with tight sg | Restart `tailscaled`; rotate auth key |
| Vaultwarden DB corruption | Restore Backblaze snapshot | Rotate any uncertain secrets |
| GCP / AWS billing alert | Revoke runner label; cap invocations | Hetzner spot absorbs load |
