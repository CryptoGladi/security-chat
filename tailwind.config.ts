import type { Config } from 'tailwindcss'

export default {
  content: [ "./src/**/*.{js,jsx,ts,tsx}" ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
  daisyui: {
		themes: ['dracula'],
		logs: process.env.NODE_ENV === 'production' ? false : true
	}
} satisfies Config

