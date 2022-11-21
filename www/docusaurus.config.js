// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').Config} */
const config = {
	title: "Kirbo",
	tagline: "A secure and fast npm/yarn alternative",
	url: "https://kirbo.pink",
	baseUrl: "/",
	onBrokenLinks: "throw",
	onBrokenMarkdownLinks: "warn",
	favicon: "img/favicon.ico",

	// GitHub pages deployment config.
	// If you aren't using GitHub pages, you don't need these.
	organizationName: "aslilac",
	projectName: "kirbo",

	// Even if you don't use internalization, you can use this field to set useful
	// metadata like html lang. For example, if your site is Chinese, you may want
	// to replace "en" with "zh-Hans".
	i18n: {
		defaultLocale: "en",
		locales: ["en"],
	},

	presets: [
		[
			"classic",
			/** @type {import('@docusaurus/preset-classic').Options} */
			({
				docs: {
					sidebarPath: require.resolve("./sidebars.js"),
					editUrl: "https://github.com/aslilac/kirbo/tree/main/www/",
				},
				theme: {
					customCss: require.resolve("./src/css/custom.css"),
				},
			}),
		],
	],

	themeConfig:
		/** @type {import('@docusaurus/preset-classic').ThemeConfig} */
		({
			navbar: {
				logo: {
					alt: "Kirbo",
					src: "https://cdn.mckayla.cloud/-/9dd72ce86ff44066bd0f2a6b902373ad/Logo.webp",
				},
				items: [
					{
						type: "doc",
						docId: "intro",
						position: "right",
						label: "Tutorial",
					},
					{
						href: "https://github.com/facebook/docusaurus",
						label: "GitHub",
						position: "right",
					},
				],
			},
			footer: {
				style: "dark",
				links: [
					{
						title: "Docs",
						items: [
							{
								label: "Tutorial",
								to: "/docs/intro",
							},
						],
					},
					{
						title: "Contributing",
						items: [
							{
								label: "Code of Conduct",
								to: "/coc",
							},
							{
								label: "GitHub",
								href: "https://github.com/aslilac/kirbo",
							},
						],
					},
				],
				copyright: `© ${new Date().getFullYear()} &hearts; McKayla`,
			},
			prism: {
				theme: lightCodeTheme,
				darkTheme: darkCodeTheme,
			},
		}),
};

module.exports = config;
