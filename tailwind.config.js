export default {
	content: ['./src/**/*.{html,js,svelte,ts}', './index.html', './src/app.html'],
	theme: {
		extend: {}
	},
	plugins: [require('daisyui')],

	daisyui: {
		themes: ['dracula'],
		logs: process.env.NODE_ENV === 'production' ? false : true
	}
};
