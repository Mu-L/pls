const { path } = require("@vuepress/utils");
const packageInfo = require("../../package.json");

const features = {
  text: "Features",
  children: [
    "/features/colors",
    "/features/icons",
    "/features/suffixes",
    "/features/details",
    "/features/sorting",
    "/features/importance",
    "/features/collapse",
  ],
};

module.exports = {
  lang: "en-GB",
  title: "pls",
  description: packageInfo.description,
  base: "/pls/",

  head: [["link", { rel: "icon", href: "/favicon.png" }]],

  theme: "@vuepress/theme-default",
  themeConfig: {
    repo: packageInfo.repository.replace("github:", ""),
    docsBranch: "docs",
    docsDir: "docs",

    navbar: [
      { text: "Get started", link: "/get_started" },
      features,
      { text: "PyPI", link: "https://pypi.org/project/pls/" },
    ],

    sidebar: {
      "/features/": [features],
    },
  },

  alias: {
    "@theme/HomeHero.vue": path.resolve(__dirname, "./components/HomeHero.vue"),
  },

  plugins: [
    [
      "@vuepress/plugin-docsearch",
      {
        apiKey: "aab9e7596d3aa3ef1a9834543eadbf60",
        indexName: "pls",
        appId: "V3X44L2GDB",
        placeholder: "Search...",
      },
    ],
  ],
};
