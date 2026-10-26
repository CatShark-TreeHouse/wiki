// The changelog entries (one per pull request merged into main) plus the
// derived bits the list, its dialog and the per-entry page all need: the
// slug that makes a permalink ("/changelog/26/") is computed in exactly one
// place, and the page descriptions double as link previews in Discord and
// Telegram.
import items from "../data/changelog.json";
import { entrySlug } from "./entries";

export type ChangelogEntry = {
  date: string;
  /** The pull request that landed the change; null for the initial commit. */
  pr: number | null;
  title: string;
  summary: string;
};

export type ChangeView = ChangelogEntry & {
  slug: string;
  href: string;
  prHref: string | null;
  /** True for the newest entry only. */
  latest: boolean;
  /** The per-entry page description: fixed, so an old link keeps its preview. */
  description: string;
  ogTitle: string;
};

const REPO = "https://github.com/CatShark-TreeHouse/wiki";

/** Newest first, whatever order the file is in. */
export const entries = (items as ChangelogEntry[])
  .slice()
  .sort((a, b) => b.date.localeCompare(a.date) || (b.pr ?? 0) - (a.pr ?? 0));

export const latest = entries[0];

export function prHref(pr: number): string {
  return `${REPO}/pull/${pr}`;
}

/** The PR number when there is one (stable even if the title is reworded). */
export function changeSlug(entry: ChangelogEntry): string {
  return entry.pr != null
    ? String(entry.pr)
    : entrySlug(`${entry.date} ${entry.title}`);
}

function endSentence(text: string): string {
  return /[.!?]$/.test(text) ? text : `${text}.`;
}

export function toView(
  entry: ChangelogEntry,
  base: string,
  isLatest: boolean,
): ChangeView {
  const slug = changeSlug(entry);
  return {
    ...entry,
    slug,
    href: `${base}/changelog/${slug}/`,
    prHref: entry.pr != null ? prHref(entry.pr) : null,
    latest: isLatest,
    description: `${entry.date}: ${endSentence(entry.summary)}`,
    ogTitle: `Changelog: ${entry.title}`,
  };
}

/** Every entry as a view, newest first, for the list and the static routes. */
export function changeViews(base: string): ChangeView[] {
  return entries.map((entry, i) => toView(entry, base, i === 0));
}

/** "Latest change, 2026-10-26: <title>. <summary>" (the list page's preview). */
export function describeLatest(): string {
  return `Latest change, ${latest.date}: ${latest.title}. ${endSentence(latest.summary)}`;
}
