/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,ts,jsx,tsx,mdx}",
    "../../packages/ui/src/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          DEFAULT: "#7ebab5",
          50: "#eef8f7",
          100: "#d5eeeb",
          200: "#adddd8",
          300: "#85c5be",
          400: "#7ebab5",
          500: "#5da8a0",
          600: "#4a8d86",
          700: "#3e736e",
          800: "#355d59",
          900: "#2e4f4c",
          950: "#172f2d",
        },
        background: "#090a0c",
        surface: {
          DEFAULT: "#111318",
          elevated: "#181b21",
          overlay: "#1e2128",
        },
      },
    },
  },
  plugins: [],
};
