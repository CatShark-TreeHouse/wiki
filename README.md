# CatShark TreeHouse Wiki

The CatShark TreeHouse network wiki: an Astro + Starlight site in
`frontend/short-shepherd` with the rules, joining guide, staff, moderation strategy,
incident records, ban register, and the controlled-content lists. Everything is static
and edited through pull requests; it deploys to GitHub Pages.

## Structure

| Path                                         | What                                                    |
|----------------------------------------------|---------------------------------------------------------|
| `frontend/short-shepherd/src/content/docs`   | The pages (Markdown / MDX)                              |
| `frontend/short-shepherd/src/data`           | Data files: controlled-content lists, staff, bans       |
| `frontend/short-shepherd/src/components`     | Components that render those data files                 |
| `.github/workflows`                          | CI (format, check, build) and the GitHub Pages deploy   |

## Development

Tasks are run via [`just`](https://github.com/casey/just):

```sh
just            # list recipes
just install    # npm install
just dev        # Astro dev server on :4321
just check      # what CI runs: format check, astro check, build
```

## Deployment

The domain is pure configuration; no code changes needed when it exists.
Everything is set through environment variables:

| Variable          | Where                | What                                                                                              |
| ----------------- | -------------------- | ------------------------------------------------------------------------------------------------- |
| `SITE_URL`        | frontend build       | Public origin of the wiki site; enables sitemap + canonical URLs (unset: build works, skips both) |
| `SITE_BASE`       | frontend build       | Sub-path for a GitHub Pages project site (e.g. `/wiki`); unset for a custom domain                |

## Contributing

- `main` is protected: **no direct pushes**.
- Every change lands through a **squash-merged pull request**.
- CI must pass before a PR can merge: `prettier --check`, `astro check`, `astro build`.

Run `just fmt` before pushing to keep CI green.
