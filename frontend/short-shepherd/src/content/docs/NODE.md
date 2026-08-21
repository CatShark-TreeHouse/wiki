---
path: frontend/short-shepherd/src/content/docs
charted: 2026-08-21
fs:
  - name: index.mdx
    role: landing page
    node: false
  - name: controlled.mdx
    role: controlled-content lists page (ControlledLists component)
    node: false
  - name: start-here/
    role: welcome.md, joining.md
    node: false
  - name: rules/
    role: network-rules.md (numbered clauses, source of § anchors), controlled-content.md (policy)
    node: false
  - name: community/
    role: faq.md, roles-and-teams.md, staff.mdx (StaffTeams)
    node: false
  - name: moderation/
    role: strategy.mdx (PowersDiagram), bans.mdx (BanList), bewares/ (index.mdx list + one page per person, e.g. patel.mdx)
    node: false
  - name: incidents/
    role: index.md plus one dated record per incident (260819.md)
    node: false
---
**Is:** The wiki pages, one directory per sidebar section.

**Conventions:** Plain prose pages are `.md`; pages that embed a component are `.mdx`. Incident files are named YYMMDD. Bewares get a per-person page under `moderation/bewares/` with evidence in `public/evidence/`. Headings in `rules/network-rules.md` use `<span class="cs-num">n.n</span>` so anchors stay linkable; do not change that format.

**Entry points:** `index.mdx`.
