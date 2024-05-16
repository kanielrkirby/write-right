/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,jsx,ts,tsx,rs}"],
  theme: {
    extend: {
      fontFamily: {
        'mono': ['Cutive Mono', 'monospace'],
      }
    },
  },
  plugins: [],
}
