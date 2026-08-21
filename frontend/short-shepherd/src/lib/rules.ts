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

const anchors = new Map<string, string>();
for (const m of rulesSource.matchAll(
  /^#{3,4} <span class="cs-num">([\d.]+)<\/span>\s*(.+)$/gm,
)) {
  const num = m[1];
  const title = m[2].replace(/<[^>]+>/g, "").trim();
  anchors.set(num, slugify(`${num} ${title}`));
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
