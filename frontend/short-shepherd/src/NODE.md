---
path: frontend/short-shepherd/src
charted: 2026-08-21
fs:
  - name: content/docs/
    role: the wiki pages (md/mdx), one directory per sidebar section; no NODE.md inside (Astro would publish it)
    node: false
  - name: content.config.ts
    role: docs collection with Starlight's stock loader/schema
    node: false
  - name: data/
    role: static JSON lists: controlled-content.json, bans.json, bewares.json, staff.json (edited via PRs)
    node: false
  - name: components/
    role: renderers for data/: ControlledLists, BanList, BewareList, BewareEntry, StaffTeams, PowersDiagram; PageTitle overrides Starlight's title
    node: false
  - name: lib/
    role: rules.ts turns "§ n.n" strings into links to Network Rules anchors (built from the md source)
    node: false
  - name: plugins/
    role: remark-base-links.mjs prefixes root-relative links with SITE_BASE
    node: false
  - name: styles/
    role: custom.css theme overrides
    node: false
  - name: assets/
    role: flag.png, paw.png used by the site
    node: false
---
**Is:** Site source: content pages plus the JSON data and the components that render it.

**Pages (`content/docs/`):** `index.mdx` (landing), `controlled.mdx` (ControlledLists), `start-here/` (welcome, joining), `rules/` (network-rules.md with numbered clauses that are the source of § anchors; controlled-content.md policy), `community/` (faq, roles-and-teams, staff.mdx), `moderation/` (strategy.mdx, bans.mdx, `bewares/` index plus one page per person with evidence in `public/evidence/`), `incidents/` (index plus one YYMMDD record per incident). Plain prose pages are `.md`; pages embedding a component are `.mdx`. Headings in `rules/network-rules.md` use `<span class="cs-num">n.n</span>` so anchors stay linkable; do not change that format.

**Conventions:** Data-driven pages are a thin `.mdx` importing a component from `components/` that reads `data/*.json`. Controlled-content aliases are single whitespace-free tokens; statuses are banned/controlled. Rule references in data use "§ n.n" so `lib/rules.ts` can link them.

**Entry points:** `content/docs/index.mdx`, `components/`, `data/`.
