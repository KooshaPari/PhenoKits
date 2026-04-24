# phenotype.dev DNS Configuration (Cloudflare)

Configure these records in your Cloudflare DNS control panel.

## Primary Domain (phenotype.dev)

### Option A: Vercel Nameservers (Recommended)

Point your domain registrar to Vercel's nameservers. Vercel will manage all DNS records.

1. In Vercel project → Settings → Domains
2. Copy the nameserver list (typically 4 nameservers)
3. In your registrar's DNS settings, replace nameservers with Vercel's list
4. Allow 24-48 hours for propagation

**Advantages:**
- Vercel auto-configures SSL/TLS
- Automatic DNS propagation
- No manual A/CNAME records needed

### Option B: CNAME / A Records (Manual)

Use this if you want to keep Cloudflare as your DNS provider.

#### A Record (root domain)
| Field | Value |
|-------|-------|
| Type | A |
| Name | @ (or phenotype.dev) |
| IPv4 Address | `76.76.19.103` |
| TTL | 3600 (1 hour) |
| Proxy | DNS only (gray cloud) |

**Note:** Vercel's IPv4 address may change. Get the authoritative address from Vercel project settings.

#### CNAME Record (www subdomain)
| Field | Value |
|-------|-------|
| Type | CNAME |
| Name | www |
| Target | `cname.vercel-dns.com` |
| TTL | 3600 (1 hour) |
| Proxy | DNS only (gray cloud) |

## Subdomains

For federated docsite subdomains (if using separate Vercel projects):

### focalpoint.phenotype.dev
| Field | Value |
|-------|-------|
| Type | CNAME |
| Name | focalpoint |
| Target | `cname.vercel-dns.com` |
| TTL | 3600 |

### docs.phenotype.dev (if separate from main site)
| Field | Value |
|-------|-------|
| Type | CNAME |
| Name | docs |
| Target | `cname.vercel-dns.com` |
| TTL | 3600 |

### api.phenotype.dev (if API subdomain needed)
| Field | Value |
|-------|-------|
| Type | CNAME |
| Name | api |
| Target | `cname.vercel-dns.com` |
| TTL | 3600 |

## Cloudflare Settings (if using Option B)

1. **SSL/TLS Mode**: Full (Strict)
   - Vercel provides automatic SSL certificates
2. **Page Rules**: (Optional)
   - Always HTTPS: `phenotype.dev/*` → Force HTTPS
   - Cache Level: Cache Everything (for static content)
3. **Email Routing**: (Optional)
   - Forward `contact@phenotype.dev` to `kooshapari@gmail.com`

## Verification

After configuring DNS, verify propagation:

```bash
# Check A record
dig phenotype.dev +short
# Expected: Vercel's IPv4

# Check CNAME
dig www.phenotype.dev +short
# Expected: cname.vercel-dns.com

# Check propagation globally
# Use: https://whatsmydns.net
```

## SSL/TLS Certificate

- **Automatic** via Vercel (no manual setup required)
- Certificate issued by Cloudflare or Let's Encrypt depending on Vercel config
- Auto-renews 30 days before expiration
- Accessible at: https://phenotype.dev (green lock)

## Troubleshooting

**DNS not resolving after 24 hours?**
1. Verify nameservers were updated at registrar (not just Cloudflare)
2. Clear your local DNS cache: `sudo dscacheutil -flushcache` (macOS)
3. Try from different network/device to rule out local cache

**Cloudflare showing "name server" error?**
- Ensure you're using Cloudflare's nameservers at your registrar
- Cloudflare tab should show "Active Nameservers"

**SSL certificate shows "Not Trusted"?**
- Wait 15-30 minutes for Vercel to issue certificate
- Force refresh: Ctrl+Shift+R in browser
- Check Vercel dashboard → Deployments → SSL/TLS status
