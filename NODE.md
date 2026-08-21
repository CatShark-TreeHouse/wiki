---
path: .
charted: 2026-08-21
fs:
  - name: frontend/short-shepherd/
    role: the Astro + Starlight wiki site (the only code)
    node: true
  - name: .github/workflows/
    role: ci.yml (format:check, astro check, build) and pages.yml (GitHub Pages deploy)
    node: false
  - name: Justfile
    role: task runner; `just check` is the gate, `just dev`, `just fmt`, `just install`
    node: false
  - name: README.md
    role: structure, dev, deploy env vars (SITE_URL, SITE_BASE), contributing
    node: false
  - name: .chartignore
    role: /chart ignore rules
    node: false
  - name: texput.log
    role: stray TeX log, ignore
    node: false
---
**Is:** The CatShark TreeHouse network wiki repo: a static Starlight site (rules, joining, staff, moderation, incidents, bans, controlled-content lists) edited via PRs and deployed to GitHub Pages.

**Conventions:** `main` is protected; squash-merge PRs only. Gate before a PR: `just check`. Prettier formats everything. No em dashes in wiki prose. Static lists change only through PR edits to JSON.

**Entry points:** `Justfile`, then `frontend/short-shepherd/astro.config.mjs`.

**Refs:** README.md; memory `project-file-system-map`, `no-em-dashes-in-wiki-prose`, `controlled-content-status-terms`, `bot-moderation-abandoned-frontend-focus`.

**Reading this tree:** To understand any path, read every `NODE.md` from the root down to that directory; each node states only what its ancestors have not. Entries with `node: false` are fully described by their `fs` line and have no `NODE.md` of their own.
