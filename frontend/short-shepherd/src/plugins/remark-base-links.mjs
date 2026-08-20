// Prefix root-relative links ("/rules/network-rules/") with Astro's base path
// so the same content works at "/" and at "/wiki/" (GitHub Pages project
// site). Anchors, protocol URLs and already-prefixed links are left alone.
export function remarkBaseLinks() {
  const base = (process.env.SITE_BASE || "/").replace(/\/?$/, "/");
  if (base === "/") return () => {};
  const prefix = base.slice(0, -1); // "/wiki"

  function fix(url) {
    if (typeof url !== "string") return url;
    if (!url.startsWith("/") || url.startsWith("//")) return url;
    if (url === prefix || url.startsWith(prefix + "/")) return url;
    return prefix + url;
  }

  function walk(node) {
    if (!node || typeof node !== "object") return;
    if ((node.type === "link" || node.type === "definition") && node.url) {
      node.url = fix(node.url);
    }
    // MDX JSX attributes such as <LinkCard href="/…" />
    if (
      (node.type === "mdxJsxFlowElement" ||
        node.type === "mdxJsxTextElement") &&
      Array.isArray(node.attributes)
    ) {
      for (const attr of node.attributes) {
        if (
          attr.type === "mdxJsxAttribute" &&
          (attr.name === "href" || attr.name === "link") &&
          typeof attr.value === "string"
        ) {
          attr.value = fix(attr.value);
        }
      }
    }
    // Raw HTML in markdown (the rules index)
    if (node.type === "html" && typeof node.value === "string") {
      node.value = node.value.replace(
        /href="(\/[^"/][^"]*)"/g,
        (m, u) => `href="${fix(u)}"`,
      );
    }
    if (Array.isArray(node.children)) node.children.forEach(walk);
  }

  return (tree) => walk(tree);
}
