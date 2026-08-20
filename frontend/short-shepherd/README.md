# short-shepherd

The CatShark TreeHouse wiki site: [Astro](https://astro.build) +
[Starlight](https://starlight.astro.build).

Pages live in `src/content/docs/`. The
[Banned & Controlled page](src/content/docs/controlled.mdx) renders
`src/data/controlled-content.json` at build time
(`src/components/ControlledLists.astro`); edit that file through a pull
request to change the lists.

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

- `SITE_URL`: public origin of the deployed site (e.g.
  `https://wiki.example.net`). Enables the sitemap and canonical URLs; unset,
  the build just skips them.
- `SITE_BASE`: sub-path the site is served from, e.g. `/wiki` for a GitHub
  Pages project site. Unset for a custom domain or local work.

## Theme

The look lives in `src/styles/custom.css` (a Bluesky-style shell in gold and charcoal) and
`astro.config.mjs`. Fonts are self-hosted via Fontsource: Bricolage Grotesque
for headings, Inter for body text.
