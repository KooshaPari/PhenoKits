# AGENTS.md — docs

## Project Overview

- **Name**: docs
- **Description**: Documentation hub for the Phenotype ecosystem - guides, specs, and reference
- **Location**: `/Users/kooshapari/CodeProjects/Phenotype/repos/docs`
- **Language Stack**: Markdown, VitePress (Vue/JavaScript)
- **Published**: Internal (Phenotype ecosystem)

## Quick Start Commands

```bash
# Navigate to docs
cd /Users/kooshapari/CodeProjects/Phenotype/repos/docs

# Install dependencies
npm install

# Start dev server
npm run docs:dev

# Build documentation
npm run docs:build

# Preview built docs
npm run docs:preview
```

## Architecture

```
docs/
├── .vitepress/               # VitePress configuration
│   ├── config.mjs            # Main config
│   ├── theme/                # Custom theme
│   └── dist/                 # Build output
├── api/                      # API documentation
├── assets/                   # Static assets (images, etc.)
├── cli/                      # CLI documentation
├── contributing/             # Contribution guidelines
├── dashboard/                # Dashboard docs
├── fa/                       # Farsi documentation
├── fa-Latn/                  # Farsi (Latin script)
├── guide/                    # User guides
├── hexagonal-ports.md        # Hexagonal architecture spec
├── hexagonal-spec.md         # Hexagonal spec details
├── index.md                  # Homepage
├── reference/                # Reference documentation
│   └── sessions/             # Session documentation
└── specs/                    # Specifications
```

## Quality Standards

### Content Standards
- **UTF-8 encoding only**: No smart quotes, em-dashes, or special characters
- **Line length**: 100 characters for readability
- **Frontmatter required**: All `.md` files need proper VitePress frontmatter

### Validation
```bash
# Validate encoding (requires AgilePlus)
cd /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus
agileplus validate-encoding --all --fix
```

### Markdown Style
- Use ATX-style headers (`#` not underline)
- Fenced code blocks with language tags
- Consistent bullet points (`-` not `*`)

## Git Workflow

### Branch Naming
Format: `docs/<type>/<description>`

Types: `feat`, `fix`, `chore`, `refactor`

Examples:
- `docs/feat/api-reference-v2`
- `docs/fix/typos-guide`

### Commit Format
```
<type>(docs): <description>

Examples:
- docs(specs): add hexagonal ports specification
- fix(docs): correct CLI command examples
```

## File Structure

```
docs/
├── .vitepress/
│   ├── config.mjs           # Site configuration
│   ├── theme/
│   │   ├── index.js         # Theme entry
│   │   └── custom.css       # Custom styles
│   └── dist/                # Build output (gitignored)
├── [content directories]
│   ├── index.md             # Landing page
│   └── ...
└── package.json             # Node dependencies
```

## CLI Commands

```bash
# Development
npm run docs:dev              # Start dev server (localhost:5173)
npm run docs:build            # Build for production
npm run docs:preview          # Preview production build

# Quality
npm run docs:build 2>&1 | grep -i error  # Check for build errors
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Dev server won't start | Check `npm install` completed |
| Build fails | Check for broken links or bad frontmatter |
| Encoding issues | Run AgilePlus validation |
| Theme not applying | Verify `.vitepress/theme/` structure |

## Dependencies

- **VitePress**: Static site generator
- **Vue**: Underlying framework
- **Node.js**: Runtime (see `package.json` for version)

## Multi-Language Support

- `fa/` - Farsi (Persian) documentation
- `fa-Latn/` - Farsi in Latin script
- Default: English at root

## Agent Notes

When working in docs:
1. All content changes need proper frontmatter
2. Images go in `assets/` directory
3. Follow VitePress conventions for links
4. Check encoding with AgilePlus validator
5. Cross-reference related specs in `specs/` directory
