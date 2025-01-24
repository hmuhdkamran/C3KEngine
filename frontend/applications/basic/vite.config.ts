import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import Layouts from 'vite-plugin-vue-layouts'
import Pages from 'vite-plugin-pages'
import dotenv from 'dotenv'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'
import { PlusProComponentsResolver } from '@plus-pro-components/resolver'

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
    }),
    Pages({
      dirs: ['./src/views'],
    }),
    AutoImport({
      resolvers: [ElementPlusResolver()]
    }),
    Components({
      resolvers: [ElementPlusResolver(), PlusProComponentsResolver()]
    })
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
