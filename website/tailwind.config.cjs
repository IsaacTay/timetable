/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
  ],
  theme: {
    extend: {},
  },

  plugins: [
    require('@tailwindcss/typography'),
    require("daisyui"),
  ],
}
