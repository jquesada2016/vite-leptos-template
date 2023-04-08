/** @type {import('tailwindcss').Config} */
export default {
  content: ["index.html", "src/**/*.ts", "wasm/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
};
