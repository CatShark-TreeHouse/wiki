---
title: Controlled Content
description: "How the banned and controlled lists work: what the statuses mean, what counts, and how moderators maintain them."
---

Some artists, characters, and kinks are restricted in the network: either
**banned** outright or **controlled**, meaning allowed only behind a spoiler. This page explains the policy;
the actual entries live on the **[Banned & Controlled page](/controlled/)**.
Respecting the lists is [§ 13](/rules/network-rules/#13-respect-the-controlled-content-lists) of the Network Rules.

## The Two Statuses

| Status         | What it means                                                                                                                                                                                                                                       |
| :------------- | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Banned**     | Not allowed in the network, period ([§ 13.1](/rules/network-rules/#131-artists), [§ 13.2](/rules/network-rules/#132-characters), [§ 13.3](/rules/network-rules/#133-tags)). This **includes stickers** and, for characters, any art they appear in. |
| **Controlled** | Allowed, but it **must** be sent behind a spoiler and with a content warning ([§ 13.4](/rules/network-rules/#134-spoiler-controlled-content)).                                                                                                      |

## What Can Be Controlled

- **Artists**: a banned artist's work may not be posted at all, stickers
  included ([§ 13.1](/rules/network-rules/#131-artists)). You can assume any artist that draws cub or zoo content
  is banned, whether or not they are on the list yet; the list is not
  exhaustive and [§ 13](/rules/network-rules/#13-respect-the-controlled-content-lists) applies either way.
- **Characters**: a banned character covers stickers and any piece of art
  they appear in ([§ 13.2](/rules/network-rules/#132-characters)). Aliases of the same character are listed
  individually.
- **Kinks** (tags): banned kinks may not be posted at all ([§ 13.3](/rules/network-rules/#133-tags));
  controlled kinks must carry a content warning and be sent behind a spoiler
  ([§ 13.4](/rules/network-rules/#134-spoiler-controlled-content)).

## Enforcement

Posting banned content is removed on sight and counts as a breach of
[§ 13.1](/rules/network-rules/#131-artists), [§ 13.2](/rules/network-rules/#132-characters) or [§ 13.3](/rules/network-rules/#133-tags), depending on what it was. Posting controlled content without a spoiler is a breach of [§ 13.4](/rules/network-rules/#134-spoiler-controlled-content);
the usual first response is a request to delete and repost it spoilered.
Either one, repeated, falls under [§ 16](/rules/network-rules/#16-repeat-offenders).

## How the Lists Are Maintained

The lists are a file in the wiki's repository, changed only through reviewed
pull requests. Every change is a revision: it records who proposed it, who
approved it, and when it landed, and nothing is ever edited in place without
that trail. That is why each row on the
[Banned & Controlled page](/controlled/) carries an **Added** date, and why
the full history of any entry can be read back on
[GitHub](https://github.com/CatShark-TreeHouse/wiki/commits/main/frontend/short-shepherd/src/data/controlled-content.json).

Entries are added with a single-word alias (for example `NozomyArts`), a
type, a status, an optional reason, and the date they were added. Removing
an entry is also a revision, so the history shows when something stopped
being controlled and why.

:::note[Not sure about something?]
If you're unsure whether a piece of art, an artist, or a kink is okay to
post, ask one of the [staff](/community/staff/) first. That's always the
safe move, and the same advice the rules give for real-life NSFW
([§ 11.2](/rules/network-rules/#112-ask-before-posting)).
:::
