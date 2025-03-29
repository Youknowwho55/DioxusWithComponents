/**
 * @format
 * @type {import('tailwindcss').Config}
 */

module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [],
  "tailwindCSS.experimental.configFile": "/learning/assets/tailwind.css",
};
