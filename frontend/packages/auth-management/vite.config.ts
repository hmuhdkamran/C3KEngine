import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'

import Pages from 'vite-plugin-pages'
import Layouts from 'vite-plugin-vue-layouts'

import dotenv from 'dotenv'
import qiankun from 'vite-plugin-qiankun';

const envFile = '.env'
const useDevMode = true 

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
    qiankun('c3k-api-auth', { useDevMode })
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
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
  build: {
    chunkSizeWarningLimit: 5000,
    outDir: '../production/auth-management',
    emptyOutDir: true,
    manifest: true,
    target: 'esnext',
  },
})
