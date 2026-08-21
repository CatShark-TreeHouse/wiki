# CatShark TreeHouse wiki

Astro + Starlight site in `frontend/short-shepherd/`; `just check` runs what CI runs. Start a session with `/familiarize` (reads the `NODE.md` tree).

## Branch Strategy
- `main` — stable; the only long-lived branch. All PRs target it. **Never push directly to `main`** (a GitHub ruleset enforces this).
- `feature/*` — a unit of new work, branched from `main` (e.g. `feature/<ticket>-short-slug`).
- `bug/*` — a bug fix, branched from `main`.
- `chore/*`, `docs/*`, `hotfix/*` — maintenance, documentation, and urgent fixes.

## Commits
- PRs merge via **Squash and merge** only (other methods are disabled), so `main` keeps **one commit per PR**; the branch itself may carry several granular commits. Merged branches auto-delete.
- Required CI checks (`format` · `lint` · `test`) must pass before merge.
- Local `/understand` + `/remember` briefings live in `.understand/` (tracked in the repo).
