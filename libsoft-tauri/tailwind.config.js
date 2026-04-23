/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      // ── Font ────────────────────────────────────────────────────────────────
      fontFamily: {
        mono: ['"JetBrains Mono"', '"JetBrainsMono Nerd Font"', "monospace"],
        sans: ['"JetBrains Mono"', '"JetBrainsMono Nerd Font"', "monospace"],
      },

      // ── Color palette ────────────────────────────────────────────────────────
      colors: {
        // Dark backgrounds
        surface: {
          950: "#0a0c12",   // deepest app bg
          900: "#10131d",   // panel bg
          800: "#161928",   // card / sidebar bg
          700: "#1e2235",   // elevated card
          600: "#252a3f",   // hover
          500: "#2e3450",   // border
        },
        // Cobalt blue – primary accent
        cobalt: {
          50:  "#e6f0ff",
          100: "#bdd5ff",
          200: "#94baff",
          300: "#6ba0ff",
          400: "#4285fe",
          500: "#2563eb",   // primary button
          600: "#1d4ed8",
          700: "#1e40af",
          800: "#1e3a8a",
          900: "#1e3070",
        },
        // Emerald green – success / returned status
        emerald: {
          50:  "#ecfdf5",
          100: "#d1fae5",
          200: "#a7f3d0",
          300: "#6ee7b7",
          400: "#34d399",
          500: "#10b981",   // success
          600: "#059669",
          700: "#047857",
          800: "#065f46",
          900: "#064e3b",
        },
        // Amber – overdue / warning
        amber: {
          400: "#fbbf24",
          500: "#f59e0b",
        },
        // Rose – error / danger
        rose: {
          400: "#fb7185",
          500: "#f43f5e",
        },
      },

      // ── Border radius ────────────────────────────────────────────────────────
      borderRadius: {
        xl:  "0.75rem",
        "2xl": "1rem",
        "3xl": "1.5rem",
      },

      // ── Box shadows ──────────────────────────────────────────────────────────
      boxShadow: {
        glow:    "0 0 20px rgba(37, 99, 235, 0.35)",
        "glow-sm": "0 0 10px rgba(37, 99, 235, 0.25)",
        panel:   "0 4px 24px rgba(0,0,0,0.45)",
        card:    "0 2px 12px rgba(0,0,0,0.35)",
      },

      // ── Transitions ──────────────────────────────────────────────────────────
      transitionDuration: {
        150: "150ms",
        250: "250ms",
      },
    },
  },
  plugins: [],
};
