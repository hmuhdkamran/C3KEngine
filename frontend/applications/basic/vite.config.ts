import { fileURLToPath, URL } from 'node:url'
import { resolve } from "path";

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import Layouts from 'vite-plugin-vue-layouts'
import Pages from 'vite-plugin-pages'
import VueI18n from '@intlify/unplugin-vue-i18n/vite'
import dotenv from 'dotenv'

const envFile = '.env'
dotenv.config({ path: envFile })

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    vueDevTools(),
    Layouts({
      layoutsDirs: 'src/components/layouts/',
      pagesDirs: 'src/views',
      defaultLayout: 'default',
    }),
    Pages({
      extensions: ['vue'],
      importMode: 'async',
      syncIndex: false,
      dirs: ['./src/views'],
    }),
    VueI18n({
      runtimeOnly: true,
      compositionOnly: true,
      fullInstall: true,
      include: [resolve(__dirname, "locales/**")],
    }),
  ],
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
