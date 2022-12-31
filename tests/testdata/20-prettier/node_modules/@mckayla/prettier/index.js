module.exports = {
	printWidth: 90,
	quoteProps: "consistent",
	tabWidth: 4, // Necessary for prettier to align things, even though we use tabs
	trailingComma: "all",
	useTabs: true,
	overrides: [
		{
			files: ["./**/*.yml", "./**/*.yaml"],
			options: { tabWidth: 2 },
		},
	],
};
