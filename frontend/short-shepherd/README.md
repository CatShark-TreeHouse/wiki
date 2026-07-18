# short-shepherd

The CatShark TreeHouse wiki site — [Astro](https://astro.build) +
[Starlight](https://starlight.astro.build).

Static pages (rules, joining, staff…) live in `src/content/docs/`. The
[Banned & Spoilered page](src/content/docs/controlled.mdx) is different: it
fetches the live controlled-content lists from the backend API in the browser
(`src/components/ControlledLists.astro`).

## Commands

| Command                | Action                                               |
| :--------------------- | :--------------------------------------------------- |
| `npm install`          | Install dependencies                                 |
| `npm run dev`          | Dev server at `localhost:4321`                       |
| `npm run build`        | Production build to `./dist/`                        |
| `npm run preview`      | Preview the production build                         |
| `npm run check`        | `astro check` (types + content)                      |
| `npm run format`       | Prettier write (run before pushing to keep CI green) |
| `npm run format:check` | Prettier check (what CI runs)                        |

## Configuration

Both knobs are baked in at build time; neither is needed for local work.

- `SITE_URL` — public origin of the deployed site (e.g.
  `https://wiki.example.net`). Enables the sitemap and canonical URLs; unset,
  the build just skips them.
- `PUBLIC_API_BASE` — origin of the backend API. Unset, it defaults to
  same-origin in production and `http://localhost:8080` in dev (matching
  `just run-api`).

## Theme

The look lives in `src/styles/custom.css` (ocean-teal dark, sunlit light) and
`astro.config.mjs`. Fonts are self-hosted via Fontsource: Bricolage Grotesque
for headings, Inter for body text.
