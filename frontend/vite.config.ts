import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      // Proxy all routes starting with /craft to localhost:3000
      "/craft": {
        target: "http://127.0.0.1:3000",
        secure: false,
        changeOrigin: true,
      },
      "/zipcode": {
        target: "http://127.0.0.1:3000",
        secure: false,
        changeOrigin: true,
      },
    },
  },
});
