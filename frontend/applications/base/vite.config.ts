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
  plugins: [vue(), vueJsx(), vueDevTools()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  server: {
    host: process.env.VITE_APP_MICRO_SERVER,
    port: Number(process.env.VITE_APP_MICRO_PORT),
    open: false,
    cors: {
      origin: process.env.VITE_APP_CROSS_ORIGIN
    },
    watch: {
      usePolling: false,
      disableGlobbing: false,
    },
  },
  define: { 'process.env': {} },
  build: {
    rollupOptions: {
      input: {
        index: './index.html',
        'root-config': './src/main.ts',
      },
      output: {
        format: 'system',
        entryFileNames: '[name].js',
        assetFileNames: 'assets/[name][ext]',
        globals: {
          'single-spa': 'singleSpa',
          'single-spa-layout': 'singleSpaLayout',
        },
      },
      preserveEntrySignatures: 'strict',
      external: ['single-spa', 'single-spa-vue', 'single-spa-layout'],
    },
  },
})
