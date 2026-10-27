// The changelog: the network's major events, each pointing at the pull
// request that landed it, grouped by day. A day is the unit that gets a page (/changelog/2026-10-26/), a dialog
// and a shareable link, so its slug, permalink, heading and the description
// that doubles as its link preview are all computed here, in one place.
import items from "../data/changelog.json";

export type Change = {
  date: string;
  /** The pull request that landed the change; null when there was none. */
  pr: number | null;
  title: string;
  summary: string;
};

export type Day = {
  /** ISO date; also the slug. */
  date: string;
  /** Newest pull request first. */
  changes: Change[];
};

export type DayView = Day & {
  slug: string;
  href: string;
  /** "October 26, 2026" */
  heading: string;
  /** "3 changes" */
  count: string;
  /** True for the newest day only. */
  latest: boolean;
  /** The day page's description: fixed, so an old link keeps its preview. */
  description: string;
  ogTitle: string;
};

const REPO = "https://github.com/CatShark-TreeHouse/wiki";

/** Every change, newest first, whatever order the file is in. */
export const changes = (items as Change[])
  .slice()
  .sort((a, b) => b.date.localeCompare(a.date) || (b.pr ?? 0) - (a.pr ?? 0));

/** Days, newest first, each holding its changes newest first. */
export const days: Day[] = [];
for (const change of changes) {
  const last = days[days.length - 1];
  if (last && last.date === change.date) last.changes.push(change);
  else days.push({ date: change.date, changes: [change] });
}

export const latest = days[0];

export function prHref(pr: number): string {
  return `${REPO}/pull/${pr}`;
}

export function headingFor(date: string): string {
  return new Date(`${date}T00:00:00Z`).toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
    timeZone: "UTC",
  });
}

export function countFor(day: Day): string {
  const n = day.changes.length;
  return n === 1 ? "1 change" : `${n} changes`;
}

/** "3 changes: first title; second title; third title." */
export function summarize(day: Day): string {
  const titles = day.changes.map((c) => c.title).join("; ");
  return `${countFor(day)}: ${titles}.`;
}

export function toView(day: Day, base: string, isLatest: boolean): DayView {
  const heading = headingFor(day.date);
  return {
    ...day,
    slug: day.date,
    href: `${base}/changelog/${day.date}/`,
    heading,
    count: countFor(day),
    latest: isLatest,
    description: `Changelog for ${heading}. ${summarize(day)}`,
    ogTitle: `Changelog, ${heading}`,
  };
}

/** Every day as a view, newest first, for the list and the static routes. */
export function dayViews(base: string): DayView[] {
  return days.map((day, i) => toView(day, base, i === 0));
}

/** The list page's description (and link preview): the newest day. */
export function describeLatest(): string {
  return `Latest changes, ${headingFor(latest.date)}. ${summarize(latest)}`;
}
