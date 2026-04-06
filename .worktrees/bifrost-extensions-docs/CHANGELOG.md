# Changelog

All notable changes to bifrost-extensions are documented here.
Format: [CalVer](https://calver.org) — `YEAR.MONTH(WAVE).PATCH`

## [Unreleased]

### Bug Fixes

- **ci**: Simplify pages-deploy workflow - remove phenodocs, split build/deploy jobs (#110) ([e9e8151](https://github.com/KooshaPari/bifrost-extensions/commit/e9e81519afee4f323bf590122d9bdae187ea0eff))

### Chores

- Sync local commits to main (#111) ([c0edfd1](https://github.com/KooshaPari/bifrost-extensions/commit/c0edfd10f8fd13f50d285e96421dc35d6c56d5ea))
- Rename Go module to bifrost-extensions, bump x/net to v0.52.0, add CSS baseline (#112) ([3df7ded](https://github.com/KooshaPari/bifrost-extensions/commit/3df7dede82154ad1cb8bb0273832bef6c8734dff))
- Rename module to bifrost-extensions, bump golang.org/x/net, add impeccable CSS (#113) ([dc57aeb](https://github.com/KooshaPari/bifrost-extensions/commit/dc57aeb2c5a1f5a0493f04e69f4ceeb0aa474e2c))

## [0.1.0-golem] - 2026-03-28

### Bug Fixes

- **ci**: Use PR head SHA for merge commit detection (#3) ([8dd0914](https://github.com/KooshaPari/bifrost-extensions/commit/8dd0914e3ccdf46376dad305dd3ed8cba2d2999f))
- Improve error handling in alert sync workflow (#18) ([7162f84](https://github.com/KooshaPari/bifrost-extensions/commit/7162f84cbaad82b38ce173275fb8084032384efc))
- Remove broken local replace directive for bifrost/core (#13) ([44c2116](https://github.com/KooshaPari/bifrost-extensions/commit/44c21169e844b9924954ed3e843e15cc554756fc))
- GitHub Pages deployment (#52) ([e6b8d0d](https://github.com/KooshaPari/bifrost-extensions/commit/e6b8d0dc5d4e353f63059eb3b93f7303bb299e5e))
- Resolve build blockers and dependency issues (#30) ([56c92e3](https://github.com/KooshaPari/bifrost-extensions/commit/56c92e3f807a3c74b0bf5da3ec6efc74435a0dc4))
- Resolve remaining build blockers and dependency issues (#61) ([d72bf6e](https://github.com/KooshaPari/bifrost-extensions/commit/d72bf6e2e834b9a3ede7b1920563f27b381b907b))
- Resolve build blockers and align with origin/main (#67) ([131e782](https://github.com/KooshaPari/bifrost-extensions/commit/131e782dac6a048343ef689012a22db4ad57a34f))
- Resolve build blockers, dependency issues, and GitHub Pages deployment (#87) ([ecd45dd](https://github.com/KooshaPari/bifrost-extensions/commit/ecd45ddd42f55adce5870726a5e6ea9e096cee30))
- Resolve GitHub Pages build and deployment errors (#91) ([5a72909](https://github.com/KooshaPari/bifrost-extensions/commit/5a72909a3094736f64c681a1be953e3ffdbc3645))
- Resolve build blockers and align with latest origin/main (#86) ([96f232b](https://github.com/KooshaPari/bifrost-extensions/commit/96f232b59561c45188e21d1a46e5fb6d67e6e75e))
- Resolve build blockers and dependency alignment (#94) ([1e5add3](https://github.com/KooshaPari/bifrost-extensions/commit/1e5add3f51f195032a694b654a8e1a8c16136b33))
- Prevent security guard CI timeout when GITGUARDIAN_API_KEY is missing (#98) ([40d109f](https://github.com/KooshaPari/bifrost-extensions/commit/40d109ff256d46e0a231a738c1425107485c0e3a))
- Resolve build blockers - update module paths and remove local replacements (#104) ([f56276d](https://github.com/KooshaPari/bifrost-extensions/commit/f56276d4ca84c531628d7a54a24047994cf8440f))

### CI/CD

- **policy**: Enforce layered fix PR gate (#2) ([fbc3f4b](https://github.com/KooshaPari/bifrost-extensions/commit/fbc3f4b3b1979b697d0bcc7ee4770990b5b01d62))
- Add automated alert-to-issue sync workflow (#5) ([257e52a](https://github.com/KooshaPari/bifrost-extensions/commit/257e52afb7f5ad84e7f0ac685adcebdb92a9f9a8))
- Fix GitHub Pages deployment and mass docs synchronization (#55) ([f760d23](https://github.com/KooshaPari/bifrost-extensions/commit/f760d232ef0ae8283475bc304cc13ae1aa9ed85c))
- Fix GitHub Pages deployment by adding workflow_dispatch and updating build type (#68) ([913accf](https://github.com/KooshaPari/bifrost-extensions/commit/913accffb9d9178213beca8d0543da3c27ec70f4))

### Chores

- Chore/review bot governance wave1 (#1)

* docs: add git-backed documentation governance scaffold

* chore: standardize CodeRabbit and Gemini review policy

Apply repo-level bot review config and rate-limit governance.

Co-authored-by: Codex <noreply@openai.com>

* chore(governance): stabilize review bot policy

Set CodeRabbit PR blocking to info and codify review retrigger and rate-limit queue rules in agent governance docs.

Co-authored-by: Codex <noreply@openai.com>

* docs(governance): codify review bot retrigger protocol

Document CodeRabbit and Gemini retrigger commands plus rate-limit queue policy for agents.

Co-authored-by: Codex <noreply@openai.com>

---------

Co-authored-by: Codex <noreply@openai.com> ([22b55e8](https://github.com/KooshaPari/bifrost-extensions/commit/22b55e8c2804f3cbd7ad40889cb1010562466639))
- Migrate to composite policy-gate action (#7) ([060675f](https://github.com/KooshaPari/bifrost-extensions/commit/060675fac96c4bcff0b35024dcacb86e96ddbb6c))
- Add SECURITY.md and .gitignore governance files (#8) ([9a57b7e](https://github.com/KooshaPari/bifrost-extensions/commit/9a57b7eab402a1c02b9b64c372fb33a28af90799))
- Migrate to composite policy-gate action (#9) ([07312a1](https://github.com/KooshaPari/bifrost-extensions/commit/07312a1fdf220185a2db88a5f65064a6e2aa6c00))
- Add lint-test workflow and golangci config (#11) ([b9d2608](https://github.com/KooshaPari/bifrost-extensions/commit/b9d260865558168801b4b79c306d1811b2d8a762))
- Standardize CodeRabbit and Gemini review policy (#21) ([858c844](https://github.com/KooshaPari/bifrost-extensions/commit/858c844fdd9adc50fb70f5cc80f67c1be742adde))
- Roll out policy-gate merge-token workflows (#24) ([aca454c](https://github.com/KooshaPari/bifrost-extensions/commit/aca454c6bd800a971f79a65ecb72755562a89820))
- Policy gate rollout (#53) ([5866935](https://github.com/KooshaPari/bifrost-extensions/commit/58669355f7c581de7e4f58e6455d2207b18f8b3f))
- Standardize CodeRabbit and Gemini review policy (#41) ([9ec83f2](https://github.com/KooshaPari/bifrost-extensions/commit/9ec83f2d3b92e58038f710a87902507b72a50b7a))
- Add lint-test workflow and golangci config (#33) ([29a1aec](https://github.com/KooshaPari/bifrost-extensions/commit/29a1aec0a59e9dee48059166bcc8473f27389bf1))
- Add spec documentation (#63) ([69a7b64](https://github.com/KooshaPari/bifrost-extensions/commit/69a7b6442fa3f1641945d83ea0562b626d2440fe))
- Remove obsolete version from docker-compose files (#85) ([d95a63a](https://github.com/KooshaPari/bifrost-extensions/commit/d95a63ac0fae01ce50f0702db6ecf382aa053163))
- Add spec documentation (PRD, ADR, FR, PLAN, trackers) (#89) ([49321cf](https://github.com/KooshaPari/bifrost-extensions/commit/49321cfc689075681af010a5621388ed14212434))
- Standardize CodeRabbit and Gemini review policy (#80) ([8813e8e](https://github.com/KooshaPari/bifrost-extensions/commit/8813e8e1fee1c470efaccb6febdc9866d593479c))
- **governance**: Remove duplicated governance blocks, reference thegent templates (#100) ([fe9b343](https://github.com/KooshaPari/bifrost-extensions/commit/fe9b34380bb1cd29bf2653ef18a65039c3e06524))

### Documentation

- Unify VitePress docs categories (#4) ([5d5b47a](https://github.com/KooshaPari/bifrost-extensions/commit/5d5b47aed6e106efd78f1a6edd079153d0acc8e9))
- Mass sync main (#50) ([1e9564d](https://github.com/KooshaPari/bifrost-extensions/commit/1e9564dda1e295a25e14d60ed6038f0335e0ca50))
- Add comprehensive feature comparison matrix (#88) ([0fefddd](https://github.com/KooshaPari/bifrost-extensions/commit/0fefddd10c3bee505c7cd7c129cae947d3a2e27a))
- Add comprehensive feature comparison matrix (#90) ([d05caf6](https://github.com/KooshaPari/bifrost-extensions/commit/d05caf640f9518c44dd11e7d6403aa5863979ed8))
- Replace stub spec docs with real content derived from codebase (#99) ([9c20784](https://github.com/KooshaPari/bifrost-extensions/commit/9c20784a042a749b4547227eb8808138d8cd6e5c))
- Add FR-SLM section covering local SLM integration layer (#101) ([34e2989](https://github.com/KooshaPari/bifrost-extensions/commit/34e298948ac945ffdc74d46214118acb669b77cd))
- Add spec docs and directory structure (#107) ([405073d](https://github.com/KooshaPari/bifrost-extensions/commit/405073d061ae638f2627fa9aecd98ac7a5222403))

### Features

- Create standalone packages for modularity (#25) ([be87fa1](https://github.com/KooshaPari/bifrost-extensions/commit/be87fa1819f5c33e669eed5d4530dc2bcc145c26))
- Modernize tooling and CI workflows (#26) ([fab6089](https://github.com/KooshaPari/bifrost-extensions/commit/fab60898ede033c18a5a6eb6a6790557d7e5d932))
- Add phenotype-go-kit integration modules for config, middleware, and CLI (#103) ([f161098](https://github.com/KooshaPari/bifrost-extensions/commit/f1610987611e020118f50b1fcb973980fc423205))
- **deps**: Migrate from phenotype-go-kit monolith to standalone packages (#102) ([5b461ed](https://github.com/KooshaPari/bifrost-extensions/commit/5b461ed0a9e1906d92013a73b67d3769115fd577))

### Refactoring

- Integrate phenotype-go-kit modules (#6) ([e095496](https://github.com/KooshaPari/bifrost-extensions/commit/e095496369558b9a87185e593acabcc6517edc57))
- Migrate to standalone phenotype-go-middleware and phenotype-go-config (#10) ([f68fd78](https://github.com/KooshaPari/bifrost-extensions/commit/f68fd78a4e5cdefa5ed23006988f6daf200d37eb))
- Phenotype-go-kit integration (config, middleware, CLI) (#95) ([f93ad39](https://github.com/KooshaPari/bifrost-extensions/commit/f93ad391a3ab73652bfc066baa940f32bfc80d10))

### Testing

- Add unit tests for contentsafety and intelligentrouter plugins (#106) ([569628d](https://github.com/KooshaPari/bifrost-extensions/commit/569628d99e19b1c72632fedbb64c3936b4877c80))

### Restore

- Recover bifrost-extensions from Augment checkpoints ([7237a26](https://github.com/KooshaPari/bifrost-extensions/commit/7237a2693270c378176c807102a67856b6ecf394))


