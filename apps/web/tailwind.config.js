/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        background: "var(--bg-primary)",
        surface: "var(--bg-secondary)",
        border: "var(--border-color)",
        accent: {
          DEFAULT: "#3b82f6",
          hover: "#2563eb",
        },
        severity: {
          trace: "#6b7280",
          debug: "#3b82f6",
          info: "#10b981",
          notice: "#06b6d4",
          warning: "#f59e0b",
          error: "#ef4444",
          critical: "#dc2626",
          fatal: "#881337",
          unknown: "#9ca3af",
        },
      },
    },
  },
  plugins: [],
};
