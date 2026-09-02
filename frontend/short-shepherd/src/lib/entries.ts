// The shape of a controlled-content entry plus the derived bits the list, the
// modal and the per-entry profile page all need. Kept here so the slug that
// makes a permalink ("/controlled/lugiem/") is computed in exactly one place.
import items from "../data/controlled-content.json";

export type ControlledEntry = {
  alias: string;
  type: string;
  status: string;
  reason: string | null;
  added: string;
};

export type EntryView = ControlledEntry & {
  slug: string;
  href: string;
  typeLabel: string;
  statusLabel: string;
  description: string;
};

const TYPE_LABELS: Record<string, string> = {
  artist: "Artist",
  character: "Character",
  tag: "Kink",
};

const TYPE_NOUNS: Record<string, string> = {
  artist: "artist",
  character: "character",
  tag: "kink",
};

export function entrySlug(alias: string): string {
  return alias
    .toLowerCase()
    .normalize("NFKD")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
}

/** The one-sentence summary used as the page description, so a link shared in
 *  Discord or Telegram already says who this is and why they are listed. */
function describe(entry: ControlledEntry): string {
  const noun = TYPE_NOUNS[entry.type] ?? entry.type;
  const head = `${entry.alias} is a ${entry.status} ${noun} on the CatShark TreeHouse network.`;
  if (!entry.reason) return head;
  const reason = /[.!?]$/.test(entry.reason)
    ? entry.reason
    : `${entry.reason}.`;
  return `${head} Reason: ${reason}`;
}

export function toView(entry: ControlledEntry, base: string): EntryView {
  const slug = entrySlug(entry.alias);
  return {
    ...entry,
    slug,
    href: `${base}/controlled/${slug}/`,
    typeLabel: TYPE_LABELS[entry.type] ?? entry.type,
    statusLabel: entry.status.charAt(0).toUpperCase() + entry.status.slice(1),
    description: describe(entry),
  };
}

export const entries = items as ControlledEntry[];

/** Every entry as a view, for the list page and the static profile routes. */
export function entryViews(base: string): EntryView[] {
  return entries.map((entry) => toView(entry, base));
}
