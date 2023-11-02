/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,svelte,js,ts}'],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["light", "dracula"],
    darkTheme: "dracula",
  },
  plugins: [require("@tailwindcss/typography"), require(("daisyui"))],
}   