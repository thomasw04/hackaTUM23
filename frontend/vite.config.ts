import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      '/craft*': {
        target: 'http://backend:8000',
        changeOrigin: true,
      },
      '/zipcode*': {
        target: 'http://backend:8000',
        changeOrigin: true,
      },
    },
  },
})
