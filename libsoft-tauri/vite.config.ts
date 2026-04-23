import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react()],
  // Prevent Vite from hiding Rust errors
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: {
      // Watch the Tauri source for hot-reload triggers
      ignored: ["**/src-tauri/**"],
    },
  },
}));
