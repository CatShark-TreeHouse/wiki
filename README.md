# CatShark TreeHouse — Wiki

Monorepo for the Zuri Cat Tree / FurrK network wiki: a Telegram bot for moderators to
maintain the controlled-content lists, an HTTP API that serves the live ban/spoiler
lists, and an Astro site that displays the rules (static) and the controlled-content
lists (dynamic).

The bot is information-access only — it edits the lists, it does not moderate users
(user moderation is handled by Group Guardian).

## Structure

| Path | What |
|------|------|
| `backend/domain` | Core types and repository traits |
| `backend/persistence` | In-memory and SQLite implementations of the repositories |
| `backend/bot` | Telegram bot: `/add_ban`, `/add_spoiler`, `/check` (admin-gated); DMs the joining checklist to users requesting to join |
| `backend/api` | axum HTTP API + process entrypoint (runs the API, and the bot if `TELOXIDE_TOKEN` is set) |
| `frontend/short-shepherd` | Astro + Starlight site |

## Development

Tasks are run via [`just`](https://github.com/casey/just):

```sh
just            # list recipes
just build      # build the Rust workspace
just test       # run backend tests
just run-api    # run the API (and bot, if TELOXIDE_TOKEN is set) on :8080
just fe-dev     # run the Astro dev server
```

## Deployment

The domain is pure configuration — no code changes needed when it exists.
Everything is set through environment variables:

| Variable          | Where                | What                                                                                              |
| ----------------- | -------------------- | ------------------------------------------------------------------------------------------------- |
| `SITE_URL`        | frontend build       | Public origin of the wiki site; enables sitemap + canonical URLs (unset: build works, skips both) |
| `PUBLIC_API_BASE` | frontend build       | Origin of the HTTP API if it is not same-origin with the site                                     |
| `WIKI_URL`        | bot (runtime)        | Wiki link used in bot DMs (default: this GitHub repo)                                             |
| `DATABASE_URL`    | API/bot (runtime)    | SQLite URL (default: `sqlite://wiki.db?mode=rwc`)                                                 |
| `TELOXIDE_TOKEN`  | bot (runtime)        | Telegram bot token; unset runs the HTTP API only                                                  |

## Contributing

- `main` is protected — **no direct pushes**.
- Every change lands through a **squash-merged pull request**.
- CI (lint + format + tests) must pass before a PR can merge:
  - Backend: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`
  - Frontend: `prettier --check`, `astro check`, `astro build`

Run `cargo fmt` and `npm run format` (in `frontend/short-shepherd`) before pushing to keep CI green.
