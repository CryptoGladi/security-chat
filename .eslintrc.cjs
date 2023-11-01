module.exports = {
	env: {
		browser: true,
		es2021: true
	},
	extends: ['eslint:recommended', 'plugin:solid/typescript', 'plugin:tailwindcss/recommended'],
	parser: '@typescript-eslint/parser',
	plugins: ['solid'],
	overrides: [
		{
			env: {
				node: true
			},
			files: ['.eslintrc.{js,cjs}'],
			parserOptions: {
				sourceType: 'script'
			}
		}
	],
	parserOptions: {
		ecmaFeatures: {
			jsx: true
		}
	},
	rules: {
		'solid/reactivity': 'warn',
		'solid/no-destructure': 'warn',
		'solid/jsx-no-undef': 'error'
	}
};
