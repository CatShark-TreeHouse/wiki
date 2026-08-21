// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import { remarkBaseLinks } from "./src/plugins/remark-base-links.mjs";

// https://astro.build/config
export default defineConfig({
  // The public origin of the deployed site (e.g. https://wiki.example.net).
  // Set SITE_URL at build time once the domain exists; it enables the
  // sitemap and canonical URLs. Unset, the build works and just skips both.
  site: process.env.SITE_URL || undefined,
  // Set when the site is served from a sub-path (GitHub Pages project site:
  // https://<org>.github.io/wiki/ needs base "/wiki").
  base: process.env.SITE_BASE || undefined,
  markdown: { remarkPlugins: [remarkBaseLinks] },
  integrations: [
    starlight({
      title: "CatShark TreeHouse",
      description:
        "The wiki of the CatShark TreeHouse network: rules, joining, staff, and the live controlled-content lists.",
      logo: { src: "./src/assets/flag.png", alt: "CatShark TreeHouse flag" },
      components: { PageTitle: "./src/components/PageTitle.astro" },
      favicon: "/favicon.png",
      customCss: [
        "@fontsource-variable/inter/index.css",
        "@fontsource-variable/bricolage-grotesque/index.css",
        "./src/styles/custom.css",
      ],
      editLink: {
        baseUrl:
          "https://github.com/CatShark-TreeHouse/wiki/edit/main/frontend/short-shepherd/",
      },
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/CatShark-TreeHouse/wiki",
        },
      ],
      sidebar: [
        {
          label: "Start Here",
          items: [
            { label: "Welcome", slug: "start-here/welcome" },
            { label: "How to Join", slug: "start-here/joining" },
          ],
        },
        {
          label: "Rules",
          items: [
            { label: "Network Rules", slug: "rules/network-rules" },
            { label: "Controlled Content", slug: "rules/controlled-content" },
            { label: "Banned & Controlled", slug: "controlled" },
          ],
        },
        {
          label: "Moderation",
          items: [
            { label: "Moderation Strategy", slug: "moderation/strategy" },
            { label: "Bans", slug: "moderation/bans" },
            {
              label: "Bewares",
              items: [
                { label: "Overview", slug: "moderation/bewares" },
                { label: "Patel", slug: "moderation/bewares/patel" },
              ],
            },
            {
              label: "Incidents",
              items: [
                { label: "Overview", slug: "incidents" },
                { label: "260819: Latte v. Reggie", slug: "incidents/260819" },
              ],
            },
          ],
        },
        {
          label: "Community",
          items: [
            { label: "Roles & Teams", slug: "community/roles-and-teams" },
            { label: "Staff", slug: "community/staff" },
            { label: "FAQ", slug: "community/faq" },
          ],
        },
      ],
    }),
  ],
});
