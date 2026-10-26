// The changelog entries (one per pull request merged into main) plus the
// one-line summary of the latest change that becomes the /changelog/ page
// description, and so the link preview Discord and Telegram show for it.
import items from "../data/changelog.json";

export type ChangelogEntry = {
  date: string;
  /** The pull request that landed the change; null for the initial commit. */
  pr: number | null;
  title: string;
  summary: string;
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

/** "Latest change, 2026-10-26: <title>. <summary>" */
export function describeLatest(): string {
  const summary = /[.!?]$/.test(latest.summary)
    ? latest.summary
    : `${latest.summary}.`;
  return `Latest change, ${latest.date}: ${latest.title}. ${summary}`;
}
