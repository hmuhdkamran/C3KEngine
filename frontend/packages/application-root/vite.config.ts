import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import Pages from 'vite-plugin-pages'
import Layouts from 'vite-plugin-vue-layouts'
import dotenv from 'dotenv'

const envFile = '.env'

dotenv.config({ path: envFile })

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    Pages({
      dirs: ['./src/pages'],
    }),
    Layouts({
      layoutsDirs: './src/layouts/',
    }),
  ],
  server: {
    host: process.env.LOCAL_PATH,
    port: Number(process.env.LOCAL_PORT),
    open: false,
    watch: {
      usePolling: false,
      disableGlobbing: false,
    },
  },
  define: { 'process.env': {} },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  build: {
    chunkSizeWarningLimit: 5000,
    outDir: '../production/root',
    emptyOutDir: true,
    manifest: true,
    target: 'esnext',
  },
})
