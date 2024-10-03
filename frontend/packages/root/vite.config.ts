import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import path from 'path';
import dotenv from 'dotenv'

const envFile = '.env'

dotenv.config({ path: envFile })

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    host: process.env.LOCAL_PATH,
    port: Number(process.env.LOCAL_PORT),
    open: false,
    watch: {
      usePolling: true,
      disableGlobbing: false,
    },
  },
  define: { 'process.env': {} },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src')
    }
  },
  build: {
    chunkSizeWarningLimit: 5000,
    outDir: '../production/root',
    emptyOutDir: true,
    manifest: true,
    target: 'esnext',
  },
})
