/* eslint-disable global-require */
/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: ['class', '[arco-theme="dark"]'],
	content: ['./index.html', './src/**/*.{vue,js,ts,jsx,tsx}'],
	theme: {
		extend: {},
	},
	plugins: [],
};
