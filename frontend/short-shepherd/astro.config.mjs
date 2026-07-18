// @ts-check
import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";

// https://astro.build/config
export default defineConfig({
  integrations: [
    starlight({
      title: "CatShark TreeHouse",
      description:
        "The wiki of the CatShark TreeHouse network — rules, joining, staff, and the live controlled-content lists.",
      logo: { src: "./src/assets/catshark.svg" },
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
            { label: "Banned & Spoilered (live)", slug: "controlled" },
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
