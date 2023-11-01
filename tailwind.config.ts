import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{js,jsx,ts,tsx}'],
	theme: {
		extend: {}
	},
	plugins: [require('daisyui'), require('tailwind-scrollbar')],
	daisyui: {
		themes: ['dracula'],
		logs: false
	}
} satisfies Config;
