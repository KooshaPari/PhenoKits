# Workflow Rollout PRs
Generated: 2026-04-05

## PRs Created Successfully
| Repo | PR URL | Status |
|------|--------|--------|
| PhenoObservability | https://github.com/KooshaPari/PhenoObservability/pull/1 | Created |
| ResilienceKit | https://github.com/KooshaPari/ResilienceKit/pull/1 | Created |
| TestingKit | https://github.com/KooshaPari/TestingKit/pull/1 | Created |
| PhenoHandbook | https://github.com/KooshaPari/PhenoHandbook/pull/1 | Created |

## Already Existed (Skipped)
| Repo | PR URL |
|------|--------|
| DataKit | https://github.com/KooshaPari/DataKit/pull/1 |
| PhenoKits | https://github.com/KooshaPari/PhenoKits/pull/1 |
| AuthKit | https://github.com/KooshaPari/AuthKit/pull/1 |
| GDK | https://github.com/KooshaPari/GDK/pull/3 |
| BytePort | https://github.com/KooshaPari/BytePort/pull/14 |

## Failed to Create
| Repo | Error | Category |
|------|-------|----------|
| Conft | no git remotes found | Configuration |
| PhenoDevOps | branch has no history in common with main | Git History |
| phenoSDK | Could not resolve to a Repository | Repository Not Found |
| PhenoProject | branch has no history in common with main | Git History |
| agslag-docs | Repository was archived so is read-only | Archived |
| chatta | Repository was archived so is read-only | Archived |
| FixitRs | Repository was archived so is read-only | Archived |
| go-nippon | Repository was archived so is read-only | Archived |
| McpKit | No commits between main and chore/add-reusable-workflows | Empty Branch |
| netweave-final2 | Repository was archived so is read-only | Archived |
| PhenoMCP | No commits between main and chore/add-reusable-workflows | Empty Branch |
| PhenoRuntime | No commits between main and chore/add-reusable-workflows | Empty Branch |
| PhenoSpecs | No commits between main and chore/add-reusable-workflows | Empty Branch |
| phenotype-registry | No commits between main and chore/add-reusable-workflows | Empty Branch |
| PlayCua | No commits between main and chore/add-reusable-workflows | Empty Branch |
| AtomsBot | Repository was archived so is read-only | Archived |
| canvasApp | Repository was archived so is read-only | Archived |
| KDesktopVirt | No commits between main and chore/add-reusable-workflows | Empty Branch |
| kmobile | Repository was archived so is read-only | Archived |
| localbase3 | Repository was archived so is read-only | Archived |
| pheno | No commits between main and chore/add-reusable-workflows | Empty Branch |
| PhenoPlugins | No commits between main and chore/add-reusable-workflows | Empty Branch |
| PhenoProc | No commits between main and chore/add-reusable-workflows | Empty Branch |
| Tracely | No commits between main and chore/add-reusable-workflows | Empty Branch |
| zen | Repository was archived so is read-only | Archived |

## Special Cases (Configuration Issues)
| Repo | Issue |
|------|-------|
| koosha-portfolio | Origin remote points to PhenoKits.git instead of its own repo |
| artifacts | Origin remote points to PhenoKits.git instead of its own repo |
| Profila | Repository not found locally |
| ObservabilityKit | Repository not found locally |

## Summary
- **Total Repos Checked**: 38
- **PRs Created**: 4
- **Already Existed**: 5
- **Failed**: 25

### Failure Breakdown
| Category | Count |
|----------|-------|
| Archived repositories | 11 |
| Empty branch (no commits to merge) | 11 |
| Git history issues (orphan branch) | 2 |
| Missing remote configuration | 1 |
| Repository not found | 1 |

## Recommendations

### For Failed PRs

1. **Archived Repositories (11 repos)**:
   - agslag-docs, chatta, FixitRs, go-nippon, netweave-final2, AtomsBot, canvasApp, kmobile, localbase3, zen
   - These repos are archived on GitHub and cannot receive PRs
   - Consider unarchiving if workflows are still needed

2. **Empty Branches (11 repos)**:
   - McpKit, PhenoMCP, PhenoRuntime, PhenoSpecs, phenotype-registry, PlayCua, KDesktopVirt, pheno, PhenoPlugins, PhenoProc, Tracely
   - The `chore/add-reusable-workflows` branch exists but has no unique commits
   - The branch may have been reset or was never populated with workflow files
   - Need to re-run the workflow installation script

3. **Git History Issues (2 repos)**:
   - PhenoDevOps, PhenoProject
   - The workflow branch has no common history with main (likely an orphan branch)
   - Need to recreate the branch from main and add workflows

4. **Configuration Issues (1 repo)**:
   - Conft: Has no git remotes configured
   - Need to add origin remote pointing to correct GitHub repository

### For Special Cases

1. **koosha-portfolio & artifacts**:
   - Origin remote misconfigured (points to PhenoKits.git)
   - Need to fix remote: `git remote set-url origin git@github.com:KooshaPari/<repo>.git`
   - Then push branch and create PR

2. **Profila & ObservabilityKit**:
   - Repositories not found in /Users/kooshapari/CodeProjects/Phenotype/repos
   - May be in different location or have different names
   - Verify repository names and locations
