// Turn "§ 4.1" references in data strings into links to the matching clause
// on the Network Rules page. The anchor map is built from the rules source
// at build time, so it cannot drift from the headings.
import rulesSource from "../content/docs/rules/network-rules.md?raw";

const slugify = (s: string) =>
  s
    .toLowerCase()
    .replace(/<[^>]+>/g, "")
    .replace(/[^a-z0-9\s-]/g, "")
    .trim()
    .replace(/\s+/g, "-");

/** A rule ("19") or one of its clauses ("19.3"), as the headings name it. */
export type RuleRef = {
  num: string;
  title: string;
  /** The heading's id on the Network Rules page. */
  anchor: string;
  /** The rule a clause belongs to; null for a rule. */
  parent: string | null;
};

/** Every rule and clause, in page order. */
export const ruleRefs: RuleRef[] = [];
const anchors = new Map<string, string>();
for (const m of rulesSource.matchAll(
  /^#{3,4} <span class="cs-num">([\d.]+)<\/span>\s*(.+)$/gm,
)) {
  const num = m[1];
  const title = m[2].replace(/<[^>]+>/g, "").trim();
  const anchor = slugify(`${num} ${title}`);
  anchors.set(num, anchor);
  ruleRefs.push({
    num,
    title,
    anchor,
    parent: num.includes(".") ? num.split(".")[0] : null,
  });
}

/** Where the content under a heading starts, in the rendered rules page. */
function locate(html: string, anchor: string) {
  const id = html.indexOf(`id="${anchor}"`);
  if (id < 0) throw new Error(`Rule heading #${anchor} not found`);
  const wrapper = html.lastIndexOf('<div class="sl-heading-wrapper', id);
  const heading = html.lastIndexOf("<h", id);
  // Starlight wraps a heading and its anchor link in a div.
  const wrapped = wrapper >= 0 && !html.slice(wrapper, heading).includes("</");
  const closer = wrapped ? "</div>" : "</h";
  const close = html.indexOf(closer, id);
  const bodyStart = wrapped
    ? close + closer.length
    : html.indexOf(">", close) + 1;
  return { bodyStart };
}

/** The rendered content under a rule or clause heading, up to the next heading of its level or above. */
export function ruleSection(html: string, ref: RuleRef): string {
  const { bodyStart } = locate(html, ref.anchor);
  const levels = ref.parent ? "2-4" : "2-3";
  const next = new RegExp(
    `<div class="sl-heading-wrapper level-h[${levels}]|<h[${levels}][ >]`,
  );
  const rest = html.slice(bodyStart);
  const end = rest.search(next);
  return (end < 0 ? rest : rest.slice(0, end)).trim();
}

/** The text of a piece of rendered HTML, for a description. */
export function plainText(html: string): string {
  return html
    .replace(/<[^>]+>/g, " ")
    .replace(/&quot;/g, '"')
    .replace(/&#(?:39|x27);/g, "'")
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&amp;/g, "&")
    .replace(/\s+/g, " ")
    .replace(/ ([.,;:)])/g, "$1")
    .trim();
}

export function rulesHref(base: string, num: string): string | null {
  const slug = anchors.get(num);
  return slug ? `${base}/rules/network-rules/#${slug}` : null;
}

/** Escape text and wrap every "§ n" / "§ n.m" in a link when known. */
export function linkRules(base: string, text: string): string {
  const esc = text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
  return esc.replace(/§ ?(\d+(?:\.\d+)?)/g, (whole, num) => {
    const href = rulesHref(base, num);
    return href ? `<a href="${href}">§ ${num}</a>` : whole;
  });
}
