import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vite'
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import Pages from 'vite-plugin-pages'
import Layouts from 'vite-plugin-vue-layouts'
import DefineOptions from 'unplugin-vue-define-options/vite'
import VueI18nPlugin from '@intlify/unplugin-vue-i18n/vite'
import vue from '@vitejs/plugin-vue'
import dotenv from 'dotenv'
import qiankun from 'vite-plugin-qiankun';

const envFile = '.env'
const useDevMode = true 

dotenv.config({ path: envFile })

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    AutoImport({
      imports: ['vue', 'vue-router', '@vueuse/core', 'vue-i18n', 'pinia'],
      vueTemplate: true,
    }),
    Pages({
      dirs: ['./src/pages'],
    }),
    Components({
      dirs: ['src/components'],
      dts: true,
    }),
    Layouts({
      layoutsDirs: './src/layouts/',
    }),
    VueI18nPlugin({
      runtimeOnly: true,
      compositionOnly: true,
      include: [
        fileURLToPath(new URL('./src/plugins/i18n/locales/**', import.meta.url)),
      ],
    }),
    DefineOptions(),
    qiankun('c3k-user-management', { useDevMode })
  ],
  server: {
    host: process.env.LOCAL_PATH,
    port: Number(process.env.LOCAL_PORT),
    open: false,
    watch: {
      usePolling: false,
      disableGlobbing: false,
    },
    headers: {
      'Access-Control-Allow-Origin': '*'
    }
  },
  define: { 'process.env': {} },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
    },
  },
  build: {
    chunkSizeWarningLimit: 5000,
    outDir: '../production/user-management',
    emptyOutDir: true,
    manifest: true,
    target: 'esnext',
  },
})
