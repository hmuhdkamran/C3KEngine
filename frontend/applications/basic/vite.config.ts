import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import dotenv from 'dotenv'

const envFile = '.env'
dotenv.config({ path: envFile })

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
  server: {
    host: process.env.VITE_APP_MICRO_SERVER,
    port: Number(process.env.VITE_APP_MICRO_PORT),
    open: false,
    watch: {
      usePolling: false,
      disableGlobbing: false,
    },
  },
  define: { 'process.env': {} },
  build: {
    cssCodeSplit: false,
    rollupOptions: {
      input: ['src/main.ts', 'assets/main.css'],
      preserveEntrySignatures: 'strict',
      output: {
        entryFileNames: '[name].js',
        chunkFileNames: '[name].[hash].js',
        assetFileNames: '[name].[ext]',
      },
    },
  },
})
