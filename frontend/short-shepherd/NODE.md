---
path: frontend/short-shepherd
charted: 2026-08-21
fs:
  - name: astro.config.mjs
    role: Starlight config, sidebar (new pages go here), fonts, remark plugin, SITE_URL/SITE_BASE
    node: false
  - name: src/
    role: pages, data, components, helpers
    node: true
  - name: public/
    role: static assets; evidence/ holds transcript HTML + PNGs linked from bewares/incidents; favicon.png, flag.png
    node: false
  - name: package.json
    role: scripts (dev, build, check, format, format:check); deps astro, @astrojs/starlight, fontsource Inter + Bricolage Grotesque
    node: false
  - name: tsconfig.json
    role: TS config
    node: false
  - name: .prettierrc.json
    role: prettier + prettier-plugin-astro
    node: false
  - name: .vscode/
    role: editor recommendations
    node: false
  - name: README.md
    role: stock Starlight readme
    node: false
---
**Is:** The Astro + Starlight site ("short-shepherd") that is the whole wiki.

**Conventions:** Run npm scripts from this directory (or via root `just`). Every new page must be added to the sidebar in `astro.config.mjs`. Internal links are written root-relative; the remark plugin prefixes the base.

**Entry points:** `astro.config.mjs`, `src/content/docs/index.mdx`.
